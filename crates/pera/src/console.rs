// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::io::{stderr, Write};
use std::ops::Deref;

use async_trait::async_trait;
use clap::Command;
use clap::CommandFactory;
use clap::FromArgMatches;
use clap::Parser;
use colored::Colorize;
use pera_sdk::wallet_context::WalletContext;

use crate::client_commands::SwitchResponse;
use crate::client_commands::{PeraClientCommandResult, PeraClientCommands};
use crate::shell::{
    install_shell_plugins, AsyncHandler, CacheKey, CommandStructure, CompletionCache, Shell,
};

const PERA: &str = "   _____       _    ______                       __
  / ___/__  __(_)  / ____/___  ____  _________  / /__
  \\__ \\/ / / / /  / /   / __ \\/ __ \\/ ___/ __ \\/ / _ \\
 ___/ / /_/ / /  / /___/ /_/ / / / (__  ) /_/ / /  __/
/____/\\__,_/_/   \\____/\\____/_/ /_/____/\\____/_/\\___/";

#[derive(Parser)]
#[clap(name = "", rename_all = "kebab-case", no_binary_name = true)]
pub struct ConsoleOpts {
    #[clap(subcommand)]
    pub command: PeraClientCommands,
    /// Returns command outputs in JSON format.
    #[clap(long, global = true)]
    pub json: bool,
}

pub async fn start_console(
    context: WalletContext,
    out: &mut (dyn Write + Send),
    err: &mut (dyn Write + Send),
) -> Result<(), anyhow::Error> {
    let app: Command = PeraClientCommands::command();
    writeln!(out, "{}", PERA.cyan().bold())?;
    let mut version = env!("CARGO_PKG_VERSION").to_owned();
    if let Some(git_rev) = std::option_env!("GIT_REVISION") {
        version.push('-');
        version.push_str(git_rev);
    }
    writeln!(out, "--- Pera Console {version} ---")?;
    writeln!(out)?;
    writeln!(out, "{}", context.config.deref())?;

    let client = context.get_client().await?;
    writeln!(
        out,
        "Connecting to Pera full node. API version {}",
        client.api_version()
    )?;

    if !client.available_rpc_methods().is_empty() {
        writeln!(out)?;
        writeln!(
            out,
            "Available RPC methods: {:?}",
            client.available_rpc_methods()
        )?;
    }
    if !client.available_subscriptions().is_empty() {
        writeln!(out)?;
        writeln!(
            out,
            "Available Subscriptions: {:?}",
            client.available_subscriptions()
        )?;
    }

    writeln!(out)?;
    writeln!(out, "Welcome to the Pera interactive console.")?;
    writeln!(out)?;

    let mut shell = Shell::new(
        "pera>-$ ",
        context,
        ClientCommandHandler,
        CommandStructure::from_clap(&install_shell_plugins(app)),
    );

    shell.run_async(out, err).await
}

struct ClientCommandHandler;

#[async_trait]
impl AsyncHandler<WalletContext> for ClientCommandHandler {
    async fn handle_async(
        &self,
        args: Vec<String>,
        context: &mut WalletContext,
        completion_cache: CompletionCache,
    ) -> bool {
        match handle_command(get_command(args), context, completion_cache).await {
            Err(e) => {
                let _err = writeln!(stderr(), "{}", e.to_string().red());
                false
            }
            Ok(return_value) => return_value,
        }
    }
}

fn get_command(args: Vec<String>) -> Result<ConsoleOpts, anyhow::Error> {
    let app: Command = install_shell_plugins(ConsoleOpts::command());
    Ok(ConsoleOpts::from_arg_matches(
        &app.try_get_matches_from(args)?,
    )?)
}

async fn handle_command(
    wallet_opts: Result<ConsoleOpts, anyhow::Error>,
    context: &mut WalletContext,
    completion_cache: CompletionCache,
) -> Result<bool, anyhow::Error> {
    let wallet_opts = wallet_opts?;
    let result = wallet_opts.command.execute(context).await?;

    // Update completion cache
    // TODO: Completion data are keyed by strings, are there ways to make it more error proof?
    if let Ok(mut cache) = completion_cache.write() {
        match result {
            PeraClientCommandResult::Addresses(ref addresses) => {
                let addresses = addresses
                    .addresses
                    .iter()
                    .map(|addr| format!("{}", addr.1))
                    .collect::<Vec<_>>();
                cache.insert(CacheKey::flag("--address"), addresses.clone());
                cache.insert(CacheKey::flag("--to"), addresses);
            }
            PeraClientCommandResult::Objects(ref objects) => {
                let objects = objects
                    .iter()
                    .map(|oref| format!("{}", oref.clone().into_object().unwrap().object_id))
                    .collect::<Vec<_>>();
                cache.insert(CacheKey::new("object", "--id"), objects.clone());
                cache.insert(CacheKey::flag("--gas"), objects.clone());
                cache.insert(CacheKey::flag("--coin-object-id"), objects);
            }
            _ => {}
        }
    }
    result.print(!wallet_opts.json);

    // Quit shell after RPC switch
    if matches!(
        result,
        PeraClientCommandResult::Switch(SwitchResponse { env: Some(_), .. })
    ) {
        println!("Pera environment switch completed, please restart Pera console.");
        return Ok(true);
    }
    Ok(false)
}
