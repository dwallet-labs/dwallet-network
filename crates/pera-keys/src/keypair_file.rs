// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::path::PathBuf;

use anyhow::anyhow;
use fastcrypto::encoding::{Base64, Encoding, Hex};
use fastcrypto::{secp256k1::Secp256k1KeyPair, traits::EncodeDecodeBase64};
use pera_mpc_types::ClassGroupsKeyPairAndProof;
use pera_types::crypto::{AuthorityKeyPair, NetworkKeyPair, PeraKeyPair, ToFromBytes};

pub fn write_class_groups_keypair_and_proof_to_file<P: AsRef<std::path::Path>>(
    keypair: &ClassGroupsKeyPairAndProof,
    path: P,
) -> anyhow::Result<String> {
    let serialized = serde_json::to_vec(&keypair)?;
    let contents = Base64::encode(serialized);
    std::fs::write(path, contents)?;
    Ok(Base64::encode(keypair.public_bytes()))
}

/// Write Base64 encoded `flag || privkey` to file.
pub fn write_keypair_to_file<P: AsRef<std::path::Path>>(
    keypair: &PeraKeyPair,
    path: P,
) -> anyhow::Result<()> {
    let contents = keypair.encode_base64();
    std::fs::write(path, contents)?;
    Ok(())
}

/// Write Base64 encoded `privkey` to file.
pub fn write_authority_keypair_to_file<P: AsRef<std::path::Path>>(
    keypair: &AuthorityKeyPair,
    path: P,
) -> anyhow::Result<()> {
    let contents = keypair.encode_base64();
    std::fs::write(path, contents)?;
    Ok(())
}

pub fn read_class_groups_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> anyhow::Result<ClassGroupsKeyPairAndProof> {
    let contents = std::fs::read_to_string(path)?;
    let decoded = Base64::decode(contents.as_str().trim()).map_err(|e| anyhow!(e))?;
    let keypair: ClassGroupsKeyPairAndProof = serde_json::from_slice(&decoded)?;
    Ok(keypair)
}

/// Read from file as Base64 encoded `privkey` and return a AuthorityKeyPair.
pub fn read_authority_keypair_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> anyhow::Result<AuthorityKeyPair> {
    let contents = std::fs::read_to_string(path)?;
    AuthorityKeyPair::decode_base64(contents.as_str().trim()).map_err(|e| anyhow!(e))
}

/// Read from file as Base64 encoded `flag || privkey` and return a PeraKeypair.
pub fn read_keypair_from_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<PeraKeyPair> {
    let contents = std::fs::read_to_string(path)?;
    PeraKeyPair::decode_base64(contents.as_str().trim()).map_err(|e| anyhow!(e))
}

/// Read from file as Base64 encoded `flag || privkey` and return a NetworkKeyPair.
pub fn read_network_keypair_from_file<P: AsRef<std::path::Path>>(
    path: P,
) -> anyhow::Result<NetworkKeyPair> {
    let kp = read_keypair_from_file(path)?;
    if let PeraKeyPair::Ed25519(kp) = kp {
        Ok(kp)
    } else {
        Err(anyhow!("Invalid scheme for network keypair"))
    }
}

/// Read a PeraKeyPair from a file. The content could be any of the following:
/// - Base64 encoded `flag || privkey` for ECDSA key
/// - Base64 encoded `privkey` for Raw key
/// - Bech32 encoded private key prefixed with `peraprivkey`
/// - Hex encoded `privkey` for Raw key
///
/// If `require_secp256k1` is true, it will return an error if the key is not Secp256k1.
pub fn read_key(path: &PathBuf, require_secp256k1: bool) -> Result<PeraKeyPair, anyhow::Error> {
    if !path.exists() {
        return Err(anyhow::anyhow!("Key file not found at path: {:?}", path));
    }
    let file_contents = std::fs::read_to_string(path)?;
    let contents = file_contents.as_str().trim();

    // Try base64 encoded PeraKeyPair `flag || privkey`
    if let Ok(key) = PeraKeyPair::decode_base64(contents) {
        if require_secp256k1 && !matches!(key, PeraKeyPair::Secp256k1(_)) {
            return Err(anyhow!("Key is not Secp256k1"));
        }
        return Ok(key);
    }

    // Try base64 encoded Raw Secp256k1 key `privkey`
    if let Ok(key) = Secp256k1KeyPair::decode_base64(contents) {
        return Ok(PeraKeyPair::Secp256k1(key));
    }

    // Try Bech32 encoded 33-byte `flag || private key` starting with `peraprivkey`A prefix.
    // This is the format of a private key exported from Pera Wallet or pera.keystore.
    if let Ok(key) = PeraKeyPair::decode(contents) {
        if require_secp256k1 && !matches!(key, PeraKeyPair::Secp256k1(_)) {
            return Err(anyhow!("Key is not Secp256k1"));
        }
        return Ok(key);
    }

    // Try hex encoded Raw key `privkey`
    if let Ok(bytes) = Hex::decode(contents).map_err(|e| anyhow!("Error decoding hex: {:?}", e)) {
        if let Ok(key) = Secp256k1KeyPair::from_bytes(&bytes) {
            return Ok(PeraKeyPair::Secp256k1(key));
        }
    }

    Err(anyhow!("Error decoding key from {:?}", path))
}
