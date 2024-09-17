// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::{Context, Result};
use peraoplib::cli::service::init;
use std::process::Command;
use tracing::debug;

#[cfg(test)]
#[test]
fn test_initialize_service_ext() -> Result<()> {
    // create a temp dir to work in

    let temp_dir = tempfile::tempdir().expect("creating temp dir");
    let svc_dir = temp_dir.path().join("svc");
    std::fs::create_dir(&svc_dir)?;

    // Run the command to initialize a new service
    init::bootstrap_service(&init::ServiceLanguage::Rust, &svc_dir)?;
    // Check that the Cargo.toml file was created
    assert!(svc_dir.join("Cargo.toml").exists());

    // Check that we can run `cargo build` in the new directory
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(svc_dir)
        .output()?;

    println!("cargo build output: {:?}", output);
    assert!(output.status.success());
    Ok(())
}

#[cfg(test)]
#[test]
fn test_initialize_service_pera() -> Result<()> {
    // create a temp dir to work in
    let temp_dir = tempfile::tempdir().expect("creating temp dir");
    let svc_dir = temp_dir.path().join("pera/crates/svc/");
    std::fs::create_dir_all(&svc_dir).context("creating nested dir")?;
    debug!("svc_dir: {:?}", svc_dir);
    // Create a dummy Cargo.toml file at the tempdir/pera level
    let workspace_toml_path = temp_dir.path().join("pera/Cargo.toml");
    std::fs::write(
        workspace_toml_path,
        r#"
[workspace]
members = []
  "#,
    )?;
    // Create a dummy Dockerfile at the tempdir/pera/docker/pera-services level
    let pera_services_dockerfile_path =
        temp_dir.path().join("pera/docker/pera-services/Dockerfile");
    std::fs::create_dir_all(pera_services_dockerfile_path.parent().unwrap())?;
    std::fs::write(
        &pera_services_dockerfile_path,
        r#"RUN cargo build --release \"#,
    )?;

    // Run the command to initialize a new service
    init::bootstrap_service(&init::ServiceLanguage::Rust, &svc_dir).context("bootstrapping")?;

    // Since we can't run `cargo build` in the new directory as it's not
    // actually in the Pera repo, we'll check that the Cargo.toml file was
    // created and make sure it got the right contents.
    assert!(svc_dir.join("Cargo.toml").exists());
    // Output Cargo.toml contents
    let toml_content = std::fs::read_to_string(svc_dir.join("Cargo.toml"))?;
    // Boilerplate Cargo.toml contents
    let cargo_pera_toml_content =
        std::fs::read_to_string("../mysten-service-boilerplate/Cargo-pera.toml")
            .context("reading cargo toml from boilerplate")?;

    assert_eq!(
        toml_content,
        // replace the service name, everything else should be the same
        cargo_pera_toml_content.replace("service-boilerplate", "svc")
    );
    Ok(())
}
