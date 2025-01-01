use class_groups_constants::{decryption_key_share, decryption_key_share_public_parameters};
use group::PartyID;
use mpc::{Weight, WeightedThresholdAccessStructure};
use std::collections::HashMap;

fn main() {
    let weighted_parties: HashMap<PartyID, Weight> =
        [(1, 1), (2, 1), (3, 1), (4, 1)].iter().cloned().collect();

    let quorum_threshold = 3;
    let weighted_threshold_access_structure =
        WeightedThresholdAccessStructure::new(quorum_threshold as PartyID, weighted_parties)
            .unwrap();

    let dpp = base64::encode(&decryption_key_share_public_parameters(
        &weighted_threshold_access_structure,
    ));

    // let ppp = class_groups_constants::network_dkg_final_output();

    // let decryption_shares = [1, 2, 3, 4].iter().enumerate().map(|(party_id, _) | {
    //     let decryption_share = decryption_key_share((party_id + 1) as PartyID, &weighted_threshold_access_structure);
    //     ((party_id + 1) as PartyID, decryption_share)
    // }).collect::<HashMap<PartyID, _>>();

    println!("{:?}", dpp);
}
