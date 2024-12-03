// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::anyhow;
use bip32::{ChildNumber, DerivationPath, XPrv};

use crate::keypair_file::read_authority_keypair_from_file;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::secp256r1::{Secp256r1KeyPair, Secp256r1PrivateKey};
use fastcrypto::{
    ed25519::Ed25519PrivateKey,
    secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey},
    traits::{KeyPair, ToFromBytes},
};
use pera_mpc_types::{
    generate_class_groups_keypair_and_proof_from_seed, ClassGroupsKeyPairAndProof,
};
use pera_types::{
    base_types::PeraAddress,
    crypto::{PeraKeyPair, SignatureScheme},
    error::PeraError,
};
use slip10_ed25519::derive_ed25519_private_key;

pub const DERIVATION_PATH_COIN_TYPE: u32 = 784;
pub const DERVIATION_PATH_PURPOSE_ED25519: u32 = 44;
pub const DERVIATION_PATH_PURPOSE_SECP256K1: u32 = 54;
pub const DERVIATION_PATH_PURPOSE_SECP256R1: u32 = 74;

/// Ed25519 follows SLIP-0010 using hardened path: m/44'/784'/0'/0'/{index}'
/// Secp256k1 follows BIP-32/44 using path where the first 3 levels are hardened: m/54'/784'/0'/0/{index}
/// Secp256r1 follows BIP-32/44 using path where the first 3 levels are hardened: m/74'/784'/0'/0/{index}
/// Note that the purpose node is used to distinguish signature schemes.
pub fn derive_key_pair_from_path(
    seed: &[u8],
    derivation_path: Option<DerivationPath>,
    key_scheme: &SignatureScheme,
) -> Result<(PeraAddress, PeraKeyPair), PeraError> {
    let path = validate_path(key_scheme, derivation_path)?;
    match key_scheme {
        SignatureScheme::ED25519 => {
            let indexes = path.into_iter().map(|i| i.into()).collect::<Vec<_>>();
            let derived = derive_ed25519_private_key(seed, &indexes);
            let sk = Ed25519PrivateKey::from_bytes(&derived)
                .map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?;
            let kp: Ed25519KeyPair = sk.into();
            Ok((kp.public().into(), PeraKeyPair::Ed25519(kp)))
        }
        SignatureScheme::Secp256k1 => {
            let child_xprv = XPrv::derive_from_path(seed, &path)
                .map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?;
            let kp = Secp256k1KeyPair::from(
                Secp256k1PrivateKey::from_bytes(child_xprv.private_key().to_bytes().as_slice())
                    .map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?,
            );
            Ok((kp.public().into(), PeraKeyPair::Secp256k1(kp)))
        }
        SignatureScheme::Secp256r1 => {
            let child_xprv = XPrv::derive_from_path(seed, &path)
                .map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?;
            let kp = Secp256r1KeyPair::from(
                Secp256r1PrivateKey::from_bytes(child_xprv.private_key().to_bytes().as_slice())
                    .map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?,
            );
            Ok((kp.public().into(), PeraKeyPair::Secp256r1(kp)))
        }
        SignatureScheme::ClassGroups
        | SignatureScheme::BLS12381
        | SignatureScheme::MultiSig
        | SignatureScheme::ZkLoginAuthenticator
        | SignatureScheme::PasskeyAuthenticator => Err(PeraError::UnsupportedFeatureError {
            error: format!("key derivation not supported {:?}", key_scheme),
        }),
    }
}

pub fn validate_path(
    key_scheme: &SignatureScheme,
    path: Option<DerivationPath>,
) -> Result<DerivationPath, PeraError> {
    match key_scheme {
        SignatureScheme::ED25519 => {
            match path {
                Some(p) => {
                    // The derivation path must be hardened at all levels with purpose = 44, coin_type = 784
                    if let &[purpose, coin_type, account, change, address] = p.as_ref() {
                        if Some(purpose)
                            == ChildNumber::new(DERVIATION_PATH_PURPOSE_ED25519, true).ok()
                            && Some(coin_type)
                                == ChildNumber::new(DERIVATION_PATH_COIN_TYPE, true).ok()
                            && account.is_hardened()
                            && change.is_hardened()
                            && address.is_hardened()
                        {
                            Ok(p)
                        } else {
                            Err(PeraError::SignatureKeyGenError("Invalid path".to_string()))
                        }
                    } else {
                        Err(PeraError::SignatureKeyGenError("Invalid path".to_string()))
                    }
                }
                None => Ok(format!(
                    "m/{DERVIATION_PATH_PURPOSE_ED25519}'/{DERIVATION_PATH_COIN_TYPE}'/0'/0'/0'"
                )
                .parse()
                .map_err(|_| PeraError::SignatureKeyGenError("Cannot parse path".to_string()))?),
            }
        }
        SignatureScheme::Secp256k1 => {
            match path {
                Some(p) => {
                    // The derivation path must be hardened at first 3 levels with purpose = 54, coin_type = 784
                    if let &[purpose, coin_type, account, change, address] = p.as_ref() {
                        if Some(purpose)
                            == ChildNumber::new(DERVIATION_PATH_PURPOSE_SECP256K1, true).ok()
                            && Some(coin_type)
                                == ChildNumber::new(DERIVATION_PATH_COIN_TYPE, true).ok()
                            && account.is_hardened()
                            && !change.is_hardened()
                            && !address.is_hardened()
                        {
                            Ok(p)
                        } else {
                            Err(PeraError::SignatureKeyGenError("Invalid path".to_string()))
                        }
                    } else {
                        Err(PeraError::SignatureKeyGenError("Invalid path".to_string()))
                    }
                }
                None => Ok(format!(
                    "m/{DERVIATION_PATH_PURPOSE_SECP256K1}'/{DERIVATION_PATH_COIN_TYPE}'/0'/0/0"
                )
                .parse()
                .map_err(|_| PeraError::SignatureKeyGenError("Cannot parse path".to_string()))?),
            }
        }
        SignatureScheme::Secp256r1 => {
            match path {
                Some(p) => {
                    // The derivation path must be hardened at first 3 levels with purpose = 74, coin_type = 784
                    if let &[purpose, coin_type, account, change, address] = p.as_ref() {
                        if Some(purpose)
                            == ChildNumber::new(DERVIATION_PATH_PURPOSE_SECP256R1, true).ok()
                            && Some(coin_type)
                                == ChildNumber::new(DERIVATION_PATH_COIN_TYPE, true).ok()
                            && account.is_hardened()
                            && !change.is_hardened()
                            && !address.is_hardened()
                        {
                            Ok(p)
                        } else {
                            Err(PeraError::SignatureKeyGenError("Invalid path".to_string()))
                        }
                    } else {
                        Err(PeraError::SignatureKeyGenError("Invalid path".to_string()))
                    }
                }
                None => Ok(format!(
                    "m/{DERVIATION_PATH_PURPOSE_SECP256R1}'/{DERIVATION_PATH_COIN_TYPE}'/0'/0/0"
                )
                .parse()
                .map_err(|_| PeraError::SignatureKeyGenError("Cannot parse path".to_string()))?),
            }
        }
        SignatureScheme::ClassGroups
        | SignatureScheme::BLS12381
        | SignatureScheme::MultiSig
        | SignatureScheme::ZkLoginAuthenticator
        | SignatureScheme::PasskeyAuthenticator => Err(PeraError::UnsupportedFeatureError {
            error: format!("key derivation not supported {:?}", key_scheme),
        }),
    }
}

pub fn generate_new_key(
    key_scheme: SignatureScheme,
    derivation_path: Option<DerivationPath>,
    word_length: Option<String>,
) -> Result<(PeraAddress, PeraKeyPair, SignatureScheme, String), anyhow::Error> {
    let mnemonic = Mnemonic::new(parse_word_length(word_length)?, Language::English);
    let seed = Seed::new(&mnemonic, "");
    match derive_key_pair_from_path(seed.as_bytes(), derivation_path, &key_scheme) {
        Ok((address, kp)) => Ok((address, kp, key_scheme, mnemonic.phrase().to_string())),
        Err(e) => Err(anyhow!("Failed to generate keypair: {:?}", e)),
    }
}

pub fn generate_new_class_groups_keypair_and_proof(
    path: Option<String>,
) -> Result<(PeraAddress, ClassGroupsKeyPairAndProof), anyhow::Error> {
    let path = path.ok_or_else(|| anyhow!("Path to keypair file not provided"))?;
    let bls12381 = read_authority_keypair_from_file(path)
        .map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?;
    let class_groups_seed = bls12381.copy().private().as_bytes().try_into()?;
    let keypair_and_proof = generate_class_groups_keypair_and_proof_from_seed(class_groups_seed);
    // Todo (#369): let (decryption_key, proof, encryption_key) = class_groups::dkg::proof_helpers::generate_secret_share_sized_keypair_and_proof(&mut class_groups_seed).map_err(|e| PeraError::SignatureKeyGenError(e.to_string()))?;
    Ok((bls12381.public().into(), keypair_and_proof))
}

fn parse_word_length(s: Option<String>) -> Result<MnemonicType, anyhow::Error> {
    match s {
        None => Ok(MnemonicType::Words12),
        Some(s) => match s.as_str() {
            "word12" => Ok(MnemonicType::Words12),
            "word15" => Ok(MnemonicType::Words15),
            "word18" => Ok(MnemonicType::Words18),
            "word21" => Ok(MnemonicType::Words21),
            "word24" => Ok(MnemonicType::Words24),
            _ => anyhow::bail!("Invalid word length"),
        },
    }
}
