use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use fastcrypto::encoding::{Base64, Encoding};
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use serde::Deserialize;
use ika_config::node::RootSeedWithPath;

#[derive(Deserialize)]
struct ClassGroupsKeyPairAndProofWrapper {
    inner: Box<ClassGroupsKeyPairAndProof>,
}

/// Reads a class group key pair and proof (encoded in Base64) from a file.
fn read_class_groups_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> DwalletMPCResult<Box<ClassGroupsKeyPairAndProof>> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| DwalletMPCError::FailedToReadSeed(e.to_string()))?;
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadSeed(e.to_string()))?;
    let keypair: ClassGroupsKeyPairAndProofWrapper = bcs::from_bytes(&decoded)?;
    Ok(keypair.inner)
}

pub fn create_full_class_groups_mock() -> Box<ClassGroupsKeyPairAndProof> {
    include_str!("../../../class-groups-keys-mock-files/class-groups-mock-key-full");
    let file_name = "class-groups-keys-mock-files/class-groups-mock-key-full".to_string();
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    // Safe to unwrap because the file is used for development purposes and should never fail
    read_class_groups_from_file(file_path).unwrap()
}

pub fn read_mock_root_seed() -> RootSeedWithPath {
    let file_name = "class-groups-keys-mock-files/root-seed-mock".to_string();
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    // Safe to unwrap because the file is used for development purposes and should never fail
    RootSeedWithPath::new_from_path(file_path)
}
