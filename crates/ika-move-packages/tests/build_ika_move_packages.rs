// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::Result;
use move_binary_format::{file_format::Visibility, CompiledModule};
use move_compiler::editions::Edition;
use move_package::{BuildConfig as MoveBuildConfig, LintFlag};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};
use sui_move_build::{BuildConfig, SuiPackageHooks};

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const COMPILED_PACKAGES_DIR: &str = "packages_compiled";
const DOCS_DIR: &str = "docs";
const PUBLISHED_API_FILE: &str = "published_api.txt";

#[test]
fn build_ika_move_packages() {
    move_package::package_hooks::register_package_hooks(Box::new(SuiPackageHooks));
    let tempdir = tempfile::tempdir().unwrap();
    let out_dir = if std::env::var_os("UPDATE").is_some() {
        let crate_root = Path::new(CRATE_ROOT);
        let _ = std::fs::remove_dir_all(crate_root.join(COMPILED_PACKAGES_DIR));
        let _ = std::fs::remove_dir_all(crate_root.join(DOCS_DIR));
        let _ = std::fs::remove_file(crate_root.join(PUBLISHED_API_FILE));
        crate_root
    } else {
        tempdir.path()
    };

    std::fs::create_dir_all(out_dir.join(COMPILED_PACKAGES_DIR)).unwrap();
    std::fs::create_dir_all(out_dir.join(DOCS_DIR)).unwrap();

    let packages_path = Path::new(CRATE_ROOT).join("packages");

    let ika_path = packages_path.join("ika");
    let ika_system_path = packages_path.join("ika_system");

    build_packages(&ika_path, &ika_system_path, out_dir);

    check_diff(Path::new(CRATE_ROOT), out_dir)
}

// Verify that checked-in values are the same as the generated ones
fn check_diff(checked_in: &Path, built: &Path) {
    for path in [COMPILED_PACKAGES_DIR, DOCS_DIR, PUBLISHED_API_FILE] {
        let output = std::process::Command::new("diff")
            .args(["--brief", "--recursive"])
            .arg(checked_in.join(path))
            .arg(built.join(path))
            .output()
            .unwrap();
        if !output.status.success() {
            let header =
                "Generated and checked-in ika-move-packages packages and/or docs do not match.\n\
                 Re-run with `UPDATE=1` to update checked-in packages and docs. e.g.\n\n\
                 UPDATE=1 cargo test -p ika-move-packages --test build_ika_move_packages";

            panic!(
                "{header}\n\n{}\n\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

fn build_packages(ika_path: &Path, ika_system_path: &Path, out_dir: &Path) {
    let config = MoveBuildConfig {
        generate_docs: true,
        warnings_are_errors: true,
        install_dir: Some(PathBuf::from(".")),
        lint_flag: LintFlag::LEVEL_NONE,
        default_edition: Some(Edition::E2024_BETA),
        ..Default::default()
    };
    debug_assert!(!config.test_mode);
    build_packages_with_move_config(
        ika_path,
        ika_system_path,
        out_dir,
        "ika",
        "ika_system",
        config,
    );
}

fn build_packages_with_move_config(
    ika_path: &Path,
    ika_system_path: &Path,
    out_dir: &Path,
    ika_dir: &str,
    ika_system_dir: &str,
    config: MoveBuildConfig,
) {
    let ika_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        chain_id: None, // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build(ika_path)
    .unwrap();
    let ika_system_pkg = BuildConfig {
        config: config.clone(),
        run_bytecode_verifier: true,
        print_diags_to_stderr: false,
        chain_id: None, // Framework pkg addr is agnostic to chain, resolves from Move.toml
    }
    .build(ika_system_path)
    .unwrap();

    let ika = ika_pkg.get_dependency_sorted_modules(false);
    let ika_system = ika_system_pkg.get_dependency_sorted_modules(false);

    let compiled_packages_dir = out_dir.join(COMPILED_PACKAGES_DIR);

    let ika_members =
        serialize_modules_to_file(ika.iter(), &compiled_packages_dir.join(ika_dir)).unwrap();
    let ika_system_members = serialize_modules_to_file(
        ika_system.iter(),
        &compiled_packages_dir.join(ika_system_dir),
    )
    .unwrap();

    // write out generated docs
    let docs_dir = out_dir.join(DOCS_DIR);
    let mut files_to_write = BTreeMap::new();
    relocate_docs(&ika_pkg.package.compiled_docs.unwrap(), &mut files_to_write);
    relocate_docs(
        &ika_system_pkg.package.compiled_docs.unwrap(),
        &mut files_to_write,
    );
    for (fname, doc) in files_to_write {
        let dst_path = docs_dir.join(fname);
        fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
        fs::write(dst_path, doc).unwrap();
    }

    let published_api = [ika_members.join("\n"), ika_system_members.join("\n")].join("\n");

    fs::write(out_dir.join(PUBLISHED_API_FILE), published_api).unwrap();
}

/// Post process the generated docs so that they are in a format that can be consumed by
/// docusaurus.
/// * Flatten out the tree-like structure of the docs directory that we generate for a package into
///   a flat list of packages;
/// * Deduplicate packages (since multiple packages could share dependencies); and
/// * Write out the package docs in a flat directory structure.
fn relocate_docs(files: &[(String, String)], output: &mut BTreeMap<String, String>) {
    // Turn on multi-line mode so that `.` matches newlines, consume from the start of the file to
    // beginning of the heading, then capture the heading and replace with the yaml tag for docusaurus. E.g.,
    // ```
    // -<a name="0x2_display"></a>
    // -
    // -# Module `0x2::display`
    // -
    // +---
    // +title: Module `0x2::display`
    // +---
    //```
    let re = regex::Regex::new(r"(?s).*\n#\s+(.*?)\n").unwrap();
    for (file_name, file_content) in files {
        if file_name.contains("dependencies") {
            continue;
        };
        output.entry(file_name.to_owned()).or_insert_with(|| {
            re.replace_all(
                &file_content
                    .replace("../../dependencies/", "../")
                    .replace("../dependencies/", "../")
                    .replace("dependencies/", "../"),
                "---\ntitle: $1\n---\n",
            )
            .to_string()
        });
    }
}

fn serialize_modules_to_file<'a>(
    modules: impl Iterator<Item = &'a CompiledModule>,
    file: &Path,
) -> Result<Vec<String>> {
    let mut serialized_modules = Vec::new();
    let mut members = vec![];
    for module in modules {
        let module_name = module.self_id().short_str_lossless();
        for def in module.struct_defs() {
            let sh = module.datatype_handle_at(def.struct_handle);
            let sn = module.identifier_at(sh.name);
            members.push(format!("{sn}\n\tpublic struct\n\t{module_name}"));
        }

        for def in module.enum_defs() {
            let eh = module.datatype_handle_at(def.enum_handle);
            let en = module.identifier_at(eh.name);
            members.push(format!("{en}\n\tpublic enum\n\t{module_name}"));
        }

        for def in module.function_defs() {
            let fh = module.function_handle_at(def.function);
            let fn_ = module.identifier_at(fh.name);
            let viz = match def.visibility {
                Visibility::Public => "public ",
                Visibility::Friend => "public(package) ",
                Visibility::Private => "",
            };
            let entry = if def.is_entry { "entry " } else { "" };
            members.push(format!("{fn_}\n\t{viz}{entry}fun\n\t{module_name}"));
        }

        let mut buf = Vec::new();
        module.serialize_with_version(module.version, &mut buf)?;
        serialized_modules.push(buf);
    }
    assert!(
        !serialized_modules.is_empty(),
        "Failed to find system or framework or stdlib modules"
    );

    let binary = bcs::to_bytes(&serialized_modules)?;

    fs::write(file, binary)?;

    Ok(members)
}
