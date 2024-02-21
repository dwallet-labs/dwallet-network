// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

mod package_lock;

pub mod compilation;
pub mod lock_file;
pub mod package_hooks;
pub mod resolution;
pub mod source_package;

use anyhow::Result;
use clap::*;
use lock_file::LockFile;
use move_compiler::{
    editions::{Edition, Flavor},
    Flags,
};
use move_core_types::account_address::AccountAddress;
use move_model::model::GlobalEnv;
use resolution::{dependency_graph::DependencyGraphBuilder, resolution_graph::ResolvedGraph};
use serde::{Deserialize, Serialize};
use source_package::{layout::SourcePackageLayout, parsed_manifest::DependencyKind};
use std::{
    collections::BTreeMap,
    io::{Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use crate::{
    compilation::{
        build_plan::BuildPlan, compiled_package::CompiledPackage, model_builder::ModelBuilder,
    },
    lock_file::schema::update_compiler_toolchain,
    package_lock::PackageLock,
};

#[derive(Debug, Parser, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Default)]
#[clap(about)]
pub struct BuildConfig {
    /// Compile in 'dev' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used if
    /// this flag is set. This flag is useful for development of packages that expose named
    /// addresses that are not set to a specific value.
    #[clap(name = "dev-mode", short = 'd', long = "dev", global = true)]
    pub dev_mode: bool,

    /// Compile in 'test' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used
    /// along with any code in the 'tests' directory.
    #[clap(name = "test-mode", long = "test", global = true)]
    pub test_mode: bool,

    /// Generate documentation for packages
    #[clap(name = "generate-docs", long = "doc", global = true)]
    pub generate_docs: bool,

    /// Installation directory for compiled artifacts. Defaults to current directory.
    #[clap(long = "install-dir", global = true)]
    pub install_dir: Option<PathBuf>,

    /// Force recompilation of all packages
    #[clap(name = "force-recompilation", long = "force", global = true)]
    pub force_recompilation: bool,

    /// Optional location to save the lock file to, if package resolution succeeds.
    #[clap(skip)]
    pub lock_file: Option<PathBuf>,

    /// Only fetch dependency repos to MOVE_HOME
    #[clap(long = "fetch-deps-only", global = true)]
    pub fetch_deps_only: bool,

    /// Skip fetching latest git dependencies
    #[clap(long = "skip-fetch-latest-git-deps", global = true)]
    pub skip_fetch_latest_git_deps: bool,

    /// Default flavor for move compilation, if not specified in the package's config
    #[clap(long = "default-move-flavor", global = true)]
    pub default_flavor: Option<Flavor>,

    /// Default edition for move compilation, if not specified in the package's config
    #[clap(long = "default-move-edition", global = true)]
    pub default_edition: Option<Edition>,

    /// If set, dependency packages are treated as root packages. Notably, this will remove
    /// warning suppression in dependency packages.
    #[clap(long = "dependencies-are-root", global = true)]
    pub deps_as_root: bool,

    /// If set, ignore any compiler warnings
    #[clap(long = move_compiler::command_line::SILENCE_WARNINGS, global = true)]
    pub silence_warnings: bool,

    /// If set, warnings become errors
    #[clap(long = move_compiler::command_line::WARNINGS_ARE_ERRORS, global = true)]
    pub warnings_are_errors: bool,

    /// Additional named address mapping. Useful for tools in rust
    #[clap(skip)]
    pub additional_named_addresses: BTreeMap<String, AccountAddress>,

    /// If `true`, disable linters
    #[clap(long, global = true)]
    pub no_lint: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct ModelConfig {
    /// If set, also files which are in dependent packages are considered as targets.
    pub all_files_as_targets: bool,
    /// If set, a string how targets are filtered. A target is included if its file name
    /// contains this string. This is similar as the `cargo test <string>` idiom.
    pub target_filter: Option<String>,
}

impl BuildConfig {
    /// Compile the package at `path` or the containing Move package. Exit process on warning or
    /// failure.
    pub fn compile_package<W: Write>(self, path: &Path, writer: &mut W) -> Result<CompiledPackage> {
        let resolved_graph = self.resolution_graph_for_package(path, writer)?;
        let _mutx = PackageLock::lock(); // held until function returns
        BuildPlan::create(resolved_graph)?.compile(writer)
    }

    /// Compile the package at `path` or the containing Move package. Do not exit process on warning
    /// or failure.
    pub fn compile_package_no_exit<W: Write>(
        self,
        path: &Path,
        writer: &mut W,
    ) -> Result<CompiledPackage> {
        let resolved_graph = self.resolution_graph_for_package(path, writer)?;
        let _mutx = PackageLock::lock(); // held until function returns
        BuildPlan::create(resolved_graph)?.compile_no_exit(writer)
    }

    // NOTE: If there are no renamings, then the root package has the global resolution of all named
    // addresses in the package graph in scope. So we can simply grab all of the source files
    // across all packages and build the Move model from that.
    // TODO: In the future we will need a better way to do this to support renaming in packages
    // where we want to support building a Move model.
    pub fn move_model_for_package(
        self,
        path: &Path,
        model_config: ModelConfig,
    ) -> Result<GlobalEnv> {
        // resolution graph diagnostics are only needed for CLI commands so ignore them by passing a
        // vector as the writer
        let resolved_graph = self.resolution_graph_for_package(path, &mut Vec::new())?;
        let _mutx = PackageLock::lock(); // held until function returns
        ModelBuilder::create(resolved_graph, model_config).build_model()
    }

    pub fn download_deps_for_package<W: Write>(&self, path: &Path, writer: &mut W) -> Result<()> {
        let path = SourcePackageLayout::try_find_root(path)?;
        let manifest_string =
            std::fs::read_to_string(path.join(SourcePackageLayout::Manifest.path()))?;
        let lock_string = std::fs::read_to_string(path.join(SourcePackageLayout::Lock.path())).ok();
        let _mutx = PackageLock::lock(); // held until function returns

        resolution::download_dependency_repos(manifest_string, lock_string, self, &path, writer)?;
        Ok(())
    }

    pub fn resolution_graph_for_package<W: Write>(
        mut self,
        path: &Path,
        writer: &mut W,
    ) -> Result<ResolvedGraph> {
        if self.test_mode {
            self.dev_mode = true;
        }
        let path = SourcePackageLayout::try_find_root(path)?;
        let manifest_string =
            std::fs::read_to_string(path.join(SourcePackageLayout::Manifest.path()))?;
        let lock_string = std::fs::read_to_string(path.join(SourcePackageLayout::Lock.path())).ok();
        let _mutx = PackageLock::lock(); // held until function returns

        let install_dir_set = self.install_dir.is_some();
        let install_dir = self.install_dir.as_ref().unwrap_or(&path).to_owned();

        let mut dep_graph_builder = DependencyGraphBuilder::new(
            self.skip_fetch_latest_git_deps,
            writer,
            install_dir.clone(),
        );
        let (dependency_graph, modified) = dep_graph_builder.get_graph(
            &DependencyKind::default(),
            path,
            manifest_string,
            lock_string,
        )?;

        if modified || install_dir_set {
            // (1) Write the Move.lock file if the existing one is `modified`, or
            // (2) `install_dir` is set explicitly, which may be a different directory, and where a Move.lock does not exist yet.
            let lock = dependency_graph.write_to_lock(install_dir)?;
            if let Some(lock_path) = &self.lock_file {
                lock.commit(lock_path)?;
            }
        }

        let DependencyGraphBuilder {
            mut dependency_cache,
            progress_output,
            ..
        } = dep_graph_builder;

        ResolvedGraph::resolve(
            dependency_graph,
            self,
            &mut dependency_cache,
            progress_output,
        )
    }

    pub fn compiler_flags(&self) -> Flags {
        let flags = if self.test_mode {
            Flags::testing()
        } else {
            Flags::empty()
        };
        flags
            .set_warnings_are_errors(self.warnings_are_errors)
            .set_silence_warnings(self.silence_warnings)
    }

    pub fn update_lock_file_toolchain_version(
        &self,
        path: &PathBuf,
        compiler_version: String,
    ) -> Result<()> {
        let Some(lock_file) = self.lock_file.as_ref() else {
            return Ok(());
        };
        let install_dir = self.install_dir.as_ref().unwrap_or(path).to_owned();
        let mut lock = LockFile::from(install_dir, lock_file)?;
        lock.seek(SeekFrom::Start(0))?;
        update_compiler_toolchain(
            &mut lock,
            compiler_version,
            self.default_edition.unwrap_or_default(),
            self.default_flavor.unwrap_or_default(),
        )?;
        let _mutx = PackageLock::lock();
        lock.commit(lock_file)?;
        Ok(())
    }
}
