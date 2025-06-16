use fastcrypto::bls12381::min_pk;
use fastcrypto::traits::{ToFromBytes, VerifyingKey};

fn main() {
    println!("hello world");
    let signature_bytes_ref = vec![
        138, 212, 22, 186, 216, 121, 103, 47, 226, 10, 24, 149, 103, 127, 245, 58, 58, 40, 41, 61,
        105, 108, 57, 56, 191, 36, 251, 177, 185, 77, 51, 152, 239, 62, 147, 240, 185, 25, 155,
        165, 68, 101, 214, 239, 93, 192, 21, 76, 3, 200, 247, 160, 166, 227, 106, 10, 149, 170,
        135, 128, 42, 83, 172, 89, 253, 133, 217, 46, 177, 27, 93, 195, 84, 246, 22, 81, 76, 74,
        15, 14, 182, 194, 89, 10, 45, 160, 168, 32, 206, 187, 53, 82, 24, 237, 67, 224,
    ];
    let signature =
        match <min_pk::BLS12381Signature as ToFromBytes>::from_bytes(&signature_bytes_ref) {
            Ok(signature) => signature,
            Err(_) => return,
        };

    let public_key =
        match <min_pk::BLS12381PublicKey as ToFromBytes>::from_bytes(&public_key_bytes_ref) {
            Ok(public_key) => match public_key.validate() {
                Ok(_) => public_key,
                Err(_) => return,
            },
            Err(_) => return,
        };

    let result = public_key.verify(&msg_ref, &signature).is_ok();
    if result {
        println!("Signature is valid");
    } else {
        println!("Signature is invalid");
    }
}
