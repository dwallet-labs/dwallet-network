use std::collections::HashMap;
use group::{secp256k1, PartyID};
use mpc::{Weight, WeightedThresholdAccessStructure};
use class_groups_constants::network_dkg_final_output;

fn main () {
    let public_output = network_dkg_final_output();

    // encrypted shares
    // let shares = public_output.encryptions_of_shares_per_crt_prime;
    // let shares_size_bytes = bcs::to_bytes(&shares).unwrap().len();
    // println!("Shares size in bytes: {}", shares_size_bytes);

    let weighted_parties: HashMap<PartyID, Weight> = [(1 as PartyID, 1), (2 as PartyID, 1), (3 as PartyID, 1), (4 as PartyID, 1)].iter().cloned().collect();
    let access_structure = WeightedThresholdAccessStructure::new(3 as PartyID, weighted_parties).unwrap();

    let dpp = public_output.default_decryption_key_share_public_parameters::<secp256k1::GroupElement>(&access_structure).unwrap();

    let bcs_dpp = bcs::to_bytes(&dpp).unwrap();
    let base64_dpp = base64::encode(&bcs_dpp);
    println!("Decryption key share public parameters: {}", base64_dpp);

    let pk = read_class_groups_from_file("class-groups.key").unwrap();

    let mut decryption_shares = HashMap::new();
    for i in 1..=4 {
        let decryption_key_share = public_output.default_decryption_key_shares::<secp256k1::GroupElement>(i as PartyID, pk);
        let bcs_dks = bcs::to_bytes(&decryption_key_share).unwrap();
        decryption_shares.insert(i, bcs_dks);
    }

    let bcs_decryption_shares = bcs::to_bytes(&decryption_shares).unwrap();
    let base64_decryption_shares = base64::encode(&bcs_decryption_shares);

    // pure shares
    println!("Decryption shares: {}", base64_decryption_shares);
}