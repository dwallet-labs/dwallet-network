use base64::{engine::general_purpose::STANDARD, Engine as _};
/// This module contains the secp256k1 constants for the class groups protocol.
/// NOTE: This is a temporary solution until the class groups encryption key DKG is complete.
/// Todo (#312): Remove this module and use the class groups DKG to generate the constants.
use group::secp256k1;

const PROTOCOL_PUBLIC_PARAMETERS: &str = "OlRoZSBmaW5pdGUgZmllbGQgb2YgaW50ZWdlcnMgbW9kdWxvIHByaW1lIHEgJFxtYXRoYmJ7Wn1fcSRBQTbQjF7SvzugSK/m3K66/v///////////////////wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABCVNlY3AyNTZrMQtXZWllcnN0cmFzc0FBNtCMXtK/O6BIr+bcrrr+////////////////////L/z///7///////////////////////////////////8hAnm+Zn753LusVaBilc6HCwcCm/zbLc4o2VnygVsW+BeYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADpUaGUgZmluaXRlIGZpZWxkIG9mIGludGVnZXJzIG1vZHVsbyBwcmltZSBxICRcbWF0aGJie1p9X3EkQUE20Ixe0r87oEiv5tyuuv7///////////////////8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAIAAAAAAAA6gIAAAAAAABrBAAAAAAAAKPCxpryAt2blv0kjOMQ8KtM1CvLnGhn5QAokwAfM4LKbDMorUV9uEXsni2yXCL6rjJOxx54wJPHiAjl1vOc7Z0BNP/kuGFFccP5jGUq+5VTr0Ia7Nq4KNDq/eJO7BzWZsvU696L92oURWW+9Ppee/cYFKIfRVjHSzNw2HN+8nkDvtBTkHs8LoAH6BwXHU6+D8Q2uoGQ6SJ0xgGTdZO2a49ismkYChbD9gjeE4O+gsJVGrfp0gTOC0u2jVQVSXBvQmxp63dZinvKvAeRIQItEYIzjrubqEvp61wEM1jAvcy9b116h7UVWJILAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAEeod44JLj5n2yjyGpo3b9UJGBmVnaIEvdrw6GDqKg9Bi1wrZFX2Hf5IXFR0YLind3/rN6c4x9aES+oxw6RhwhPgZvtSQrLhmhBi3XnqD9rQvBMEMXQsCSft1efoPOYPF1cuOzzgfVbVfwIjePYNrp4RYvvwVcBAAAA/x577htpQheFeKY7EeFOlb5PmieYc1MX1aqy+B+ygo68ewg3kGZF53kfJymJCGSr09lh898rcYwSf09P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";

pub fn protocol_public_parameters() -> twopc_mpc::class_groups::ProtocolPublicParameters<
    { secp256k1::SCALAR_LIMBS },
    { twopc_mpc::secp256k1::class_groups::DISCRIMINANT_LIMBS },
    secp256k1::GroupElement,
> {
    // Safe to unwrap as we're using a hardcoded constant.
    let protocol_public_parameters = STANDARD.decode(&PROTOCOL_PUBLIC_PARAMETERS).unwrap();
    bcs::from_bytes(&protocol_public_parameters).unwrap()
}
