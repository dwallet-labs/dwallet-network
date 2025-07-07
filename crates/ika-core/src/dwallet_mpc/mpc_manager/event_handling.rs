use tracing::{debug, error, warn};
use ika_types::messages_dwallet_mpc::DWalletMPCEvent;
use crate::dwallet_mpc::mpc_manager::DWalletMPCManager;

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
    pub(crate) fn handle_dwallet_db_events(&mut self, events: Vec<DWalletMPCEvent>) {
        // First try to update the network keys, and then attend to events one by one.
        self.update_network_keys();

        for event in events {
            self
                .handle_dwallet_db_event(event);
        }
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
    fn handle_dwallet_db_event(&mut self, event: DWalletMPCEvent) {
        // TODO: take `DBSuiEvent`.
        // TODO: reject events coming not from our module:
        // address: *packages_config.ika_system_package_id,
        // module: DWALLET_MODULE_NAME.to_owned(),


        // Avoid instantiation of completed events by checking they belong to the current epoch.
        if !event.override_epoch_check && event.session_info.epoch != self.epoch_id {
            warn!(
                session_identifier=?event.session_info.session_identifier,
                event_type=?event.event,
                event_epoch=?event.session_info.epoch,
                "received an event for a different epoch, skipping"
            );
            return;
        }

        // TODO: deserialize here first into an enum - but then we might deserialize for no good reason.

        if let Some(network_encryption_key_id) = event.network_encryption_key_id() {
            if !self.network_keys.key_public_data_exists(network_encryption_key_id) {
                // We don't yet have the data for this network encryption key, so we add it to the queue.
                debug!(
                        session_info=?event.session_info,
                        type=?event.event.type_,
                        network_encryption_key_id=?network_encryption_key_id,
                        "Adding event to pending for the network key"
                    );

                // TODO: need to check it doesn't exist first
                // event.session_info.session_identifier

                self.events_pending_for_network_key.entry(network_encryption_key_id).or_insert(vec![]).push(event);

                return;
            }
        }

        if event.requires_next_active_committee() && self.next_active_committee.is_none() {
            // We don't have the next active committee yet, so we have to add this event to the pending queue until it arrives.
            debug!(
                        session_info=?event.session_info,
                        type=?event.event.type_,
                        "Adding event to pending for the next epoch active committee"
                    );

            // TODO: need to check it doesn't exist first
            self.events_pending_for_next_active_committee.push(event);

            return;
        }

        let session_info = event.session_info;
        let event = event.event.clone();

        if let Some(session) = self.mpc_sessions.get(&session_info.session_identifier) {
            if session.mpc_event_data.is_some() {
                // The corresponding session already has its event data set, nothing to do.
                return;
            }
        }

        let mpc_event_data = match self.new_mpc_event_data(event, &session_info, self.next_active_committee.clone()) {
            Ok(mpc_event_data) => mpc_event_data,
            Err(e) => {
                error!(e=?e, event=?event, "failed to handle dWallet MPC event with error");

                return;
            }
        };

        self.dwallet_mpc_metrics
            .add_received_event_start(&mpc_event_data.init_protocol_data);

        if let Some(session) = self.mpc_sessions.get(&session_info.session_identifier) {
            if session.mpc_event_data.is_none() {
                if let Some(session) = self.mpc_sessions.get_mut(&session_info.session_identifier) {
                    session.mpc_event_data = Some(mpc_event_data.clone());
                }

                // It could be that this session was pending for computation, but was missing the event.
                // In that case, move it to the right queue.
                if let (index) = self.sessions_pending_for_events.iter().position(|session_pending_for_event| session_pending_for_event.session_identifier == session.session_identifier) {
                    // Safe to `unwrap()`, we just got this index.
                    let mut ready_to_advance_session_copy = self.sessions_pending_for_events.remove(index).unwrap();

                    ready_to_advance_session_copy.mpc_event_data = mpc_event_data;

                    self.insert_session_into_ordered_pending_for_computation_queue(ready_to_advance_session_copy);
                }
            }
        } else {
            self.new_mpc_session(&session_info.session_identifier, Some(mpc_event_data));
        }

        Ok(())
    }

    // TODO: deserialize
}