use class_groups_constants::network_dkg_final_output;

fn main() {
    // let a = dwallet_mpc_centralized_party::protocol_public_parameters_by_key_scheme(bcs::to_bytes(&network_dkg_final_output()).unwrap(), 1);
    let b = bcs::to_bytes(&network_dkg_final_output()).unwrap();
    println!("{:?}", b);
}