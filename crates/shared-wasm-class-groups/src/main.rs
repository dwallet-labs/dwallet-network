use std::collections::HashMap;
use base64::Engine;
use group::secp256k1;
use mpc::WeightedThresholdAccessStructure;
use dwallet_classgroups_types::ClassGroupsKeyPairAndProof;
use dwallet_mpc_types::dwallet_mpc::ClassGroupsPublicKeyAndProofBytes;
use shared_wasm_class_groups::network_dkg_final_output;

fn main() {
    let public_output = network_dkg_final_output();
    let class_groups_key_path = "/Users/yaelabergel/projects/dwallet_labs/dwallet-network-original/class-groups-keys-mock-files/class-groups-mock-key-full";
    let class_groups_key_base64 = std::fs::read(class_groups_key_path).unwrap();
    let class_groups_key = base64::engine::general_purpose::STANDARD
        .decode(class_groups_key_base64)
        .unwrap();
    let class_groups_key = bcs::from_bytes::<ClassGroupsKeyPairAndProof>(
        &class_groups_key,
    ).unwrap();

    let current_party_to_weight = HashMap::from([(1, 1), (2, 1), (3, 1), (4, 1)]);

    let current_access_structure =
        WeightedThresholdAccessStructure::new(3, current_party_to_weight).unwrap();

    let mut decryption_shares = HashMap::new();
    for i in 1..=4 {
        let decryption_share = public_output.decrypt_decryption_key_shares::<secp256k1::GroupElement>(i, &current_access_structure, class_groups_key.decryption_key()).unwrap();
        decryption_shares.insert(i, decryption_share);
    }

    let decryption_shares_base64 = base64::engine::general_purpose::STANDARD
        .encode(bcs::to_bytes(&decryption_shares).unwrap());

    println!("Decryption shares base64: {}", decryption_shares_base64);

}