use dwallet_mpc_types::dwallet_mpc::NetworkDecryptionKeyOnChainOutput;
use group::{secp256k1, PartyID};
use mpc::{Weight, WeightedThresholdAccessStructure};
use shared_wasm_class_groups::network_dkg_final_output;
use std::collections::HashMap;
use class_groups::SecretKeyShareSizedInteger;

// This test is used to generate the mock on-chain output for the network DKG.
// For internal use.
#[test]
fn build_network_dkg_on_chain_output_test() {
    let network_decryption_key_public_output = network_dkg_final_output();
    let access_structure: HashMap<PartyID, Weight> =
        [(1, 1), (2, 1), (3, 1), (4, 1)].iter().cloned().collect();
    let access_structure = WeightedThresholdAccessStructure::new(3, access_structure).unwrap();

    let decryption_key_share_public_parameters = network_decryption_key_public_output
        .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
            &access_structure,
        )
        .unwrap();
    let decryption_key_share_public_parameters_base64 =
        base64::encode(bcs::to_bytes(&decryption_key_share_public_parameters).unwrap());
    let decryption_key_share_public_parameters_path =
        "decryption_key_share_public_parameters.txt";
    std::fs::write(decryption_key_share_public_parameters_path, decryption_key_share_public_parameters_base64)
        .unwrap();

    let on_chain_output = NetworkDecryptionKeyOnChainOutput {
        encryption_key: bcs::to_bytes(&network_decryption_key_public_output.encryption_key)
            .unwrap(),
        decryption_key_share_public_parameters: bcs::to_bytes(
            &network_decryption_key_public_output
                .default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(
                    &access_structure,
                )
                .unwrap(),
        )
        .unwrap(),
        encryption_scheme_public_parameters: bcs::to_bytes(
            &network_decryption_key_public_output
                .default_encryption_scheme_public_parameters::<secp256k1::GroupElement>()
                .unwrap(),
        )
        .unwrap(),
        public_verification_keys: bcs::to_bytes(
            &network_decryption_key_public_output.public_verification_keys,
        )
        .unwrap(),
        setup_parameters_per_crt_prime: bcs::to_bytes(
            &network_decryption_key_public_output.setup_parameters_per_crt_prime,
        )
        .unwrap(),
    };

    let bcs_on_chain_output = bcs::to_bytes(&on_chain_output).unwrap();
    let base64_on_chain_output = base64::encode(&bcs_on_chain_output);
    std::fs::write(
        "network_dkg_on_chain_output.txt",
        base64_on_chain_output.clone(),
    );
    println!("{}", base64_on_chain_output);
}

#[test]
fn create_shares() {
    let mut shares: HashMap<PartyID, HashMap<PartyID, SecretKeyShareSizedInteger>> =
        HashMap::new();
    for i in 1..=4 {
        let private_output_path = format!("/Users/yaelabergel/projects/dwallet_labs/dwallet-network-original/bas64_private_output_{}.txt", i);
        let private_output = std::fs::read_to_string(private_output_path).unwrap();
        let private_output = base64::decode(private_output).unwrap();
        let private_output: HashMap<PartyID, SecretKeyShareSizedInteger> =
            bcs::from_bytes(&private_output).unwrap();
        shares.insert(i, private_output);
    }
    let shares = bcs::to_bytes(&shares).unwrap();
    let base64_shares = base64::encode(&shares);
    let shares_path = "decryption_key_shares.txt";
    std::fs::write(shares_path, base64_shares).unwrap();
}