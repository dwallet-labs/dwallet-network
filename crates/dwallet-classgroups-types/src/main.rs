// // use fastcrypto::encoding::{Base64, Encoding};
// use dwallet_classgroups_types::mock_class_groups::CGKeyPairAndProofForMockFromFile;
// use fastcrypto::encoding::Encoding;
// use ika_types::dwallet_mpc_error::DwalletMPCError;
// use std::io::prelude::*;
//
// // fn main () {
// //     // This is a placeholder function that does nothing.
// //     // It is used to make the code compile.
// //     // It will be replaced with the actual code during the comparison.
// //     println!("Hello, world!");
// //
// //     std::thread::spawn(|| {
// //         let contents = std::fs::read_to_string("class-groups-mock-key")
// //             .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string())).unwrap();
// //         let decoded = Base64::decode(contents.as_str())
// //             .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string())).unwrap();
// //         let keypair: CGKeyPairAndProofForMockFromFile = bcs::from_bytes(&decoded).unwrap();
// //         let bytes = bcs::to_bytes(&keypair.encryption_key_and_proof).unwrap();
// //     }).join().unwrap();
// //
// //
// //     // let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
// //     // let _ = e.write(&bytes);
// //     // let compressed_bytes = e.finish().unwrap();
// //
// //
// //     // println!("{:?}", compressed_bytes);
// // }
//
// use tokio::runtime::Builder;
// // use fastcrypto::encoding::{Base64, Encoding};
// use dwallet_classgroups_types::SingleClassGroupsKeyPairAndPRoof;
//
// fn main() {
//     let runtime = Builder::new_multi_thread()
//         // .worker_threads(4) // Adjust number of worker threads
//         .thread_stack_size(2 * 1024 * 1024) // 32MB stack per thread
//         .enable_all() // Enables all Tokio features
//         .build()
//         .expect("Failed to build Tokio runtime");
//
//     runtime.block_on(async {
//         let contents = std::fs::read_to_string("class-groups-mock-key")
//             .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))
//             .unwrap();
//         let decoded = fastcrypto::encoding::Base64::decode(contents.as_str())
//             .map_err(|e| DwalletMPCError::FailedToReadCGKey(e.to_string()))
//             .unwrap();
//         let keypair: CGKeyPairAndProofForMockFromFile = bcs::from_bytes(&decoded).unwrap();
//
//         // zip and map keypair.encryption_key_and_proof and keypair.decryption_key
//         let mut i = 0;
//         let ress = keypair
//             .encryption_key_and_proof
//             .into_iter()
//             .zip(keypair.decryption_key.into_iter())
//             .map(|(enc_key, dec_key)| {
//                 let pair = SingleClassGroupsKeyPairAndPRoof {
//                     encryption_key_and_proof: enc_key,
//                     decryption_key: dec_key,
//                 };
//                 let puf = bcs::to_bytes(&pair).unwrap();
//                 // write bytes into file "class-groups-mock-key-{i}"
//                 let mut file =
//                     std::fs::File::create(format!("class-groups-mock-key-{}", i)).unwrap();
//                 file.write_all(&puf).unwrap();
//                 //close file
//                 drop(file);
//                 i += 1;
//                 i
//             })
//             .collect::<Vec<u8>>();
//
//         println!("Successfully deserialized keypair");
//     });
// }
