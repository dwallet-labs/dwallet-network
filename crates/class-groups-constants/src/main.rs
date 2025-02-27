use class_groups_constants::network_dkg_final_output;

fn main () {
    let public_output = network_dkg_final_output();
    let shares = public_output.encryptions_of_shares_per_crt_prime;
    let shares_size_bytes = bcs::to_bytes(&shares).unwrap().len();
    println!("Shares size in bytes: {}", shares_size_bytes);
}