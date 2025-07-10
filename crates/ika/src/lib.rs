// Copyright (c) dWallet Labs Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

#[macro_use]
pub mod ika_commands;
mod validator_commands;

#[cfg(test)]
mod tests {
    use fastcrypto::hash::{HashFunction, Keccak256};
    use hex;
    use std::io::Write;

    pub fn keccak256_digest(bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::default();
        hasher.write_all(bytes).expect("Hasher should not fail");
        let digest = hasher.finalize();
        digest.into()
    }

    fn session_identifier_digest(preimage: &[u8]) -> [u8; 32] {
        let prefix = [b"SYSTEM", preimage].concat();
        keccak256_digest(&prefix)
    }

    #[test]
    fn test_user_session_identifier_digest() {
        let preimage: [u8; 32] = [
            123, 68, 151, 118, 61, 2, 140, 100, 5, 120, 4, 186, 163, 174, 2, 91, 115, 175, 9, 182,
            37, 116, 244, 245, 146, 124, 14, 20, 198, 212, 93, 109,
        ];

        let digest = session_identifier_digest(&preimage);
        let hex_digest = hex::encode(digest);

        println!("Keccak256(USER + preimage) = 0x{} {:?}", hex_digest, digest);

        // Optional: assert against expected output if known
        // assert_eq!(hex_digest, "expected_hex_string_here");
    }
}
