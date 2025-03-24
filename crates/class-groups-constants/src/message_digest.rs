use twopc_mpc::secp256k1;
use k256::ecdsa::hazmat::bits2field;
use sha3::Digest;
use sha3::digest::FixedOutput;
use k256::elliptic_curve::ops::Reduce;
use k256::{elliptic_curve, U256};
/// Supported hash functions for message digest.
#[derive(Clone, Debug)]
enum Hash {
    KECCAK256 = 0,
    SHA256 = 1,
}

/// Computes the message digest of a given message using the specified hash function.
fn message_digest(message: &[u8], hash_type: &Hash) -> anyhow::Result<secp256k1::Scalar> {
    let hash = match hash_type {
        Hash::KECCAK256 => bits2field::<k256::Secp256k1>(
            &sha3::Keccak256::new_with_prefix(message).finalize_fixed(),
        )
            .map_err(|e| anyhow::Error::msg(format!("KECCAK256 bits2field error: {:?}", e)))?,

        Hash::SHA256 => {
            bits2field::<k256::Secp256k1>(&sha2::Sha256::new_with_prefix(message).finalize_fixed())
                .map_err(|e| anyhow::Error::msg(format!("SHA256 bits2field error: {:?}", e)))?
        }
    };
    #[allow(clippy::useless_conversion)]
    let m = <elliptic_curve::Scalar<k256::Secp256k1> as Reduce<U256>>::reduce_bytes(&hash.into());
    Ok(U256::from(m).into())
}
