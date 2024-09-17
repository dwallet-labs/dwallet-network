// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
use tempfile::TempDir;

use pera_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use pera_types::crypto::Ed25519PeraSignature;
use pera_types::crypto::{SignatureScheme, PeraSignatureInner};
#[test]
fn mnemonic_test() {
    let temp_dir = TempDir::new().unwrap();
    let keystore_path = temp_dir.path().join("pera.keystore");
    let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    let (address, phrase, scheme) = keystore
        .generate_and_add_new_key(SignatureScheme::ED25519, None, None, None)
        .unwrap();

    let keystore_path_2 = temp_dir.path().join("pera2.keystore");
    let mut keystore2 = Keystore::from(FileBasedKeystore::new(&keystore_path_2).unwrap());
    let imported_address = keystore2
        .import_from_mnemonic(&phrase, SignatureScheme::ED25519, None, None)
        .unwrap();
    assert_eq!(scheme.flag(), Ed25519PeraSignature::SCHEME.flag());
    assert_eq!(address, imported_address);
}

#[test]
fn keystore_display_test() -> Result<(), anyhow::Error> {
    let temp_dir = TempDir::new().unwrap();
    let keystore_path = temp_dir.path().join("pera.keystore");
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path).unwrap());
    assert!(keystore.to_string().contains("pera.keystore"));
    assert!(!keystore.to_string().contains("keys:"));
    Ok(())
}
