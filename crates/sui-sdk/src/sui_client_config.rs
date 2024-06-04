// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::fmt::{Display, Formatter, Write};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{SuiClient, SuiClientBuilder, SUI_DEVNET_URL, SUI_LOCAL_NETWORK_URL, SUI_TESTNET_URL};
use sui_config::Config;
use sui_keys::keystore::{AccountKeystore, Keystore};
use sui_types::base_types::*;
use signature_mpc::twopc_mpc_protocols::{DKGCentralizedPartyOutput, DKGDecentralizedPartyOutput};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SuiClientConfig {
    pub keystore: Keystore,
    pub envs: Vec<SuiEnv>,
    pub dwallets: Vec<DWalletSecretShare>,
    pub active_env: Option<String>,
    pub active_address: Option<SuiAddress>,
    pub active_dwallet: Option<String>,
}

impl SuiClientConfig {
    pub fn new(keystore: Keystore) -> Self {
        SuiClientConfig {
            keystore,
            envs: vec![],
            dwallets: vec![],
            active_env: None,
            active_address: None,
            active_dwallet: None,
        }
    }

    pub fn get_env(&self, alias: &Option<String>) -> Option<&SuiEnv> {
        if let Some(alias) = alias {
            self.envs.iter().find(|env| &env.alias == alias)
        } else {
            self.envs.first()
        }
    }

    pub fn get_env_mut(&mut self, alias: &Option<String>) -> Option<&mut SuiEnv> {
        if let Some(alias) = alias {
            self.envs.iter_mut().find(|env| &env.alias == alias)
        } else {
            self.envs.first_mut()
        }
    }

    pub fn get_active_env_mut(&mut self) -> Result<&mut SuiEnv, anyhow::Error> {
        let active_env = self.active_env.clone(); // Clone active_env to avoid immutable borrow
        match self.get_env_mut(&active_env) {
            None => Err(anyhow!(
            "Environment configuration not found for env [{}]",
            active_env.as_deref().unwrap_or("None")
        )),
            Some(env) => Ok(env),
        }
    }

    pub fn get_active_env(&self) -> Result<&SuiEnv, anyhow::Error> {
        self.get_env(&self.active_env).ok_or_else(|| {
            anyhow!(
                "Environment configuration not found for env [{}]",
                self.active_env.as_deref().unwrap_or("None")
            )
        })
    }

    pub fn add_env(&mut self, env: SuiEnv) {
        if !self
            .envs
            .iter()
            .any(|other_env| other_env.alias == env.alias)
        {
            self.envs.push(env)
        }
    }

    pub fn get_dwallet(&self, alias: &Option<String>) -> Option<&DWalletSecretShare> {
        if let Some(alias) = alias {
            self.dwallets.iter().find(|dwallet| &dwallet.alias == alias)
        } else {
            self.dwallets.first()
        }
    }

    pub fn get_active_dwallet(&self) -> Result<&DWalletSecretShare, anyhow::Error> {
        self.get_dwallet(&self.active_dwallet).ok_or_else(|| {
            anyhow!(
                "dWallet configuration not found for dwallet [{}]",
                self.active_dwallet.as_deref().unwrap_or("None")
            )
        })
    }

    pub fn add_dwallet(&mut self, dwallet: DWalletSecretShare) {
        if !self
            .dwallets
            .iter()
            .any(|other_dwallet| other_dwallet.alias == dwallet.alias)
        {
            if self.active_dwallet.is_none() {
                self.active_dwallet = Some(dwallet.alias.clone());
            }
            self.dwallets.push(dwallet)
        }
    }

    pub fn update_ethereum_state_object_id(&mut self, object_id: ObjectID) {
        if let Some(env) = self.get_active_env_mut().ok() {
            env.state_object_id = Some(object_id);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiEnv {
    pub alias: String,
    pub rpc: String,
    pub ws: Option<String>,
    pub eth_execution_rpc: Option<String>,
    pub eth_consensus_rpc: Option<String>,
    pub eth_chain_id: Option<u64>,
    pub eth_genesis_time:Option<u64>,
    pub eth_genesis_validators_root:Option<String>,
    pub state_object_id: Option<ObjectID>,
}

impl SuiEnv {
    pub async fn create_rpc_client(
        &self,
        request_timeout: Option<std::time::Duration>,
        max_concurrent_requests: Option<u64>,
    ) -> Result<SuiClient, anyhow::Error> {
        let mut builder = SuiClientBuilder::default();
        if let Some(request_timeout) = request_timeout {
            builder = builder.request_timeout(request_timeout);
        }
        if let Some(ws_url) = &self.ws {
            builder = builder.ws_url(ws_url);
        }

        if let Some(max_concurrent_requests) = max_concurrent_requests {
            builder = builder.max_concurrent_requests(max_concurrent_requests as usize);
        }
        Ok(builder.build(&self.rpc).await?)
    }

    pub fn devnet() -> Self {
        Self {
            alias: "devnet".to_string(),
            rpc: SUI_DEVNET_URL.into(),
            ws: None,
            eth_execution_rpc: None,
            eth_consensus_rpc: None,
            eth_genesis_time: None,
            eth_chain_id: None,
            eth_genesis_validators_root: None,
            state_object_id: None,
        }
    }
    pub fn testnet() -> Self {
        Self {
            alias: "testnet".to_string(),
            rpc: SUI_TESTNET_URL.into(),
            ws: None,
            eth_execution_rpc: None,
            eth_consensus_rpc: None,
            eth_genesis_time: None,
            eth_chain_id: None,
            eth_genesis_validators_root: None,
            state_object_id: None,
        }
    }

    pub fn localnet() -> Self {
        Self {
            alias: "local".to_string(),
            rpc: SUI_LOCAL_NETWORK_URL.into(),
            ws: None,
            eth_execution_rpc: None,
            eth_consensus_rpc: None,
            eth_genesis_time: None,
            eth_chain_id: None,
            eth_genesis_validators_root: None,
            state_object_id: None,
        }
    }
}

impl Display for SuiEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Active environment : {}", self.alias)?;
        write!(writer, "RPC URL: {}", self.rpc)?;
        if let Some(ws) = &self.ws {
            writeln!(writer)?;
            write!(writer, "Websocket URL: {ws}")?;
        }
        write!(f, "{}", writer)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DWalletSecretShare {
    pub alias: String,
    //pub public_key: String,
    pub dkg_output: DKGCentralizedPartyOutput,
    pub dwallet_id: ObjectID,
    pub dwallet_cap_id: ObjectID,
}

impl Display for DWalletSecretShare {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Active dWallet : {}", self.alias)?;
        write!(writer, "dwallet_id: {}", self.dwallet_id)?;
        write!(writer, "dwallet_cap_id: {}", self.dwallet_cap_id)?;
        write!(f, "{}", writer)
    }
}

impl Config for SuiClientConfig {}

impl Display for SuiClientConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        writeln!(
            writer,
            "Managed addresses : {}",
            self.keystore.addresses().len()
        )?;
        write!(writer, "Active address: ")?;
        match self.active_address {
            Some(r) => writeln!(writer, "{}", r)?,
            None => writeln!(writer, "None")?,
        };
        writeln!(writer, "{}", self.keystore)?;
        if let Ok(env) = self.get_active_env() {
            write!(writer, "{}", env)?;
        }
        write!(f, "{}", writer)
    }
}
