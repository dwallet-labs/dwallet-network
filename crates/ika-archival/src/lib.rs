// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
#![allow(dead_code)]

pub mod reader;
pub mod writer;

#[cfg(test)]
mod tests;

use crate::reader::{ArchiveReader, ArchiveReaderMetrics};
use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::Bytes;
use fastcrypto::hash::{HashFunction, Sha3_256};
use ika_config::node::ArchiveReaderConfig;
use ika_config::object_storage_config::ObjectStoreConfig;
use ika_types::messages_checkpoint::CheckpointSequenceNumber;
use ika_types::storage::{SingleCheckpointSharedInMemoryStore, WriteStore};
use indicatif::{ProgressBar, ProgressStyle};
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use object_store::path::Path;
use prometheus::Registry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufWriter, Cursor, Read, Seek, SeekFrom, Write};
use std::num::NonZeroUsize;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sui_storage::blob::{Blob, BlobEncoding};
use sui_storage::object_store::util::{get, put};
use sui_storage::object_store::{ObjectStoreGetExt, ObjectStorePutExt};
use sui_storage::{compute_sha3_checksum, compute_sha3_checksum_for_bytes, SHA3_BYTES};
use tracing::{error, info};

#[allow(rustdoc::invalid_html_tags)]
/// Checkpoints are persisted as blob files. Files are committed to local store
/// by duration or file size. Committed files are synced with the remote store continuously. Files are
/// optionally compressed with the zstd compression format. Filenames follow the format
/// <checkpoint_seq_num>.<suffix> where `checkpoint_seq_num` is the first checkpoint present in that
/// file. MANIFEST is the index and source of truth for all files present in the archive.
///
/// State Archival Directory Layout
///  - archive/
///     - MANIFEST
///     - epoch_0/
///        - 0.ika_checkpoint
///        - 1000.ika_checkpoint
///        - 3000.ika_checkpoint
///        - ...
///        - 100000.ika_checkpoint
///     - epoch_1/
///        - 101000.ika_checkpoint
///        - ...
///
/// Blob File Disk Format
///┌──────────────────────────────┐
///│       magic <4 byte>         │
///├──────────────────────────────┤
///│  storage format <1 byte>     │
// ├──────────────────────────────┤
///│    file compression <1 byte> │
// ├──────────────────────────────┤
///│ ┌──────────────────────────┐ │
///│ │         Blob 1           │ │
///│ ├──────────────────────────┤ │
///│ │          ...             │ │
///│ ├──────────────────────────┤ │
///│ │        Blob N            │ │
///│ └──────────────────────────┘ │
///└──────────────────────────────┘
/// Blob
///┌───────────────┬───────────────────┬──────────────┐
///│ len <uvarint> │ encoding <1 byte> │ data <bytes> │
///└───────────────┴───────────────────┴──────────────┘
///
/// MANIFEST File Disk Format
///┌──────────────────────────────┐
///│        magic<4 byte>         │
///├──────────────────────────────┤
///│   serialized manifest        │
///├──────────────────────────────┤
///│      sha3 <32 bytes>         │
///└──────────────────────────────┘
pub const CHECKPOINT_MESSAGE_FILE_MAGIC: u32 = 0x0000DEAD;
const MANIFEST_FILE_MAGIC: u32 = 0x00C0FFEE;
const MAGIC_BYTES: usize = 4;
const CHECKPOINT_FILE_SUFFIX: &str = "ika_checkpoint";
const EPOCH_DIR_PREFIX: &str = "epoch_";
const MANIFEST_FILENAME: &str = "MANIFEST";

pub const PARAMS_MESSAGE_FILE_MAGIC: u32 = 0x0000DEAD;
const PARAMS_MESSAGE_FILE_SUFFIX: &str = "ika_ika_system_checkpoint";

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum FileType {
    CheckpointMessage = 0,
    IkaSystemCheckpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FileMetadata {
    pub file_type: FileType,
    pub epoch_num: u64,
    pub checkpoint_seq_range: Range<u64>,
    pub ika_system_checkpoint_seq_range: Range<u64>,
    pub sha3_digest: [u8; 32],
}

impl FileMetadata {
    pub fn file_path(&self) -> Path {
        let dir_path = Path::from(format!("{}{}", EPOCH_DIR_PREFIX, self.epoch_num));
        match self.file_type {
            FileType::CheckpointMessage => dir_path.child(&*format!(
                "{}.{CHECKPOINT_FILE_SUFFIX}",
                self.checkpoint_seq_range.start
            )),
            FileType::IkaSystemCheckpoint => dir_path.child(&*format!(
                "{}.{CHECKPOINT_FILE_SUFFIX}",
                self.ika_system_checkpoint_seq_range.start
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ManifestV1 {
    pub archive_version: u8,
    pub next_checkpoint_seq_num: u64,
    pub next_ika_system_checkpoint_seq_num: u64,
    pub file_metadata: Vec<FileMetadata>,
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Manifest {
    V1(ManifestV1),
}

impl Manifest {
    pub fn new(
        epoch: u64,
        next_checkpoint_seq_num: u64,
        next_ika_system_checkpoint_seq_num: u64,
    ) -> Self {
        Manifest::V1(ManifestV1 {
            archive_version: 1,
            next_checkpoint_seq_num,
            next_ika_system_checkpoint_seq_num,
            file_metadata: vec![],
            epoch,
        })
    }
    pub fn files(&self) -> Vec<FileMetadata> {
        match self {
            Manifest::V1(manifest) => manifest.file_metadata.clone(),
        }
    }
    pub fn epoch_num(&self) -> u64 {
        match self {
            Manifest::V1(manifest) => manifest.epoch,
        }
    }
    pub fn next_checkpoint_seq_num(&self) -> u64 {
        match self {
            Manifest::V1(manifest) => manifest.next_checkpoint_seq_num,
        }
    }
    pub fn next_checkpoint_after_epoch(&self, epoch_num: u64) -> u64 {
        match self {
            Manifest::V1(manifest) => {
                let mut summary_files: Vec<_> = manifest
                    .file_metadata
                    .clone()
                    .into_iter()
                    .filter(|f| f.file_type == FileType::CheckpointMessage)
                    .collect();
                summary_files.sort_by_key(|f| f.checkpoint_seq_range.start);
                assert!(summary_files
                    .windows(2)
                    .all(|w| w[1].checkpoint_seq_range.start == w[0].checkpoint_seq_range.end));
                assert_eq!(summary_files.first().unwrap().checkpoint_seq_range.start, 0);
                summary_files
                    .iter()
                    .find(|f| f.epoch_num > epoch_num)
                    .map(|f| f.checkpoint_seq_range.start)
                    .unwrap_or(u64::MAX)
            }
        }
    }

    pub fn next_ika_system_checkpoint_seq_num(&self) -> u64 {
        match self {
            Manifest::V1(manifest) => manifest.next_ika_system_checkpoint_seq_num,
        }
    }
    pub fn next_ika_system_checkpoint_after_epoch(&self, epoch_num: u64) -> u64 {
        match self {
            Manifest::V1(manifest) => {
                let mut summary_files: Vec<_> = manifest
                    .file_metadata
                    .clone()
                    .into_iter()
                    .filter(|f| f.file_type == FileType::IkaSystemCheckpoint)
                    .collect();
                summary_files.sort_by_key(|f| f.ika_system_checkpoint_seq_range.start);
                assert!(summary_files
                    .windows(2)
                    .all(|w| w[1].ika_system_checkpoint_seq_range.start
                        == w[0].ika_system_checkpoint_seq_range.end));
                assert_eq!(
                    summary_files
                        .first()
                        .unwrap()
                        .ika_system_checkpoint_seq_range
                        .start,
                    0
                );
                summary_files
                    .iter()
                    .find(|f| f.epoch_num > epoch_num)
                    .map(|f| f.ika_system_checkpoint_seq_range.start)
                    .unwrap_or(u64::MAX)
            }
        }
    }

    pub fn update(
        &mut self,
        epoch_num: u64,
        checkpoint_sequence_number: u64,
        checkpoint_file_metadata: FileMetadata,
    ) {
        match self {
            Manifest::V1(manifest) => {
                manifest
                    .file_metadata
                    .extend(vec![checkpoint_file_metadata]);
                manifest.epoch = epoch_num;
                manifest.next_checkpoint_seq_num = checkpoint_sequence_number;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CheckpointUpdates {
    checkpoint_file_metadata: FileMetadata,
    manifest: Manifest,
}

impl CheckpointUpdates {
    pub fn new(
        epoch_num: u64,
        checkpoint_sequence_number: u64,
        checkpoint_file_metadata: FileMetadata,
        manifest: &mut Manifest,
    ) -> Self {
        manifest.update(
            epoch_num,
            checkpoint_sequence_number,
            checkpoint_file_metadata.clone(),
        );
        CheckpointUpdates {
            checkpoint_file_metadata,
            manifest: manifest.clone(),
        }
    }
    pub fn content_file_path(&self) -> Path {
        self.checkpoint_file_metadata.file_path()
    }
    pub fn summary_file_path(&self) -> Path {
        self.checkpoint_file_metadata.file_path()
    }
    pub fn manifest_file_path(&self) -> Path {
        Path::from(MANIFEST_FILENAME)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct IkaSystemCheckpointUpdates {
    ika_system_checkpoint_file_metadata: FileMetadata,
    manifest: Manifest,
}

impl IkaSystemCheckpointUpdates {
    pub fn new(
        epoch_num: u64,
        ika_system_checkpoint_sequence_number: u64,
        ika_system_checkpoint_file_metadata: FileMetadata,
        manifest: &mut Manifest,
    ) -> Self {
        manifest.update(
            epoch_num,
            ika_system_checkpoint_sequence_number,
            ika_system_checkpoint_file_metadata.clone(),
        );
        IkaSystemCheckpointUpdates {
            ika_system_checkpoint_file_metadata,
            manifest: manifest.clone(),
        }
    }
    pub fn content_file_path(&self) -> Path {
        self.ika_system_checkpoint_file_metadata.file_path()
    }
    pub fn summary_file_path(&self) -> Path {
        self.ika_system_checkpoint_file_metadata.file_path()
    }
    pub fn manifest_file_path(&self) -> Path {
        Path::from(MANIFEST_FILENAME)
    }
}

pub fn create_file_metadata(
    file_path: &std::path::Path,
    file_type: FileType,
    epoch_num: u64,
    checkpoint_seq_range: Range<u64>,
    ika_system_checkpoint_seq_range: Range<u64>,
) -> Result<FileMetadata> {
    let sha3_digest = compute_sha3_checksum(file_path)?;
    let file_metadata = FileMetadata {
        file_type,
        epoch_num,
        checkpoint_seq_range,
        ika_system_checkpoint_seq_range,
        sha3_digest,
    };
    Ok(file_metadata)
}

pub fn create_file_metadata_from_bytes(
    bytes: Bytes,
    file_type: FileType,
    epoch_num: u64,
    checkpoint_seq_range: Range<u64>,
    ika_system_checkpoint_seq_range: Range<u64>,
) -> Result<FileMetadata> {
    let sha3_digest = compute_sha3_checksum_for_bytes(bytes)?;
    let file_metadata = FileMetadata {
        file_type,
        epoch_num,
        checkpoint_seq_range,
        ika_system_checkpoint_seq_range,
        sha3_digest,
    };
    Ok(file_metadata)
}

pub async fn read_manifest<S: ObjectStoreGetExt>(remote_store: S) -> Result<Manifest> {
    let manifest_file_path = Path::from(MANIFEST_FILENAME);
    let vec = get(&remote_store, &manifest_file_path).await?.to_vec();
    read_manifest_from_bytes(vec)
}

pub fn read_manifest_from_bytes(vec: Vec<u8>) -> Result<Manifest> {
    let manifest_file_size = vec.len();
    let mut manifest_reader = Cursor::new(vec);
    manifest_reader.rewind()?;
    let magic = manifest_reader.read_u32::<BigEndian>()?;
    if magic != MANIFEST_FILE_MAGIC {
        return Err(anyhow!("Unexpected magic byte in manifest: {}", magic));
    }
    manifest_reader.seek(SeekFrom::End(-(SHA3_BYTES as i64)))?;
    let mut sha3_digest = [0u8; SHA3_BYTES];
    manifest_reader.read_exact(&mut sha3_digest)?;
    manifest_reader.rewind()?;
    let mut content_buf = vec![0u8; manifest_file_size - SHA3_BYTES];
    manifest_reader.read_exact(&mut content_buf)?;
    let mut hasher = Sha3_256::default();
    hasher.update(&content_buf);
    let computed_digest = hasher.finalize().digest;
    if computed_digest != sha3_digest {
        return Err(anyhow!(
            "Manifest corrupted, computed checksum: {:?}, stored checksum: {:?}",
            computed_digest,
            sha3_digest
        ));
    }
    manifest_reader.rewind()?;
    manifest_reader.seek(SeekFrom::Start(MAGIC_BYTES as u64))?;
    Blob::read(&mut manifest_reader)?.decode()
}

pub fn finalize_manifest(manifest: Manifest) -> Result<Bytes> {
    let mut buf = BufWriter::new(vec![]);
    buf.write_u32::<BigEndian>(MANIFEST_FILE_MAGIC)?;
    let blob = Blob::encode(&manifest, BlobEncoding::Bcs)?;
    blob.write(&mut buf)?;
    buf.flush()?;
    let mut hasher = Sha3_256::default();
    hasher.update(buf.get_ref());
    let computed_digest = hasher.finalize().digest;
    buf.write_all(&computed_digest)?;
    Ok(Bytes::from(buf.into_inner()?))
}

pub async fn write_manifest<S: ObjectStorePutExt>(
    manifest: Manifest,
    remote_store: S,
) -> Result<()> {
    let path = Path::from(MANIFEST_FILENAME);
    let bytes = finalize_manifest(manifest)?;
    put(&remote_store, &path, bytes).await?;
    Ok(())
}

pub async fn read_manifest_as_json(remote_store_config: ObjectStoreConfig) -> Result<String> {
    let metrics = ArchiveReaderMetrics::new(&Registry::default());
    let config = ArchiveReaderConfig {
        remote_store_config,
        download_concurrency: NonZeroUsize::new(1).unwrap(),
        use_for_pruning_watermark: false,
    };
    let archive_reader = ArchiveReader::new(config, &metrics)?;
    archive_reader.sync_manifest_once().await?;
    let manifest = archive_reader.get_manifest().await?;
    let json = serde_json::to_string(&manifest).expect("Failed to serialize object");
    Ok(json)
}

pub async fn write_manifest_from_json(
    remote_store_config: ObjectStoreConfig,
    json_manifest_path: std::path::PathBuf,
) -> Result<()> {
    let manifest: Manifest = serde_json::from_str(&fs::read_to_string(json_manifest_path)?)?;
    let store = remote_store_config.make()?;
    write_manifest(manifest, store).await?;
    Ok(())
}

pub async fn verify_archive_with_checksums(
    remote_store_config: ObjectStoreConfig,
    concurrency: usize,
) -> Result<()> {
    let metrics = ArchiveReaderMetrics::new(&Registry::default());
    let config = ArchiveReaderConfig {
        remote_store_config,
        download_concurrency: NonZeroUsize::new(concurrency).unwrap(),
        use_for_pruning_watermark: false,
    };
    let archive_reader = ArchiveReader::new(config, &metrics)?;
    archive_reader.sync_manifest_once().await?;
    let manifest = archive_reader.get_manifest().await?;
    info!(
        "Next checkpoint in archive store: {}",
        manifest.next_checkpoint_seq_num()
    );

    let file_metadata = archive_reader.verify_manifest(manifest).await?;
    // Account for both summary and content files
    let num_files = file_metadata.len() * 2;
    archive_reader
        .verify_file_consistency(file_metadata)
        .await?;
    info!("All {} files are valid", num_files);
    Ok(())
}

pub async fn verify_archive_with_local_store<S>(
    store: S,
    remote_store_config: ObjectStoreConfig,
    concurrency: usize,
    interactive: bool,
) -> Result<()>
where
    S: WriteStore + Clone + Send + 'static,
{
    let metrics = ArchiveReaderMetrics::new(&Registry::default());
    let config = ArchiveReaderConfig {
        remote_store_config,
        download_concurrency: NonZeroUsize::new(concurrency).unwrap(),
        use_for_pruning_watermark: false,
    };
    let archive_reader = ArchiveReader::new(config, &metrics)?;
    archive_reader.sync_manifest_once().await?;
    let latest_checkpoint_in_archive = archive_reader.latest_available_checkpoint().await?;
    info!(
        "Latest available checkpoint in archive store: {}",
        latest_checkpoint_in_archive
    );
    let latest_checkpoint = store
        .get_highest_synced_checkpoint()
        .map_err(|_| anyhow!("Failed to read highest synced checkpoint"))?
        .map(|c| c.sequence_number)
        .unwrap_or(0);
    info!("Highest synced checkpoint in db: {latest_checkpoint}");
    let action_counter = Arc::new(AtomicU64::new(0));
    let checkpoint_counter = Arc::new(AtomicU64::new(0));
    let progress_bar = if interactive {
        let progress_bar = ProgressBar::new(latest_checkpoint_in_archive).with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos}/{len}({msg})")
                .unwrap(),
        );
        let cloned_progress_bar = progress_bar.clone();
        let cloned_counter = action_counter.clone();
        let cloned_checkpoint_counter = checkpoint_counter.clone();
        let instant = Instant::now();
        tokio::spawn(async move {
            loop {
                let total_checkpoints_loaded = cloned_checkpoint_counter.load(Ordering::Relaxed);
                let total_checkpoints_per_sec =
                    total_checkpoints_loaded as f64 / instant.elapsed().as_secs_f64();
                let total_txns_per_sec =
                    cloned_counter.load(Ordering::Relaxed) as f64 / instant.elapsed().as_secs_f64();
                cloned_progress_bar.set_position(latest_checkpoint + total_checkpoints_loaded);
                cloned_progress_bar.set_message(format!(
                    "checkpoints/s: {}, txns/s: {}",
                    total_checkpoints_per_sec, total_txns_per_sec
                ));
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Some(progress_bar)
    } else {
        let cloned_store = store.clone();
        tokio::spawn(async move {
            loop {
                let latest_checkpoint = cloned_store
                    .get_highest_synced_checkpoint()
                    .map_err(|_| anyhow!("Failed to read highest synced checkpoint"))?
                    .map(|c| c.sequence_number)
                    .unwrap_or(0);
                let percent = (latest_checkpoint * 100) / latest_checkpoint_in_archive;
                info!("done = {percent}%");
                tokio::time::sleep(Duration::from_secs(60)).await;
                if percent >= 100 {
                    break;
                }
            }
            Ok::<(), anyhow::Error>(())
        });
        None
    };
    archive_reader
        .read(
            store.clone(),
            (latest_checkpoint + 1)..u64::MAX,
            action_counter,
            checkpoint_counter,
        )
        .await?;
    progress_bar.iter().for_each(|p| p.finish_and_clear());
    let end = store
        .get_highest_synced_checkpoint()
        .map_err(|_| anyhow!("Failed to read watermark"))?
        .map(|c| c.sequence_number)
        .unwrap_or(0);
    info!("Highest verified checkpoint: {}", end);
    Ok(())
}

pub async fn verify_archive_with_local_store_ika_system_checkpoint<S>(
    store: S,
    remote_store_config: ObjectStoreConfig,
    concurrency: usize,
    interactive: bool,
) -> Result<()>
where
    S: WriteStore + Clone + Send + 'static,
{
    let metrics = ArchiveReaderMetrics::new(&Registry::default());
    let config = ArchiveReaderConfig {
        remote_store_config,
        download_concurrency: NonZeroUsize::new(concurrency).unwrap(),
        use_for_pruning_watermark: false,
    };
    let archive_reader = ArchiveReader::new(config, &metrics)?;
    archive_reader.sync_manifest_once().await?;
    let latest_ika_system_checkpoint_in_archive = archive_reader
        .latest_available_ika_system_checkpoint()
        .await?;
    info!(
        "Latest available ika_system_checkpoint in archive store: {}",
        latest_ika_system_checkpoint_in_archive
    );
    let latest_ika_system_checkpoint = store
        .get_highest_synced_ika_system_checkpoint()
        .map_err(|_| anyhow!("Failed to read highest synced ika_system_checkpoint"))?
        .map(|c| c.sequence_number)
        .unwrap_or(0);
    info!("Highest synced ika_system_checkpoint in db: {latest_ika_system_checkpoint}");
    let action_counter = Arc::new(AtomicU64::new(0));
    let ika_system_checkpoint_counter = Arc::new(AtomicU64::new(0));
    let progress_bar = if interactive {
        let progress_bar = ProgressBar::new(latest_ika_system_checkpoint_in_archive).with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos}/{len}({msg})")
                .unwrap(),
        );
        let cloned_progress_bar = progress_bar.clone();
        let cloned_counter = action_counter.clone();
        let cloned_ika_system_checkpoint_counter = ika_system_checkpoint_counter.clone();
        let instant = Instant::now();
        tokio::spawn(async move {
            loop {
                let total_ika_system_checkpoints_loaded =
                    cloned_ika_system_checkpoint_counter.load(Ordering::Relaxed);
                let total_ika_system_checkpoints_per_sec =
                    total_ika_system_checkpoints_loaded as f64 / instant.elapsed().as_secs_f64();
                let total_txns_per_sec =
                    cloned_counter.load(Ordering::Relaxed) as f64 / instant.elapsed().as_secs_f64();
                cloned_progress_bar.set_position(
                    latest_ika_system_checkpoint + total_ika_system_checkpoints_loaded,
                );
                cloned_progress_bar.set_message(format!(
                    "ika_system_checkpoints/s: {}, txns/s: {}",
                    total_ika_system_checkpoints_per_sec, total_txns_per_sec
                ));
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Some(progress_bar)
    } else {
        let cloned_store = store.clone();
        tokio::spawn(async move {
            loop {
                let latest_ika_system_checkpoint = cloned_store
                    .get_highest_synced_ika_system_checkpoint()
                    .map_err(|_| anyhow!("Failed to read highest synced ika_system_checkpoint"))?
                    .map(|c| c.sequence_number)
                    .unwrap_or(0);
                let percent =
                    (latest_ika_system_checkpoint * 100) / latest_ika_system_checkpoint_in_archive;
                info!("done = {percent}%");
                tokio::time::sleep(Duration::from_secs(60)).await;
                if percent >= 100 {
                    break;
                }
            }
            Ok::<(), anyhow::Error>(())
        });
        None
    };
    archive_reader
        .read_ika_system_checkpoints(
            store.clone(),
            (latest_ika_system_checkpoint + 1)..u64::MAX,
            action_counter,
            ika_system_checkpoint_counter,
        )
        .await?;
    progress_bar.iter().for_each(|p| p.finish_and_clear());
    let end = store
        .get_highest_synced_ika_system_checkpoint()
        .map_err(|_| anyhow!("Failed to read watermark"))?
        .map(|c| c.sequence_number)
        .unwrap_or(0);
    info!("Highest verified ika_system_checkpoint: {}", end);
    Ok(())
}
