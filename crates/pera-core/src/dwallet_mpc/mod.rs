use pera_types::base_types::{EpochId, ObjectID};

mod dkg;
pub mod mpc_events;
pub mod mpc_instance;
pub mod mpc_manager;
pub mod mpc_outputs_manager;
pub mod mpc_party;
pub mod network_dkg;
mod presign;
pub mod sign;

const SECP256K1_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(0);
const RISTRETTO_DKG_SESSION_ID: ObjectID = ObjectID::from_single_byte(1);
pub const FIRST_EPOCH_ID: EpochId = 0;
