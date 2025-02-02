// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::authority::authority_store::{ExecutionLockWriteGuard, IkaLockResult};
use crate::authority::epoch_start_configuration::EpochFlag;
use crate::authority::epoch_start_configuration::EpochStartConfiguration;
use crate::authority::AuthorityStore;
use crate::state_accumulator::AccumulatorStore;
use crate::transaction_outputs::TransactionOutputs;

use futures::{future::BoxFuture, FutureExt};
use mysten_common::sync::notify_read::NotifyRead;
use prometheus::Registry;
use std::sync::Arc;
use ika_protocol_config::ProtocolVersion;
use ika_storage::package_object_cache::PackageObjectCache;
use ika_types::accumulator::Accumulator;
use ika_types::base_types::VerifiedExecutionData;
use ika_types::base_types::{EpochId, ObjectID, ObjectRef, SequenceNumber};
use ika_types::bridge::{get_bridge, Bridge};
use ika_types::digests::{TransactionDigest, TransactionEffectsDigest, TransactionEventsDigest};
use ika_types::effects::{TransactionEffects, TransactionEvents};
use ika_types::error::{IkaError, IkaResult};
use ika_types::message_envelope::Message;
use ika_types::messages_checkpoint::CheckpointSequenceNumber;
use ika_types::object::Object;
use ika_types::storage::{MarkerValue, ObjectKey, ObjectOrTombstone, ObjectStore, PackageObject};
use ika_types::ika_system_state::{get_ika_system_state, IkaSystemState};
use ika_types::transaction::{VerifiedSignedTransaction, VerifiedTransaction};
use tap::TapFallible;
use tracing::instrument;
use typed_store::Map;

use super::{
    implement_passthrough_traits, CheckpointCache, ExecutionCacheCommit, ExecutionCacheMetrics,
    ExecutionCacheReconfigAPI, ExecutionCacheWrite, ObjectCacheRead, StateSyncAPI, TestingAPI,
    TransactionCacheRead,
};

pub struct PassthroughCache {
    store: Arc<AuthorityStore>,
    metrics: Arc<ExecutionCacheMetrics>,
    package_cache: Arc<PackageObjectCache>,
    executed_effects_digests_notify_read: NotifyRead<TransactionDigest, TransactionEffectsDigest>,
}

impl PassthroughCache {
    pub fn new(store: Arc<AuthorityStore>, metrics: Arc<ExecutionCacheMetrics>) -> Self {
        Self {
            store,
            metrics,
            package_cache: PackageObjectCache::new(),
            executed_effects_digests_notify_read: NotifyRead::new(),
        }
    }

    pub fn new_for_tests(store: Arc<AuthorityStore>, registry: &Registry) -> Self {
        let metrics = Arc::new(ExecutionCacheMetrics::new(registry));
        Self::new(store, metrics)
    }

    pub fn store_for_testing(&self) -> &Arc<AuthorityStore> {
        &self.store
    }

    fn revert_state_update_impl(&self, digest: &TransactionDigest) -> IkaResult {
        self.store.revert_state_update(digest)
    }

    fn clear_state_end_of_epoch_impl(&self, execution_guard: &ExecutionLockWriteGuard) {
        self.store
            .clear_object_per_epoch_marker_table(execution_guard)
            .tap_err(|e| {
                tracing::error!(?e, "Failed to clear object per-epoch marker table");
            })
            .ok();
    }

    fn bulk_insert_genesis_objects_impl(&self, objects: &[Object]) -> IkaResult {
        self.store.bulk_insert_genesis_objects(objects)
    }

    fn insert_genesis_object_impl(&self, object: Object) -> IkaResult {
        self.store.insert_genesis_object(object)
    }
}

impl ObjectCacheRead for PassthroughCache {
    fn get_package_object(&self, package_id: &ObjectID) -> IkaResult<Option<PackageObject>> {
        self.package_cache
            .get_package_object(package_id, &*self.store)
    }

    fn force_reload_system_packages(&self, system_package_ids: &[ObjectID]) {
        self.package_cache
            .force_reload_system_packages(system_package_ids.iter().cloned(), self);
    }

    fn get_object(&self, id: &ObjectID) -> IkaResult<Option<Object>> {
        self.store.get_object(id).map_err(Into::into)
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: SequenceNumber,
    ) -> IkaResult<Option<Object>> {
        Ok(self.store.get_object_by_key(object_id, version)?)
    }

    fn multi_get_objects_by_key(
        &self,
        object_keys: &[ObjectKey],
    ) -> Result<Vec<Option<Object>>, IkaError> {
        Ok(self.store.multi_get_objects_by_key(object_keys)?)
    }

    fn object_exists_by_key(
        &self,
        object_id: &ObjectID,
        version: SequenceNumber,
    ) -> IkaResult<bool> {
        self.store.object_exists_by_key(object_id, version)
    }

    fn multi_object_exists_by_key(&self, object_keys: &[ObjectKey]) -> IkaResult<Vec<bool>> {
        self.store.multi_object_exists_by_key(object_keys)
    }

    fn get_latest_object_ref_or_tombstone(
        &self,
        object_id: ObjectID,
    ) -> IkaResult<Option<ObjectRef>> {
        self.store.get_latest_object_ref_or_tombstone(object_id)
    }

    fn get_latest_object_or_tombstone(
        &self,
        object_id: ObjectID,
    ) -> Result<Option<(ObjectKey, ObjectOrTombstone)>, IkaError> {
        self.store.get_latest_object_or_tombstone(object_id)
    }

    fn find_object_lt_or_eq_version(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
    ) -> IkaResult<Option<Object>> {
        self.store.find_object_lt_or_eq_version(object_id, version)
    }

    fn get_lock(&self, obj_ref: ObjectRef, epoch_store: &AuthorityPerEpochStore) -> IkaLockResult {
        self.store.get_lock(obj_ref, epoch_store)
    }

    fn _get_live_objref(&self, object_id: ObjectID) -> IkaResult<ObjectRef> {
        self.store.get_latest_live_version_for_object_id(object_id)
    }

    fn check_owned_objects_are_live(&self, owned_object_refs: &[ObjectRef]) -> IkaResult {
        self.store.check_owned_objects_are_live(owned_object_refs)
    }

    fn get_ika_system_state_object_unsafe(&self) -> IkaResult<IkaSystemState> {
        get_ika_system_state(self)
    }

    fn get_bridge_object_unsafe(&self) -> IkaResult<Bridge> {
        get_bridge(self)
    }

    fn get_marker_value(
        &self,
        object_id: &ObjectID,
        version: SequenceNumber,
        epoch_id: EpochId,
    ) -> IkaResult<Option<MarkerValue>> {
        self.store.get_marker_value(object_id, &version, epoch_id)
    }

    fn get_latest_marker(
        &self,
        object_id: &ObjectID,
        epoch_id: EpochId,
    ) -> IkaResult<Option<(SequenceNumber, MarkerValue)>> {
        self.store.get_latest_marker(object_id, epoch_id)
    }

    fn get_highest_pruned_checkpoint(&self) -> IkaResult<CheckpointSequenceNumber> {
        self.store.perpetual_tables.get_highest_pruned_checkpoint()
    }
}

impl TransactionCacheRead for PassthroughCache {
    fn multi_get_transaction_blocks(
        &self,
        digests: &[TransactionDigest],
    ) -> IkaResult<Vec<Option<Arc<VerifiedTransaction>>>> {
        Ok(self
            .store
            .multi_get_transaction_blocks(digests)?
            .into_iter()
            .map(|o| o.map(Arc::new))
            .collect())
    }

    fn multi_get_executed_effects_digests(
        &self,
        digests: &[TransactionDigest],
    ) -> IkaResult<Vec<Option<TransactionEffectsDigest>>> {
        self.store.multi_get_executed_effects_digests(digests)
    }

    fn multi_get_effects(
        &self,
        digests: &[TransactionEffectsDigest],
    ) -> IkaResult<Vec<Option<TransactionEffects>>> {
        Ok(self.store.perpetual_tables.effects.multi_get(digests)?)
    }

    fn notify_read_executed_effects_digests<'a>(
        &'a self,
        digests: &'a [TransactionDigest],
    ) -> BoxFuture<'a, IkaResult<Vec<TransactionEffectsDigest>>> {
        self.executed_effects_digests_notify_read
            .read(digests, |digests| {
                self.multi_get_executed_effects_digests(digests)
            })
            .boxed()
    }

    fn multi_get_events(
        &self,
        event_digests: &[TransactionEventsDigest],
    ) -> IkaResult<Vec<Option<TransactionEvents>>> {
        self.store.multi_get_events(event_digests)
    }
}

impl ExecutionCacheWrite for PassthroughCache {
    #[instrument(level = "debug", skip_all)]
    fn write_transaction_outputs<'a>(
        &'a self,
        epoch_id: EpochId,
        tx_outputs: Arc<TransactionOutputs>,
    ) -> BoxFuture<'a, IkaResult> {
        async move {
            let tx_digest = *tx_outputs.transaction.digest();
            let effects_digest = tx_outputs.effects.digest();

            // NOTE: We just check here that locks exist, not that they are locked to a specific TX. Why?
            // 1. Lock existence prevents re-execution of old certs when objects have been upgraded
            // 2. Not all validators lock, just 2f+1, so transaction should proceed regardless
            //    (But the lock should exist which means previous transactions finished)
            // 3. Equivocation possible (different TX) but as long as 2f+1 approves current TX its
            //    fine
            // 4. Locks may have existed when we started processing this tx, but could have since
            //    been deleted by a concurrent tx that finished first. In that case, check if the
            //    tx effects exist.
            self.store
                .check_owned_objects_are_live(&tx_outputs.locks_to_delete)?;

            self.store
                .write_transaction_outputs(epoch_id, &[tx_outputs])
                .await?;

            self.executed_effects_digests_notify_read
                .notify(&tx_digest, &effects_digest);

            self.metrics
                .pending_notify_read
                .set(self.executed_effects_digests_notify_read.num_pending() as i64);

            Ok(())
        }
        .boxed()
    }

    fn acquire_transaction_locks<'a>(
        &'a self,
        epoch_store: &'a AuthorityPerEpochStore,
        owned_input_objects: &'a [ObjectRef],
        tx_digest: TransactionDigest,
        signed_transaction: Option<VerifiedSignedTransaction>,
    ) -> BoxFuture<'a, IkaResult> {
        self.store
            .acquire_transaction_locks(
                epoch_store,
                owned_input_objects,
                tx_digest,
                signed_transaction,
            )
            .boxed()
    }
}

impl AccumulatorStore for PassthroughCache {
    fn get_object_ref_prior_to_key_deprecated(
        &self,
        object_id: &ObjectID,
        version: ika_types::base_types::VersionNumber,
    ) -> IkaResult<Option<ObjectRef>> {
        self.store
            .get_object_ref_prior_to_key_deprecated(object_id, version)
    }

    fn get_root_state_accumulator_for_epoch(
        &self,
        epoch: EpochId,
    ) -> IkaResult<Option<(CheckpointSequenceNumber, Accumulator)>> {
        self.store.get_root_state_accumulator_for_epoch(epoch)
    }

    fn get_root_state_accumulator_for_highest_epoch(
        &self,
    ) -> IkaResult<Option<(EpochId, (CheckpointSequenceNumber, Accumulator))>> {
        self.store.get_root_state_accumulator_for_highest_epoch()
    }

    fn insert_state_accumulator_for_epoch(
        &self,
        epoch: EpochId,
        checkpoint_seq_num: &CheckpointSequenceNumber,
        acc: &Accumulator,
    ) -> IkaResult {
        self.store
            .insert_state_accumulator_for_epoch(epoch, checkpoint_seq_num, acc)
    }

    fn iter_live_object_set(
        &self,
        include_wrapped_tombstone: bool,
    ) -> Box<dyn Iterator<Item = crate::authority::authority_store_tables::LiveObject> + '_> {
        self.store.iter_live_object_set(include_wrapped_tombstone)
    }
}

impl ExecutionCacheCommit for PassthroughCache {
    fn commit_transaction_outputs<'a>(
        &'a self,
        _epoch: EpochId,
        _digests: &'a [TransactionDigest],
    ) -> BoxFuture<'a, IkaResult> {
        // Nothing needs to be done since they were already committed in write_transaction_outputs
        async { Ok(()) }.boxed()
    }

    fn persist_transactions(&self, _digests: &[TransactionDigest]) -> BoxFuture<'_, IkaResult> {
        // Nothing needs to be done since they were already committed in write_transaction_outputs
        async { Ok(()) }.boxed()
    }
}

implement_passthrough_traits!(PassthroughCache);
