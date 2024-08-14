// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::fmt::{Display, Formatter, Write};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use sui_config::Config;
use sui_keys::keystore::{AccountKeystore, Keystore};
use sui_types::base_types::*;

use crate::{SuiClient, SuiClientBuilder, SUI_DEVNET_URL, SUI_LOCAL_NETWORK_URL, SUI_TESTNET_URL};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SuiClientConfig {
    pub keystore: Keystore,
    pub envs: Vec<SuiEnv>,
    pub active_env: Option<String>,
    pub active_address: Option<SuiAddress>,
}

/// Configuration settings for the Ethereum light client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthClientSettings {
    pub eth_execution_rpc: Option<String>,
    pub eth_consensus_rpc: Option<String>,
    pub state_object_id: Option<ObjectID>,
}

impl SuiClientConfig {
    pub fn new(keystore: Keystore) -> Self {
        SuiClientConfig {
            keystore,
            envs: vec![],
            active_env: None,
            active_address: None,
        }
    }

    pub fn get_env(&self, alias: &Option<String>) -> Option<&SuiEnv> {
        if let Some(alias) = alias {
            self.envs.iter().find(|env| &env.alias == alias)
        } else {
            self.envs.first()
        }
    }

    pub fn get_active_env(&self) -> Result<&SuiEnv, anyhow::Error> {
        self.get_env(&self.active_env).ok_or_else(|| {
            anyhow!(
                "Environment configuration wasn't found for the environment: [{}]",
                self.active_env.as_deref().unwrap_or("None")
            )
        })
    }

    fn get_active_env_mut(&mut self) -> Result<&mut SuiEnv, anyhow::Error> {
        let active_env = self.active_env.clone();
        self.get_env_mut(&active_env).ok_or_else(|| {
            anyhow!(
                "Environment configuration wasn't found for the environment: [{}]",
                active_env.unwrap_or("None".to_string())
            )
        })
    }

    fn get_env_mut(&mut self, alias: &Option<String>) -> Option<&mut SuiEnv> {
        alias
            .as_ref()
            .and_then(|alias| self.envs.iter_mut().find(|env| &env.alias == alias))
    }

    pub fn update_ethereum_state_object_id(
        &mut self,
        object_id: ObjectID,
    ) -> Result<(), anyhow::Error> {
        let env = self.get_active_env_mut()?;
        env.eth_client_settings
            .as_mut()
            .map(|config| config.state_object_id = Some(object_id))
            .ok_or_else(|| {
                anyhow!(
                    "no Ethereum State object ID found for active environment: [{}]",
                    env.alias
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiEnv {
    pub alias: String,
    pub rpc: String,
    pub ws: Option<String>,
    pub eth_client_settings: Option<EthClientSettings>,
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
            eth_client_settings: None,
        }
    }
    pub fn testnet() -> Self {
        Self {
            alias: "testnet".to_string(),
            rpc: SUI_TESTNET_URL.into(),
            ws: None,
            eth_client_settings: None,
        }
    }

    pub fn localnet() -> Self {
        Self {
            alias: "local".to_string(),
            rpc: SUI_LOCAL_NETWORK_URL.into(),
            ws: None,
            eth_client_settings: None,
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
