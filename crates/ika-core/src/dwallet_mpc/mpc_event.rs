use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::deserialize_event_contents;
use crate::dwallet_mpc::dwallet_dkg::{
    dwallet_dkg_first_party_session_request, dwallet_dkg_second_party_session_request,
    dwallet_imported_key_verification_request_event_session_request,
};
use crate::dwallet_mpc::dwallet_mpc_service::DWalletMPCService;
use crate::dwallet_mpc::encrypt_user_share::start_encrypted_share_verification_session_request;
use crate::dwallet_mpc::make_dwallet_user_secret_key_shares_public::make_dwallet_user_secret_key_shares_public_request_event_session_request;
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;
use crate::dwallet_mpc::mpc_session::MPCEventData;
use crate::dwallet_mpc::network_dkg::network_dkg_session_request;
use crate::dwallet_mpc::presign::presign_party_session_request;
use crate::dwallet_mpc::reconfiguration::network_decryption_key_reconfiguration_session_request_from_event;
use crate::dwallet_mpc::sign::{
    get_verify_partial_signatures_session_request, sign_party_session_request,
};
use dwallet_mpc_types::dwallet_mpc::DWalletMPCNetworkKeyScheme;
use ika_types::committee::Committee;
use ika_types::dwallet_mpc_error::DwalletMPCError;
use ika_types::error::{IkaError, IkaResult};
use ika_types::messages_dwallet_mpc::{
    DBSuiEvent, DWalletDKGFirstRoundRequestEvent, DWalletDKGSecondRoundRequestEvent,
    DWalletEncryptionKeyReconfigurationRequestEvent, DWalletImportedKeyVerificationRequestEvent,
    DWalletMPCEvent, DWalletNetworkDKGEncryptionKeyRequestEvent, DWalletSessionEvent,
    DWalletSessionEventTrait, EncryptedShareVerificationRequestEvent, FutureSignRequestEvent,
    MakeDWalletUserSecretKeySharesPublicRequestEvent, PresignRequestEvent, SignRequestEvent,
    DWALLET_MODULE_NAME,
};
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

impl DWalletMPCManager {
    /// Handle a batch of MPC events.
    ///
    /// This function might be called more than once for a given session, as we periodically
    /// check for uncompleted events - in which case the event will be ignored.
    ///
    /// A new MPC session is only created once at the first time the event was received
    /// (per-epoch, if it was uncompleted in the previous epoch it will be created again for the next one.)
    ///
    /// If the event already exists in `self.mpc_sessions`, we do not add it.
    ///
    /// If there is no session info, and we've got it in this call,
    /// we update that field in the open session.
    pub(crate) fn handle_dwallet_db_events(
        &mut self,
        events: Vec<DBSuiEvent>,
        epoch_store: &AuthorityPerEpochStore,
    ) {
        // We only update `next_active_committee` here, so once we update it there is no longer going to be any pending events for it in this epoch.
        if self.next_active_committee.is_none() {
            let got_next_active_committee = self.try_receiving_next_active_committee();
            if got_next_active_committee {
                // `..` stands for `RangeFull`, and calling `drain(..)` will drain (i.e. mutate) the vector thus removing all events from it.
                let events_pending_for_next_active_committee: Vec<_> = self
                    .events_pending_for_next_active_committee
                    .drain(..)
                    .collect();

                for event in events_pending_for_next_active_committee {
                    self.handle_mpc_event(event);
                }
            }
        }

        // First try to update the network keys.
        let newly_updated_network_keys_ids = self.update_network_keys();

        // Then handle events for which we just received the public data for.
        // Since events are only added to the `events_pending_for_network_key` queue in this function,
        // we know once we got the network key data no more will be pending for that key,
        // so its safe to only handle these at the time of update, as it will remain an empty queue afterward.
        for key_id in newly_updated_network_keys_ids {
            for event in self
                .events_pending_for_network_key
                .remove(&key_id)
                .unwrap_or_default()
            {
                // We know this won't fail on missing network key, but it could be waiting for the next committee,
                // in which case it would be added to that queue.
                self.handle_mpc_event(event);
            }
        }

        for event in events {
            self.handle_sui_event(event, epoch_store);
        }
    }

    fn handle_sui_event(&mut self, event: DBSuiEvent, epoch_store: &AuthorityPerEpochStore) {
        if event.type_.address != *epoch_store.packages_config.ika_system_package_id
            || event.type_.module != DWALLET_MODULE_NAME.into()
        {
            error!(
                module=?event.type_.module,
                address=?event.type_.address,
                "received an event from a wrong SUI module - rejecting!"
            );

            return;
        }

        let event = match self.parse_sui_event(event.clone(), epoch_store) {
            Ok(event) => {
                debug!(
                    session_identifier=?event.session_request.session_identifier,
                    session_type=?event.session_request.session_type,
                    mpc_protocol=?event.session_request.request_input,
                    mpc_round=?event.session_request.request_input,
                    current_epoch=?epoch_store.epoch(),
                    "Successfully processed a missed event from Sui"
                );

                event
            }
            Err(e) => {
                error!(
                    event=?event,
                    error=?e,
                    "error while parsing SUI event"
                );

                return;
            }
        };

        self.handle_mpc_event(event)
    }

    /// Handle an MPC event.
    ///
    /// This function might be called more than once for a given session, as we periodically
    /// check for uncompleted events.
    ///
    /// A new MPC session is only created once at the first time the event was received
    /// (per-epoch, if it was uncompleted in the previous epoch it will be created again for the next one.)
    ///
    /// If the event already exists in `self.mpc_sessions`, we do not add it.
    ///
    /// If there is no session info, and we've got it in this call,
    /// we update that field in the open session.
    fn handle_mpc_event(&mut self, event: DWalletMPCEvent) {
        let session_identifier = event.session_request.session_identifier;

        // Avoid instantiation of completed events by checking they belong to the current epoch.
        // We only pull uncompleted events, so we skip the check for those, but pushed events might be completed.
        if !event.pulled && event.session_request.epoch != self.epoch_id {
            warn!(
                session_identifier=?session_identifier,
                session_type=?event.session_request.session_type,
                event_epoch=?event.session_request.epoch,
                "received an event for a different epoch, skipping"
            );

            return;
        }

        if let Some(network_encryption_key_id) = event
            .session_request
            .request_input
            .get_network_encryption_key_id()
        {
            if !self
                .network_keys
                .key_public_data_exists(&network_encryption_key_id)
            {
                // We don't yet have the data for this network encryption key, so we add it to the queue.
                debug!(
                    session_request=?event.session_request,
                    session_type=?event.session_request.session_type,
                    network_encryption_key_id=?network_encryption_key_id,
                    "Adding event to pending for the network key"
                );

                // TODO: need to check it doesn't exist first
                // event.session_request.session_identifier

                let events_pending_for_this_network_key = self
                    .events_pending_for_network_key
                    .entry(network_encryption_key_id)
                    .or_insert(vec![]);

                if events_pending_for_this_network_key
                    .iter()
                    .find(|e| e.session_request.session_identifier == session_identifier)
                    .is_none()
                {
                    // Add an event with this session ID only if it doesn't exist.
                    events_pending_for_this_network_key.push(event);
                }

                return;
            }
        }

        if event.session_request.requires_next_active_committee
            && self.next_active_committee.is_none()
        {
            // We don't have the next active committee yet, so we have to add this event to the pending queue until it arrives.
            debug!(
                session_request=?event.session_request,
                session_type=?event.session_request.session_type,
                "Adding event to pending for the next epoch active committee"
            );

            if self
                .events_pending_for_next_active_committee
                .iter()
                .find(|e| e.session_request.session_identifier == session_identifier)
                .is_none()
            {
                // Add an event with this session ID only if it doesn't exist.
                self.events_pending_for_next_active_committee.push(event);
            }

            return;
        }

        if let Some(session) = self.mpc_sessions.get(&session_identifier) {
            if session.mpc_event_data.is_some() {
                // The corresponding session already has its event data set, nothing to do.
                return;
            }
        }

        let mpc_event_data =
            match self.new_mpc_event_data(event.clone(), self.next_active_committee.clone()) {
                Ok(mpc_event_data) => mpc_event_data,
                Err(e) => {
                    error!(e=?e, event=?event, "failed to handle dWallet MPC event with error");

                    return;
                }
            };

        self.dwallet_mpc_metrics
            .add_received_event_start(&mpc_event_data.request_input);

        if let Some(session) = self.mpc_sessions.get_mut(&session_identifier) {
            if session.mpc_event_data.is_none() {
                session.mpc_event_data = Some(mpc_event_data.clone());

                // It could be that this session was pending for computation, but was missing the event.
                // In that case, move it to the right queue.
                if let Some(index) =
                    self.sessions_pending_for_events
                        .iter()
                        .position(|session_pending_for_event| {
                            session_pending_for_event.session_identifier
                                == session.session_identifier
                        })
                {
                    // Safe to `unwrap()`, we just got this index.
                    let mut ready_to_advance_session_copy =
                        self.sessions_pending_for_events.remove(index).unwrap();

                    ready_to_advance_session_copy.mpc_event_data = Some(mpc_event_data);

                    self.insert_session_into_ordered_pending_for_computation_queue(
                        ready_to_advance_session_copy,
                    );
                }
            }
        } else {
            self.new_mpc_session(&session_identifier, Some(mpc_event_data));
        }
    }

    /// Parses a SUI event into a dWallet MPC event.
    pub(crate) fn parse_sui_event(
        &self,
        event: DBSuiEvent,
        epoch_store: &AuthorityPerEpochStore,
    ) -> anyhow::Result<DWalletMPCEvent> {
        let packages_config = &epoch_store.packages_config;

        let session_request = if event.type_
            == DWalletSessionEvent::<DWalletImportedKeyVerificationRequestEvent>::type_(
                packages_config,
            ) {
            dwallet_imported_key_verification_request_event_session_request(
                deserialize_event_contents::<DWalletImportedKeyVerificationRequestEvent>(
                    &event.contents,
                    event.pulled,
                )?,
            )
        } else if event.type_
            == DWalletSessionEvent::<MakeDWalletUserSecretKeySharesPublicRequestEvent>::type_(
                packages_config,
            )
        {
            make_dwallet_user_secret_key_shares_public_request_event_session_request(
                deserialize_event_contents::<MakeDWalletUserSecretKeySharesPublicRequestEvent>(
                    &event.contents,
                    event.pulled,
                )?,
            )
        } else if event.type_
            == DWalletSessionEvent::<DWalletDKGFirstRoundRequestEvent>::type_(packages_config)
        {
            dwallet_dkg_first_party_session_request(deserialize_event_contents::<
                DWalletDKGFirstRoundRequestEvent,
            >(&event.contents, event.pulled)?)?
        } else if event.type_
            == DWalletSessionEvent::<DWalletDKGSecondRoundRequestEvent>::type_(packages_config)
        {
            dwallet_dkg_second_party_session_request(deserialize_event_contents::<
                DWalletDKGSecondRoundRequestEvent,
            >(&event.contents, event.pulled)?)
        } else if event.type_ == DWalletSessionEvent::<PresignRequestEvent>::type_(packages_config)
        {
            let deserialized_event: DWalletSessionEvent<PresignRequestEvent> =
                deserialize_event_contents(&event.contents, event.pulled)?;

            presign_party_session_request(deserialized_event)
        } else if event.type_ == DWalletSessionEvent::<SignRequestEvent>::type_(packages_config) {
            let deserialized_event: DWalletSessionEvent<SignRequestEvent> =
                deserialize_event_contents(&event.contents, event.pulled)?;

            sign_party_session_request(&deserialized_event)
        } else if event.type_
            == DWalletSessionEvent::<FutureSignRequestEvent>::type_(packages_config)
        {
            let deserialized_event: DWalletSessionEvent<FutureSignRequestEvent> =
                deserialize_event_contents(&event.contents, event.pulled)?;

            get_verify_partial_signatures_session_request(&deserialized_event)
        } else if event.type_
            == DWalletSessionEvent::<DWalletNetworkDKGEncryptionKeyRequestEvent>::type_(
                packages_config,
            )
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletNetworkDKGEncryptionKeyRequestEvent,
            > = deserialize_event_contents(&event.contents, event.pulled)?;

            network_dkg_session_request(deserialized_event, DWalletMPCNetworkKeyScheme::Secp256k1)?
        } else if event.type_
            == DWalletSessionEvent::<DWalletEncryptionKeyReconfigurationRequestEvent>::type_(
                packages_config,
            )
        {
            let deserialized_event: DWalletSessionEvent<
                DWalletEncryptionKeyReconfigurationRequestEvent,
            > = deserialize_event_contents(&event.contents, event.pulled)?;

            network_decryption_key_reconfiguration_session_request_from_event(deserialized_event)
        } else if event.type_
            == DWalletSessionEvent::<EncryptedShareVerificationRequestEvent>::type_(packages_config)
        {
            let deserialized_event: DWalletSessionEvent<EncryptedShareVerificationRequestEvent> =
                deserialize_event_contents(&event.contents, event.pulled)?;

            start_encrypted_share_verification_session_request(deserialized_event)
        } else {
            return Err(anyhow::anyhow!("unsupported event"));
        };

        let event = DWalletMPCEvent {
            session_request,
            pulled: event.pulled,
        };

        Ok(event)
    }

    fn new_mpc_event_data(
        &self,
        event: DWalletMPCEvent,
        next_active_committee: Option<Committee>,
    ) -> Result<MPCEventData, DwalletMPCError> {
        let epoch_store = self.epoch_store()?;

        MPCEventData::try_new(
            event,
            epoch_store,
            &self.network_keys,
            next_active_committee,
            self.validators_class_groups_public_keys_and_proofs.clone(),
        )
    }
}

impl DWalletMPCService {
    /// Proactively pull uncompleted events from the Sui network.
    /// We do that to assure we don't miss any events.
    /// These events might be from a different Epoch, not necessarily the current one.
    pub(crate) async fn fetch_uncompleted_events(&mut self) -> Vec<DBSuiEvent> {
        let epoch_store = self.epoch_store.clone();
        loop {
            match self
                .sui_client
                .pull_dwallet_mpc_uncompleted_events(epoch_store.epoch())
                .await
            {
                Ok(events) => {
                    return events;
                }
                Err(err) => {
                    error!(
                        ?err,
                        current_epoch=?self.epoch_store.epoch(),
                         "Failed to load missed events from Sui"
                    );
                    if let IkaError::EpochEnded(_) = err {
                        return vec![];
                    };
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    /// Read events from perpetual tables, remove them, and store in the current epoch tables.
    pub(crate) fn receive_new_sui_events(&mut self) -> IkaResult<Vec<DBSuiEvent>> {
        let pending_events = match self.new_events_receiver.try_recv() {
            Ok(events) => events,
            Err(broadcast::error::TryRecvError::Empty) => {
                debug!("No new Sui events to process");
                return Ok(vec![]);
            }
            Err(e) => {
                return Err(IkaError::ReveiverError(e.to_string()));
            }
        };

        let events: Vec<_> = pending_events
            .into_iter()
            .map(|event| DBSuiEvent {
                type_: event.type_,
                contents: event.bcs.into_bytes(),
                pulled: false,
            })
            .collect();

        Ok(events)
    }
}
