// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use anyhow::bail;
use include_directory::{include_directory, Dir, DirEntry};
use move_binary_format::file_format::AddressIdentifierIndex;
use move_binary_format::CompiledModule;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::path::Path;
use sui_types::{base_types::ObjectID, MOVE_STDLIB_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID};
use tempfile::TempDir;

/// Represents a system package in the framework, that's built from the source code inside
/// ika-framework.
#[derive(Clone, Serialize, PartialEq, Eq, Deserialize)]
pub struct IkaMovePackage {
    pub name: &'static str,
    pub bytes: Vec<Vec<u8>>,
    pub dependencies: Vec<ObjectID>,
    pub ika_dependencies: Vec<&'static str>,
}

impl IkaMovePackage {
    pub fn new(
        name: &'static str,
        raw_bytes: &'static [u8],
        dependencies: &[ObjectID],
        ika_dependencies: &[&'static str],
    ) -> Self {
        let bytes: Vec<Vec<u8>> = bcs::from_bytes(raw_bytes).unwrap();
        Self {
            name,
            bytes,
            dependencies: dependencies.to_vec(),
            ika_dependencies: ika_dependencies.to_vec(),
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn bytes(&self) -> &[Vec<u8>] {
        &self.bytes
    }

    pub fn dependencies(&self) -> &[ObjectID] {
        &self.dependencies
    }

    pub fn modules_with_deps(
        &self,
        ika_dependencies_map: HashMap<String, ObjectID>,
    ) -> anyhow::Result<Vec<CompiledModule>> {
        let mut ika_dependencies = Vec::new();
        for name in self.ika_dependencies.iter() {
            let Some(id) = ika_dependencies_map.get(*name) else {
                bail!("Missing ika dependency {}", name);
            };
            ika_dependencies.push((name.to_string(), *id));
        }
        let mut modules: Vec<_> = self
            .bytes
            .iter()
            .map(|b| CompiledModule::deserialize_with_defaults(b).unwrap())
            .collect();

        for module in modules.iter_mut() {
            let cloned_module = module.clone();
            let mut address_identifiers_map = HashMap::new();
            for module_handle in module.module_handles.iter_mut() {
                let name = cloned_module.identifier_at(module_handle.name);
                for (n, id) in ika_dependencies.iter() {
                    if name.as_str() == n && !address_identifiers_map.contains_key(n) {
                        address_identifiers_map
                            .insert(n.clone(), cloned_module.address_identifiers.len() as u16);
                        module.address_identifiers.push((*id).into());
                    }
                }
            }
            for module_handle in module.module_handles.iter_mut() {
                let name = cloned_module.identifier_at(module_handle.name);
                for (n, _id) in ika_dependencies.iter() {
                    if name.as_str() == n {
                        module_handle.address =
                            AddressIdentifierIndex(*address_identifiers_map.get(n).unwrap());
                    }
                }
            }
        }
        Ok(modules)
    }

    pub fn bytes_with_deps(
        &self,
        ika_dependencies_map: HashMap<String, ObjectID>,
    ) -> anyhow::Result<Vec<Vec<u8>>> {
        let modules = self.modules_with_deps(ika_dependencies_map)?;
        let mut bytes = Vec::new();
        for module in modules {
            let mut buf = Vec::new();
            module.serialize_with_version(module.version, &mut buf)?;
            bytes.push(buf);
        }
        Ok(bytes)
    }

    pub fn full_deps(
        &self,
        ika_dependencies_map: HashMap<String, ObjectID>,
    ) -> anyhow::Result<Vec<ObjectID>> {
        let mut ika_dependencies = Vec::new();
        ika_dependencies.extend(self.dependencies.iter());
        for name in self.ika_dependencies.iter() {
            let Some(id) = ika_dependencies_map.get(*name) else {
                unreachable!("Missing ika dependency {}", name);
            };
            ika_dependencies.push(*id);
        }

        Ok(ika_dependencies)
    }
}

impl std::fmt::Debug for IkaMovePackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {:?}", self.name)?;
        writeln!(f, "Size: {}", self.bytes.len())?;
        writeln!(f, "Dependencies: {:?}", self.dependencies)?;
        writeln!(f, "Ika Dependencies: {:?}", self.ika_dependencies)?;
        Ok(())
    }
}

macro_rules! define_system_packages {
    ([$(($name:expr, $path:expr, $deps:expr, $ika_deps:expr)),* $(,)?]) => {{
        static PACKAGES: Lazy<Vec<IkaMovePackage>> = Lazy::new(|| {
            vec![
                $(IkaMovePackage::new(
                    $name,
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/packages_compiled", "/", $path)),
                    &$deps,
                    &$ika_deps,
                )),*
            ]
        });
        Lazy::force(&PACKAGES)
    }}
}

pub struct BuiltInIkaMovePackages;

impl BuiltInIkaMovePackages {
    pub fn iter_system_packages() -> impl Iterator<Item = &'static IkaMovePackage> {
        // All system packages in the current build should be registered here, and this is the only
        // place we need to worry about if any of them changes.
        // TODO: Is it possible to derive dependencies from the bytecode instead of manually specifying them?
        define_system_packages!([
            (
                "ika",
                "ika",
                [MOVE_STDLIB_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID],
                []
            ),
            (
                "ika_common",
                "ika_common",
                [MOVE_STDLIB_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID],
                []
            ),
            (
                "ika_system",
                "ika_system",
                [MOVE_STDLIB_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID],
                ["ika", "ika_common"]
            ),
            (
                "ika_dwallet_2pc_mpc",
                "ika_dwallet_2pc_mpc",
                [MOVE_STDLIB_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID],
                ["ika", "ika_common", "ika_system"]
            ),
        ])
        .iter()
    }

    pub fn all_package_names() -> Vec<String> {
        Self::iter_system_packages()
            .map(|p| p.name.to_string())
            .collect()
    }

    pub fn get_package_by_name(name: &str) -> &'static IkaMovePackage {
        Self::iter_system_packages()
            .find(|s| s.name == name)
            .unwrap()
    }
}

pub const DEFAULT_IKA_MOVE_PACKAGES_PATH: &str = env!("CARGO_MANIFEST_DIR");

static CONTRACTS_DIR: Dir<'_> = include_directory!("$CARGO_MANIFEST_DIR/packages");

pub fn save_contracts_to_temp_dir() -> anyhow::Result<TempDir> {
    let temp_dir =
        tempfile::tempdir().map_err(|e| anyhow::anyhow!("Failed to create temp dir: {}", e))?;
    let path = temp_dir.path();
    save_dir_entries(path, CONTRACTS_DIR.entries())?;
    Ok(temp_dir)
}

fn save_dir_entries<'a>(path: &Path, dir_entries: &'a [DirEntry<'a>]) -> anyhow::Result<()> {
    for dir_entry in dir_entries {
        match dir_entry {
            DirEntry::Dir(dir) => {
                save_dir_entries(path, dir.entries())?;
            }
            DirEntry::File(file) => {
                let file_path = path.join(file.path());
                std::fs::create_dir_all(Path::new(&file_path).parent().unwrap())
                    .map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
                std::fs::write(file_path, file.contents())
                    .map_err(|e| anyhow::anyhow!("Failed to write file: {}", e))?;
            }
        }
    }
    Ok(())
}
