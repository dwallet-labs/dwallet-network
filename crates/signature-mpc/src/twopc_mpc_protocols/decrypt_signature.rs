use homomorphic_encryption::AdditivelyHomomorphicDecryptionKeyShare;
use tiresias::{
    DecryptionKeyShare, EncryptionKey
    ,
};
use twopc_mpc::paillier::PLAINTEXT_SPACE_SCALAR_LIMBS;

pub type PartialDecryptionProof = <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<
    PLAINTEXT_SPACE_SCALAR_LIMBS,
    EncryptionKey,
>>::PartialDecryptionProof;

pub type DecryptionShare = <DecryptionKeyShare as AdditivelyHomomorphicDecryptionKeyShare<
    PLAINTEXT_SPACE_SCALAR_LIMBS,
    EncryptionKey,
>>::DecryptionShare;
