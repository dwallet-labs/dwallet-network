use class_groups_constants::network_dkg_final_output;

fn main() {
    // print protocol public parameters base64 encoded
    println!(
        "{}",
        base64::encode(
            bcs::to_bytes(
                &network_dkg_final_output()
                    .default_encryption_scheme_public_parameters::<twopc_mpc::secp256k1::GroupElement>()
                    .unwrap()
            )
            .unwrap()
        )
    );
    println!("Hello, world!");
}
