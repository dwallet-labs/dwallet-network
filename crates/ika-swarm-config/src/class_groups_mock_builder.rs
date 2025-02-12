use dwallet_classgroups_types::SingleClassGroupsKeyPairAndPRoof;
use dwallet_classgroups_types::{ClassGroupsKeyPairAndProof, NUM_OF_CLASS_GROUPS_KEYS};

pub fn create_full_class_groups_mock() -> ClassGroupsKeyPairAndProof {
    let mut decryption_keys = Vec::new();
    let mut encryption_keys_and_proofs = Vec::new();
    for i in 0..NUM_OF_CLASS_GROUPS_KEYS {
        let file_name = format!("class-groups-keys-mock-files/class-groups-mock-key-{}", i);
        let bytes = std::fs::read(file_name).unwrap();
        let keypair: SingleClassGroupsKeyPairAndPRoof = bcs::from_bytes(&bytes).unwrap();
        decryption_keys.push(keypair.decryption_key);
        encryption_keys_and_proofs.push(keypair.encryption_key_and_proof);
    }

    ClassGroupsKeyPairAndProof::new(
        decryption_keys.try_into().unwrap(),
        encryption_keys_and_proofs.try_into().unwrap(),
    )
}
