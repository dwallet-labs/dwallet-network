use fastcrypto::encoding::{Base64, Encoding};
use dwallet_classgroups_types::mock_class_groups::CGKeyPairAndProofForMockFromFile;
use ika_types::dwallet_mpc_error::DwalletMPCError;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

fn main () {
    // This is a placeholder function that does nothing.
    // It is used to make the code compile.
    // It will be replaced with the actual code during the comparison.
    println!("Hello, world!");

    let contents = std::fs::read_to_string("class-groups-mock-key")
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string())).unwrap();
    let decoded = Base64::decode(contents.as_str())
        .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string())).unwrap();
    let keypair: CGKeyPairAndProofForMockFromFile = bcs::from_bytes(&decoded).unwrap();
    let bytes = bcs::to_bytes(&keypair.encryption_key_and_proof).unwrap();


    let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
    let _ = e.write(&bytes);
    let compressed_bytes = e.finish().unwrap();


    println!("{:?}", compressed_bytes);
}