use dwallet_mpc_types::dwallet_mpc::{
    DWALLET_DECRYPTION_KEY_RESHARE_REQUEST_EVENT_NAME,
    DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME,
    DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME,
    DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT,
    DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT,
    ENCRYPTED_SHARE_VERIFICATION_REQUEST_EVENT_NAME, FUTURE_SIGN_REQUEST_EVENT_NAME,
    LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME, PRESIGN_REQUEST_EVENT_STRUCT_NAME,
    SIGN_REQUEST_EVENT_STRUCT_NAME, START_NETWORK_DKG_EVENT_STRUCT_NAME,
    VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME,
};
use im::HashMap;
use prometheus::{register_int_gauge_with_registry, IntGauge, Registry};
use std::sync::Arc;

struct MPCProtocolRoundMetrics {
    advance_calls: IntGauge,
    advance_completions: IntGauge,

    completions_count: IntGauge,
    last_completion_duration: IntGauge,
}

struct MPCProtocolMetrics {
    received_events_start_count: IntGauge,
    rounds_metrics: HashMap<usize, MPCProtocolRoundMetrics>,
}

pub struct DWalletMPCMetrics {
    protocols_metrics: HashMap<usize, MPCProtocolMetrics>,
}

impl DWalletMPCMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let mut protocols_metrics = HashMap::new();
        let mpc_protocols = vec![
            START_NETWORK_DKG_EVENT_STRUCT_NAME,
            DWALLET_DKG_FIRST_ROUND_REQUEST_EVENT_STRUCT_NAME,
            DWALLET_MAKE_DWALLET_USER_SECRET_KEY_SHARES_PUBLIC_REQUEST_EVENT,
            DWALLET_IMPORTED_KEY_VERIFICATION_REQUEST_EVENT,
            // TODO (#650): Rename Move structs
            DWALLET_DKG_SECOND_ROUND_REQUEST_EVENT_STRUCT_NAME,
            // TODO (#650): Rename Move structs
            PRESIGN_REQUEST_EVENT_STRUCT_NAME,
            SIGN_REQUEST_EVENT_STRUCT_NAME,
            LOCKED_NEXT_COMMITTEE_EVENT_STRUCT_NAME,
            VALIDATOR_DATA_FOR_SECRET_SHARE_STRUCT_NAME,
            START_NETWORK_DKG_EVENT_STRUCT_NAME,
            ENCRYPTED_SHARE_VERIFICATION_REQUEST_EVENT_NAME,
            FUTURE_SIGN_REQUEST_EVENT_NAME,
            DWALLET_DECRYPTION_KEY_RESHARE_REQUEST_EVENT_NAME,
        ];
        for protocol in mpc_protocols {
            
        }
    }
}
