// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::{read_manifest, FileMetadata, FileType, Manifest, CHECKPOINT_MESSAGE_FILE_MAGIC};
use anyhow::{anyhow, Context, Result};
use bytes::buf::Reader;
use bytes::{Buf, Bytes};
use futures::{StreamExt, TryStreamExt};
use ika_config::node::ArchiveReaderConfig;
use ika_types::message::DwalletCheckpointMessageKind;
use ika_types::messages_checkpoint::{
    CertifiedDWalletCheckpointMessage, CheckpointSequenceNumber, VerifiedCheckpointMessage,
    VerifiedDWalletCheckpointMessage,
};
use ika_types::storage::WriteStore;
use prometheus::{register_int_counter_vec_with_registry, IntCounterVec, Registry};
use rand::seq::SliceRandom;
use std::borrow::Borrow;
use std::future;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use sui_storage::object_store::http::HttpDownloaderBuilder;
use sui_storage::object_store::util::get;
use sui_storage::object_store::ObjectStoreGetExt;
use sui_storage::{compute_sha3_checksum_for_bytes, make_iterator, verify_checkpoint};
use tokio::sync::oneshot::Sender;
use tokio::sync::{oneshot, Mutex};
use tracing::info;

#[derive(Debug)]
pub struct ArchiveReaderMetrics {
    pub archive_actions_read: IntCounterVec,
    pub archive_checkpoints_read: IntCounterVec,
}

impl ArchiveReaderMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let this = Self {
            archive_actions_read: register_int_counter_vec_with_registry!(
                "archive_actions_read",
                "Number of actions read from archive",
                &["bucket"],
                registry
            )
            .unwrap(),
            archive_checkpoints_read: register_int_counter_vec_with_registry!(
                "archive_checkpoints_read",
                "Number of checkpoints read from archive",
                &["bucket"],
                registry
            )
            .unwrap(),
        };
        Arc::new(this)
    }
}

// ArchiveReaderBalancer selects archives for reading based on whether they can fulfill a checkpoint request
#[derive(Default, Clone)]
pub struct ArchiveReaderBalancer {
    readers: Vec<Arc<ArchiveReader>>,
}

impl ArchiveReaderBalancer {
    pub fn new(configs: Vec<ArchiveReaderConfig>, registry: &Registry) -> Result<Self> {
        let mut readers = vec![];
        let metrics = ArchiveReaderMetrics::new(registry);
        for config in configs.into_iter() {
            readers.push(Arc::new(ArchiveReader::new(config.clone(), &metrics)?));
        }
        Ok(ArchiveReaderBalancer { readers })
    }
    pub async fn get_archive_watermark(&self) -> Result<Option<u64>> {
        let mut checkpoints: Vec<Result<CheckpointSequenceNumber>> = vec![];
        for reader in self
            .readers
            .iter()
            .filter(|r| r.use_for_pruning_watermark())
        {
            let latest_checkpoint = reader.latest_available_checkpoint().await;
            info!(
                "Latest archived checkpoint in remote store: {:?} is: {:?}",
                reader.remote_store_identifier(),
                latest_checkpoint
            );
            checkpoints.push(latest_checkpoint)
        }
        let checkpoints: Result<Vec<CheckpointSequenceNumber>> = checkpoints.into_iter().collect();
        checkpoints.map(|vec| vec.into_iter().min())
    }
    pub async fn pick_one_random(
        &self,
        checkpoint_range: Range<CheckpointSequenceNumber>,
    ) -> Option<Arc<ArchiveReader>> {
        let mut archives_with_complete_range = vec![];
        for reader in self.readers.iter() {
            let latest_checkpoint = reader.latest_available_checkpoint().await.unwrap_or(0);
            if latest_checkpoint >= checkpoint_range.end {
                archives_with_complete_range.push(reader.clone());
            }
        }
        if !archives_with_complete_range.is_empty() {
            return Some(
                archives_with_complete_range
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
            );
        }
        let mut archives_with_partial_range = vec![];
        for reader in self.readers.iter() {
            let latest_checkpoint = reader.latest_available_checkpoint().await.unwrap_or(0);
            if latest_checkpoint >= checkpoint_range.start {
                archives_with_partial_range.push(reader.clone());
            }
        }
        if !archives_with_partial_range.is_empty() {
            return Some(
                archives_with_partial_range
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
            );
        }
        None
    }
}

#[derive(Clone)]
pub struct ArchiveReader {
    bucket: String,
    concurrency: usize,
    sender: Arc<Sender<()>>,
    manifest: Arc<Mutex<Manifest>>,
    use_for_pruning_watermark: bool,
    remote_object_store: Arc<dyn ObjectStoreGetExt>,
    archive_reader_metrics: Arc<ArchiveReaderMetrics>,
}

impl ArchiveReader {
    pub fn new(config: ArchiveReaderConfig, metrics: &Arc<ArchiveReaderMetrics>) -> Result<Self> {
        let bucket = config
            .remote_store_config
            .bucket
            .clone()
            .unwrap_or("unknown".to_string());
        let remote_object_store = if config.remote_store_config.no_sign_request {
            config.remote_store_config.make_http()?
        } else {
            config.remote_store_config.make().map(Arc::new)?
        };
        let (sender, recv) = oneshot::channel();
        let manifest = Arc::new(Mutex::new(Manifest::new(0, 0)));
        // Start a background tokio task to keep local manifest in sync with remote
        Self::spawn_manifest_sync_task(remote_object_store.clone(), manifest.clone(), recv);
        Ok(ArchiveReader {
            bucket,
            manifest,
            sender: Arc::new(sender),
            remote_object_store,
            use_for_pruning_watermark: config.use_for_pruning_watermark,
            concurrency: config.download_concurrency.get(),
            archive_reader_metrics: metrics.clone(),
        })
    }

    /// This function verifies that the files in archive cover the entire range of checkpoints from
    /// sequence number 0 until the latest available checkpoint with no missing checkpoint
    pub async fn verify_manifest(&self, manifest: Manifest) -> Result<Vec<FileMetadata>> {
        let files = manifest.files();
        if files.is_empty() {
            return Err(anyhow!("Unexpected empty archive store"));
        }

        let mut checkpoint_files: Vec<_> = files
            .clone()
            .into_iter()
            .filter(|f| f.file_type == FileType::CheckpointMessage)
            .collect();

        checkpoint_files.sort_by_key(|f| f.checkpoint_seq_range.start);

        assert!(checkpoint_files
            .windows(2)
            .all(|w| w[1].checkpoint_seq_range.start == w[0].checkpoint_seq_range.end));
        assert!(checkpoint_files
            .windows(2)
            .all(|w| w[1].checkpoint_seq_range.start == w[0].checkpoint_seq_range.end));

        assert_eq!(files.first().unwrap().checkpoint_seq_range.start, 0);

        Ok(files)
    }

    /// This function downloads checkpoint and content files and ensures their computed checksum matches
    /// the one in manifest
    pub async fn verify_file_consistency(&self, files: Vec<FileMetadata>) -> Result<()> {
        let remote_object_store = self.remote_object_store.clone();
        futures::stream::iter(files.iter())
            .enumerate()
            .map(|(_, checkpoint_metadata)| {
                let remote_object_store = remote_object_store.clone();
                async move {
                    let checkpoint_data =
                        get(&remote_object_store, &checkpoint_metadata.file_path()).await?;
                    Ok::<(Bytes, &FileMetadata), anyhow::Error>((
                        checkpoint_data,
                        checkpoint_metadata,
                    ))
                }
            })
            .boxed()
            .buffer_unordered(self.concurrency)
            .try_for_each(|(checkpoint_data, checkpoint_metadata)| {
                let checksums = compute_sha3_checksum_for_bytes(checkpoint_data);
                let result = checksums.and_then(|checkpoint_checksum| {
                    (checkpoint_checksum == checkpoint_metadata.sha3_digest)
                        .then_some(())
                        .ok_or(anyhow!(
                            "Checkpoint checksum doesn't match for file: {:?}",
                            checkpoint_metadata.file_path()
                        ))?;
                    Ok::<(), anyhow::Error>(())
                });
                futures::future::ready(result)
            })
            .await
    }

    /// Load checkpoints from archive into the input store `S` for the given checkpoint
    /// range. Summaries are downloaded out of order and inserted without verification
    pub async fn read_checkpoints_for_range_no_verify<S>(
        &self,
        store: S,
        checkpoint_range: Range<CheckpointSequenceNumber>,
        checkpoint_counter: Arc<AtomicU64>,
    ) -> Result<()>
    where
        S: WriteStore + Clone,
    {
        let (checkpoint_files, start_index, end_index) = self
            .get_checkpoint_files_for_range(checkpoint_range.clone())
            .await?;
        let remote_object_store = self.remote_object_store.clone();
        let stream = futures::stream::iter(checkpoint_files.iter())
            .enumerate()
            .filter(|(index, _s)| future::ready(*index >= start_index && *index < end_index))
            .map(|(_, checkpoint_metadata)| {
                let remote_object_store = remote_object_store.clone();
                async move {
                    let checkpoint_data =
                        get(&remote_object_store, &checkpoint_metadata.file_path()).await?;
                    Ok::<Bytes, anyhow::Error>(checkpoint_data)
                }
            })
            .boxed();
        stream
            .buffer_unordered(self.concurrency)
            .try_for_each(|checkpoint_data| {
                let result: Result<(), anyhow::Error> =
                    make_iterator::<CertifiedDWalletCheckpointMessage, Reader<Bytes>>(
                        CHECKPOINT_MESSAGE_FILE_MAGIC,
                        checkpoint_data.reader(),
                    )
                    .and_then(|checkpoint_iter| {
                        checkpoint_iter
                            .filter(|s| {
                                s.sequence_number >= checkpoint_range.start
                                    && s.sequence_number < checkpoint_range.end
                            })
                            .try_for_each(|checkpoint| {
                                Self::insert_certified_checkpoint(&store, checkpoint)?;
                                checkpoint_counter.fetch_add(1, Ordering::Relaxed);
                                Ok::<(), anyhow::Error>(())
                            })
                    });
                futures::future::ready(result)
            })
            .await
    }

    /// Load given list of checkpoints from archive into the input store `S`.
    /// Summaries are downloaded out of order and inserted without verification
    pub async fn read_checkpoints_for_list_no_verify<S>(
        &self,
        store: S,
        skiplist: Vec<CheckpointSequenceNumber>,
        checkpoint_counter: Arc<AtomicU64>,
    ) -> Result<()>
    where
        S: WriteStore + Clone,
    {
        let checkpoint_files = self.get_checkpoint_files_for_list(skiplist.clone()).await?;
        let remote_object_store = self.remote_object_store.clone();
        let stream = futures::stream::iter(checkpoint_files.iter())
            .map(|checkpoint_metadata| {
                let remote_object_store = remote_object_store.clone();
                async move {
                    let checkpoint_data =
                        get(&remote_object_store, &checkpoint_metadata.file_path()).await?;
                    Ok::<Bytes, anyhow::Error>(checkpoint_data)
                }
            })
            .boxed();

        stream
            .buffer_unordered(self.concurrency)
            .try_for_each(|checkpoint_data| {
                let result: Result<(), anyhow::Error> =
                    make_iterator::<CertifiedDWalletCheckpointMessage, Reader<Bytes>>(
                        CHECKPOINT_MESSAGE_FILE_MAGIC,
                        checkpoint_data.reader(),
                    )
                    .and_then(|checkpoint_iter| {
                        checkpoint_iter
                            .filter(|s| skiplist.contains(&s.sequence_number))
                            .try_for_each(|checkpoint| {
                                Self::insert_certified_checkpoint(&store, checkpoint)?;
                                checkpoint_counter.fetch_add(1, Ordering::Relaxed);
                                Ok::<(), anyhow::Error>(())
                            })
                    });
                futures::future::ready(result)
            })
            .await
    }

    pub async fn get_checkpoints_for_list_no_verify(
        &self,
        cp_list: Vec<CheckpointSequenceNumber>,
    ) -> Result<Vec<CertifiedDWalletCheckpointMessage>> {
        let checkpoint_files = self.get_checkpoint_files_for_list(cp_list.clone()).await?;
        let remote_object_store = self.remote_object_store.clone();
        let stream = futures::stream::iter(checkpoint_files.iter())
            .map(|checkpoint_metadata| {
                let remote_object_store = remote_object_store.clone();
                async move {
                    let checkpoint_data =
                        get(&remote_object_store, &checkpoint_metadata.file_path()).await?;
                    Ok::<Bytes, anyhow::Error>(checkpoint_data)
                }
            })
            .boxed();

        stream
            .buffer_unordered(self.concurrency)
            .try_fold(Vec::new(), |mut acc, checkpoint_data| async move {
                let checkpoint_result: Result<
                    Vec<CertifiedDWalletCheckpointMessage>,
                    anyhow::Error,
                > = make_iterator::<CertifiedDWalletCheckpointMessage, Reader<Bytes>>(
                    CHECKPOINT_MESSAGE_FILE_MAGIC,
                    checkpoint_data.reader(),
                )
                .map(|checkpoint_iter| checkpoint_iter.collect::<Vec<_>>());

                match checkpoint_result {
                    Ok(checkpoints) => {
                        acc.extend(checkpoints);
                        Ok(acc)
                    }
                    Err(e) => Err(e),
                }
            })
            .await
    }

    /// Load checkpoints from archive into the input store `S` for the given
    /// checkpoint range. If latest available checkpoint in archive is older than the start of the
    /// input range then this call fails with an error otherwise we load as many checkpoints as
    /// possible until the end of the provided checkpoint range (no verification for checkpoint committee).
    pub async fn read<S>(
        &self,
        store: S,
        checkpoint_range: Range<CheckpointSequenceNumber>,
        action_counter: Arc<AtomicU64>,
        checkpoint_counter: Arc<AtomicU64>,
    ) -> Result<()>
    where
        S: WriteStore + Clone,
    {
        let manifest = self.manifest.lock().await.clone();

        let latest_available_checkpoint = manifest
            .next_checkpoint_seq_num()
            .checked_sub(1)
            .context("Checkpoint seq num underflow")?;

        if checkpoint_range.start > latest_available_checkpoint {
            return Err(anyhow!(
                "Latest available checkpoint is: {}",
                latest_available_checkpoint
            ));
        }

        let files: Vec<FileMetadata> = self.verify_manifest(manifest).await?;

        let start_index = match files
            .binary_search_by_key(&checkpoint_range.start, |c| c.checkpoint_seq_range.start)
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        let end_index = match files
            .binary_search_by_key(&checkpoint_range.end, |c| c.checkpoint_seq_range.start)
        {
            Ok(index) => index,
            Err(index) => index,
        };

        let remote_object_store = self.remote_object_store.clone();
        futures::stream::iter(files.iter())
            .enumerate()
            .filter(|(index, _c)| future::ready(*index >= start_index && *index < end_index))
            .map(|(_, checkpoint_metadata)| {
                let remote_object_store = remote_object_store.clone();
                async move {
                    let checkpoint_data =
                        get(&remote_object_store, &checkpoint_metadata.file_path()).await?;
                    Ok::<Bytes, anyhow::Error>(checkpoint_data)
                }
            })
            .boxed()
            .buffered(self.concurrency)
            .try_for_each(|checkpoint_data| {
                let result: Result<(), anyhow::Error> =
                    make_iterator::<CertifiedDWalletCheckpointMessage, Reader<Bytes>>(
                        CHECKPOINT_MESSAGE_FILE_MAGIC,
                        checkpoint_data.reader(),
                    )
                    .and_then(|checkpoint_iter| {
                        checkpoint_iter
                            .filter(|c| {
                                c.sequence_number >= checkpoint_range.start
                                    && c.sequence_number < checkpoint_range.end
                            })
                            .try_for_each(|checkpoint| {
                                let size = checkpoint.messages.len();
                                let verified_checkpoint =
                                    Self::get_or_insert_verified_checkpoint(&store, checkpoint)?;
                                // Update highest synced watermark
                                store
                                    .update_highest_synced_checkpoint(&verified_checkpoint)
                                    .map_err(|e| anyhow!("Failed to update watermark: {e}"))?;
                                action_counter.fetch_add(size as u64, Ordering::Relaxed);
                                self.archive_reader_metrics
                                    .archive_actions_read
                                    .with_label_values(&[&self.bucket])
                                    .inc_by(size as u64);
                                checkpoint_counter.fetch_add(1, Ordering::Relaxed);
                                self.archive_reader_metrics
                                    .archive_checkpoints_read
                                    .with_label_values(&[&self.bucket])
                                    .inc_by(1);
                                Ok::<(), anyhow::Error>(())
                            })
                    });
                futures::future::ready(result)
            })
            .await
    }

    /// Return latest available checkpoint in archive
    pub async fn latest_available_checkpoint(&self) -> Result<CheckpointSequenceNumber> {
        let manifest = self.manifest.lock().await.clone();
        manifest
            .next_checkpoint_seq_num()
            .checked_sub(1)
            .context("No checkpoint data in archive")
    }

    pub fn use_for_pruning_watermark(&self) -> bool {
        self.use_for_pruning_watermark
    }

    pub fn remote_store_identifier(&self) -> String {
        self.remote_object_store.to_string()
    }

    pub async fn sync_manifest_once(&self) -> Result<()> {
        Self::sync_manifest(self.remote_object_store.clone(), self.manifest.clone()).await?;
        Ok(())
    }

    pub async fn get_manifest(&self) -> Result<Manifest> {
        Ok(self.manifest.lock().await.clone())
    }

    async fn sync_manifest(
        remote_store: Arc<dyn ObjectStoreGetExt>,
        manifest: Arc<Mutex<Manifest>>,
    ) -> Result<()> {
        let new_manifest = read_manifest(remote_store.clone()).await?;
        let mut locked = manifest.lock().await;
        *locked = new_manifest;
        Ok(())
    }

    /// Insert checkpoint checkpoint without verifying it
    fn insert_certified_checkpoint<S>(
        store: &S,
        certified_checkpoint: CertifiedDWalletCheckpointMessage,
    ) -> Result<()>
    where
        S: WriteStore + Clone,
    {
        store
            .insert_checkpoint(
                VerifiedCheckpointMessage::new_unchecked(certified_checkpoint).borrow(),
            )
            .map_err(|e| anyhow!("Failed to insert checkpoint: {e}"))
    }

    /// Insert checkpoint checkpoint if it doesn't already exist (without verifying it)
    fn get_or_insert_verified_checkpoint<S>(
        store: &S,
        certified_checkpoint: CertifiedDWalletCheckpointMessage,
    ) -> Result<VerifiedDWalletCheckpointMessage>
    where
        S: WriteStore + Clone,
    {
        store
            .get_checkpoint_by_sequence_number(certified_checkpoint.sequence_number)
            .map_err(|e| anyhow!("Store op failed: {e}"))?
            .map(Ok::<VerifiedDWalletCheckpointMessage, anyhow::Error>)
            .unwrap_or_else(|| {
                let verified_checkpoint =
                    VerifiedCheckpointMessage::new_unchecked(certified_checkpoint);
                // Insert checkpoint message
                store
                    .insert_checkpoint(&verified_checkpoint)
                    .map_err(|e| anyhow!("Failed to insert checkpoint: {e}"))?;
                // Update highest verified checkpoint watermark
                store
                    .update_highest_verified_checkpoint(&verified_checkpoint)
                    .expect("store operation should not fail");
                Ok::<VerifiedDWalletCheckpointMessage, anyhow::Error>(verified_checkpoint)
            })
            .map_err(|e| anyhow!("Failed to get verified checkpoint: {:?}", e))
    }

    async fn get_checkpoint_files_for_range(
        &self,
        checkpoint_range: Range<CheckpointSequenceNumber>,
    ) -> Result<(Vec<FileMetadata>, usize, usize)> {
        let manifest = self.manifest.lock().await.clone();

        let latest_available_checkpoint = manifest
            .next_checkpoint_seq_num()
            .checked_sub(1)
            .context("Checkpoint seq num underflow")?;

        if checkpoint_range.start > latest_available_checkpoint {
            return Err(anyhow!(
                "Latest available checkpoint is: {}",
                latest_available_checkpoint
            ));
        }

        let checkpoint_files: Vec<FileMetadata> = self.verify_manifest(manifest).await?;

        let start_index = match checkpoint_files
            .binary_search_by_key(&checkpoint_range.start, |s| s.checkpoint_seq_range.start)
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        let end_index = match checkpoint_files
            .binary_search_by_key(&checkpoint_range.end, |s| s.checkpoint_seq_range.start)
        {
            Ok(index) => index,
            Err(index) => index,
        };

        Ok((checkpoint_files, start_index, end_index))
    }

    async fn get_checkpoint_files_for_list(
        &self,
        checkpoints: Vec<CheckpointSequenceNumber>,
    ) -> Result<Vec<FileMetadata>> {
        assert!(!checkpoints.is_empty());
        let manifest = self.manifest.lock().await.clone();
        let latest_available_checkpoint = manifest
            .next_checkpoint_seq_num()
            .checked_sub(1)
            .context("Checkpoint seq num underflow")?;

        let mut ordered_checkpoints = checkpoints;
        ordered_checkpoints.sort();
        if *ordered_checkpoints.first().unwrap() > latest_available_checkpoint {
            return Err(anyhow!(
                "Latest available checkpoint is: {}",
                latest_available_checkpoint
            ));
        }

        let checkpoint_files: Vec<FileMetadata> = self.verify_manifest(manifest).await?;

        let mut checkpoints_filtered = vec![];
        for checkpoint in ordered_checkpoints.iter() {
            let index = checkpoint_files
                .binary_search_by(|s| {
                    if checkpoint < &s.checkpoint_seq_range.start {
                        std::cmp::Ordering::Greater
                    } else if checkpoint >= &s.checkpoint_seq_range.end {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .expect("Archive does not contain checkpoint {checkpoint}");
            checkpoints_filtered.push(checkpoint_files[index].clone());
        }

        Ok(checkpoints_filtered)
    }

    fn spawn_manifest_sync_task<S: ObjectStoreGetExt + Clone>(
        remote_store: S,
        manifest: Arc<Mutex<Manifest>>,
        mut recv: oneshot::Receiver<()>,
    ) {
        tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let new_manifest = read_manifest(remote_store.clone()).await?;
                        let mut locked = manifest.lock().await;
                        *locked = new_manifest;
                    }
                    _ = &mut recv => break,
                }
            }
            info!("Terminating the manifest sync loop");
            Ok::<(), anyhow::Error>(())
        });
    }
}
