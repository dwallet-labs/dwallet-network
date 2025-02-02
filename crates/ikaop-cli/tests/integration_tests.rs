// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use std::process::Command;
use ikaoplib::cli::service::init;
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
fn test_initialize_service_ika() -> Result<()> {
    // create a temp dir to work in
    let temp_dir = tempfile::tempdir().expect("creating temp dir");
    let svc_dir = temp_dir.path().join("ika/crates/svc/");
    std::fs::create_dir_all(&svc_dir).context("creating nested dir")?;
    debug!("svc_dir: {:?}", svc_dir);
    // Create a dummy Cargo.toml file at the tempdir/ika level
    let workspace_toml_path = temp_dir.path().join("ika/Cargo.toml");
    std::fs::write(
        workspace_toml_path,
        r#"
[workspace]
members = []
  "#,
    )?;
    // Create a dummy Dockerfile at the tempdir/ika/docker/ika-services level
    let ika_services_dockerfile_path = temp_dir.path().join("ika/docker/ika-services/Dockerfile");
    std::fs::create_dir_all(ika_services_dockerfile_path.parent().unwrap())?;
    std::fs::write(
        &ika_services_dockerfile_path,
        r#"RUN cargo build --release \"#,
    )?;

    // Run the command to initialize a new service
    init::bootstrap_service(&init::ServiceLanguage::Rust, &svc_dir).context("bootstrapping")?;

    // Since we can't run `cargo build` in the new directory as it's not
    // actually in the Ika repo, we'll check that the Cargo.toml file was
    // created and make sure it got the right contents.
    assert!(svc_dir.join("Cargo.toml").exists());
    // Output Cargo.toml contents
    let toml_content = std::fs::read_to_string(svc_dir.join("Cargo.toml"))?;
    // Boilerplate Cargo.toml contents
    let cargo_ika_toml_content =
        std::fs::read_to_string("../mysten-service-boilerplate/Cargo-ika.toml")
            .context("reading cargo toml from boilerplate")?;

    assert_eq!(
        toml_content,
        // replace the service name, everything else should be the same
        cargo_ika_toml_content.replace("service-boilerplate", "svc")
    );
    Ok(())
}
