use class_groups_constants::network_dkg_final_output;

fn main() {
    let network_dkg_output = network_dkg_final_output();
    network_dkg_output.default_encryption_scheme_public_parameters();
    println!("Hello, world!");
}