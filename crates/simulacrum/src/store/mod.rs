// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use pera_config::genesis;
use pera_types::base_types::ObjectRef;
use pera_types::error::UserInputError;
use pera_types::transaction::InputObjects;
use pera_types::transaction::ObjectReadResult;
use pera_types::transaction::ReceivingObjectReadResult;
use pera_types::transaction::ReceivingObjects;
use pera_types::{
    base_types::{ObjectID, PeraAddress, SequenceNumber},
    committee::{Committee, EpochId},
    digests::{ObjectDigest, TransactionDigest, TransactionEventsDigest},
    effects::{TransactionEffects, TransactionEffectsAPI, TransactionEvents},
    error::PeraResult,
    messages_checkpoint::{
        CheckpointContents, CheckpointContentsDigest, CheckpointDigest, CheckpointSequenceNumber,
        VerifiedCheckpoint,
    },
    object::Object,
    storage::{BackingStore, ChildObjectResolver, ParentSync},
    transaction::{InputObjectKind, VerifiedTransaction},
};
use std::collections::BTreeMap;
pub mod in_mem_store;

pub trait SimulatorStore:
    pera_types::storage::BackingPackageStore
    + pera_types::storage::ObjectStore
    + ParentSync
    + ChildObjectResolver
{
    fn init_with_genesis(&mut self, genesis: &genesis::Genesis) {
        self.insert_checkpoint(genesis.checkpoint());
        self.insert_checkpoint_contents(genesis.checkpoint_contents().clone());
        self.insert_committee(genesis.committee().unwrap());
        self.insert_transaction(VerifiedTransaction::new_unchecked(
            genesis.transaction().clone(),
        ));
        self.insert_transaction_effects(genesis.effects().clone());
        self.insert_events(
            genesis.effects().transaction_digest(),
            genesis.events().clone(),
        );

        self.update_objects(
            genesis
                .objects()
                .iter()
                .map(|o| (o.id(), o.clone()))
                .collect(),
            vec![],
        );
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Option<VerifiedCheckpoint>;

    fn get_checkpoint_by_digest(&self, digest: &CheckpointDigest) -> Option<VerifiedCheckpoint>;

    fn get_highest_checkpint(&self) -> Option<VerifiedCheckpoint>;

    fn get_checkpoint_contents(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> Option<CheckpointContents>;

    fn get_committee_by_epoch(&self, epoch: EpochId) -> Option<Committee>;

    fn get_transaction(&self, digest: &TransactionDigest) -> Option<VerifiedTransaction>;

    fn get_transaction_effects(&self, digest: &TransactionDigest) -> Option<TransactionEffects>;

    fn get_transaction_events(&self, digest: &TransactionEventsDigest)
        -> Option<TransactionEvents>;

    fn get_transaction_events_by_tx_digest(
        &self,
        tx_digest: &TransactionDigest,
    ) -> Option<TransactionEvents>;

    fn get_object(&self, id: &ObjectID) -> Option<Object>;

    fn get_object_at_version(&self, id: &ObjectID, version: SequenceNumber) -> Option<Object>;

    fn get_system_state(&self) -> pera_types::pera_system_state::PeraSystemState;

    fn get_clock(&self) -> pera_types::clock::Clock;

    fn owned_objects(&self, owner: PeraAddress) -> Box<dyn Iterator<Item = Object> + '_>;

    fn insert_checkpoint(&mut self, checkpoint: VerifiedCheckpoint);

    fn insert_checkpoint_contents(&mut self, contents: CheckpointContents);

    fn insert_committee(&mut self, committee: Committee);

    fn insert_executed_transaction(
        &mut self,
        transaction: VerifiedTransaction,
        effects: TransactionEffects,
        events: TransactionEvents,
        written_objects: BTreeMap<ObjectID, Object>,
    );

    fn insert_transaction(&mut self, transaction: VerifiedTransaction);

    fn insert_transaction_effects(&mut self, effects: TransactionEffects);

    fn insert_events(&mut self, tx_digest: &TransactionDigest, events: TransactionEvents);

    fn update_objects(
        &mut self,
        written_objects: BTreeMap<ObjectID, Object>,
        deleted_objects: Vec<(ObjectID, SequenceNumber, ObjectDigest)>,
    );

    fn backing_store(&self) -> &dyn BackingStore;

    // TODO: This function is now out-of-sync with read_objects_for_execution from transaction_input_loader.rs.
    // For instance, it does not support the use of deleted shared objects.
    // We will need to make SimulatorStore implement ExecutionCacheRead, and keep track of deleted shared objects
    // in a marker table in order to merge this function.
    fn read_objects_for_synchronous_execution(
        &self,
        _tx_digest: &TransactionDigest,
        input_object_kinds: &[InputObjectKind],
        receiving_object_refs: &[ObjectRef],
    ) -> PeraResult<(InputObjects, ReceivingObjects)> {
        let mut input_objects = Vec::new();
        for kind in input_object_kinds {
            let obj = match kind {
                InputObjectKind::MovePackage(id) => {
                    crate::store::SimulatorStore::get_object(self, id)
                }
                InputObjectKind::ImmOrOwnedMoveObject(objref) => {
                    self.get_object_by_key(&objref.0, objref.1)?
                }

                InputObjectKind::SharedMoveObject { id, .. } => {
                    crate::store::SimulatorStore::get_object(self, id)
                }
            };

            input_objects.push(ObjectReadResult::new(
                *kind,
                obj.ok_or_else(|| kind.object_not_found_error())?.into(),
            ));
        }

        let mut receiving_objects = Vec::new();
        for objref in receiving_object_refs {
            // no need for marker table check in simulacrum
            let Some(obj) = crate::store::SimulatorStore::get_object(self, &objref.0) else {
                return Err(UserInputError::ObjectNotFound {
                    object_id: objref.0,
                    version: Some(objref.1),
                }
                .into());
            };
            receiving_objects.push(ReceivingObjectReadResult::new(*objref, obj.into()));
        }

        Ok((input_objects.into(), receiving_objects.into()))
    }
}
