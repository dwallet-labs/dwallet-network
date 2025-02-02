// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;
use std::vec;

use anyhow::anyhow;
use move_core_types::ident_str;
use move_core_types::language_storage::{ModuleId, StructTag};
use move_core_types::resolver::ModuleResolver;
use serde::Deserialize;
use serde::Serialize;

use ika_json_rpc_types::IkaProgrammableMoveCall;
use ika_json_rpc_types::IkaProgrammableTransactionBlock;
use ika_json_rpc_types::{BalanceChange, IkaArgument};
use ika_json_rpc_types::{IkaCallArg, IkaCommand};
use ika_sdk::rpc_types::{
    IkaTransactionBlockData, IkaTransactionBlockDataAPI, IkaTransactionBlockEffectsAPI,
    IkaTransactionBlockKind, IkaTransactionBlockResponse,
};
use ika_types::base_types::{ObjectID, SequenceNumber, IkaAddress};
use ika_types::gas_coin::GasCoin;
use ika_types::governance::{ADD_STAKE_FUN_NAME, WITHDRAW_STAKE_FUN_NAME};
use ika_types::object::Owner;
use ika_types::ika_system_state::IKA_SYSTEM_MODULE_NAME;
use ika_types::transaction::TransactionData;
use ika_types::{IKA_SYSTEM_ADDRESS, IKA_SYSTEM_PACKAGE_ID};

use crate::types::{
    AccountIdentifier, Amount, CoinAction, CoinChange, CoinID, CoinIdentifier, Currency,
    InternalOperation, OperationIdentifier, OperationStatus, OperationType,
};
use crate::{CoinMetadataCache, Error, IKA};

#[cfg(test)]
#[path = "unit_tests/operations_tests.rs"]
mod operations_tests;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Operations(Vec<Operation>);

impl FromIterator<Operation> for Operations {
    fn from_iter<T: IntoIterator<Item = Operation>>(iter: T) -> Self {
        Operations::new(iter.into_iter().collect())
    }
}

impl FromIterator<Vec<Operation>> for Operations {
    fn from_iter<T: IntoIterator<Item = Vec<Operation>>>(iter: T) -> Self {
        iter.into_iter().flatten().collect()
    }
}

impl IntoIterator for Operations {
    type Item = Operation;
    type IntoIter = vec::IntoIter<Operation>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Operations {
    pub fn new(mut ops: Vec<Operation>) -> Self {
        for (index, op) in ops.iter_mut().enumerate() {
            op.operation_identifier = (index as u64).into()
        }
        Self(ops)
    }

    pub fn contains(&self, other: &Operations) -> bool {
        for (i, other_op) in other.0.iter().enumerate() {
            if let Some(op) = self.0.get(i) {
                if op != other_op {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn set_status(mut self, status: Option<OperationStatus>) -> Self {
        for op in &mut self.0 {
            op.status = status
        }
        self
    }

    pub fn type_(&self) -> Option<OperationType> {
        self.0.first().map(|op| op.type_)
    }

    /// Parse operation input from rosetta operation to intermediate internal operation;
    pub fn into_internal(self) -> Result<InternalOperation, Error> {
        let type_ = self
            .type_()
            .ok_or_else(|| Error::MissingInput("Operation type".into()))?;
        match type_ {
            OperationType::PayIka => self.pay_ika_ops_to_internal(),
            OperationType::PayCoin => self.pay_coin_ops_to_internal(),
            OperationType::Stake => self.stake_ops_to_internal(),
            OperationType::WithdrawStake => self.withdraw_stake_ops_to_internal(),
            op => Err(Error::UnsupportedOperation(op)),
        }
    }

    fn pay_ika_ops_to_internal(self) -> Result<InternalOperation, Error> {
        let mut recipients = vec![];
        let mut amounts = vec![];
        let mut sender = None;
        for op in self {
            if let (Some(amount), Some(account)) = (op.amount.clone(), op.account.clone()) {
                if amount.value.is_negative() {
                    sender = Some(account.address)
                } else {
                    recipients.push(account.address);
                    let amount = amount.value.abs();
                    if amount > u64::MAX as i128 {
                        return Err(Error::InvalidInput(
                            "Input amount exceed u64::MAX".to_string(),
                        ));
                    }
                    amounts.push(amount as u64)
                }
            }
        }
        let sender = sender.ok_or_else(|| Error::MissingInput("Sender address".to_string()))?;
        Ok(InternalOperation::PayIka {
            sender,
            recipients,
            amounts,
        })
    }

    fn pay_coin_ops_to_internal(self) -> Result<InternalOperation, Error> {
        let mut recipients = vec![];
        let mut amounts = vec![];
        let mut sender = None;
        let mut currency = None;
        for op in self {
            if let (Some(amount), Some(account)) = (op.amount.clone(), op.account.clone()) {
                currency = currency.or(Some(amount.currency));
                if amount.value.is_negative() {
                    sender = Some(account.address)
                } else {
                    recipients.push(account.address);
                    let amount = amount.value.abs();
                    if amount > u64::MAX as i128 {
                        return Err(Error::InvalidInput(
                            "Input amount exceed u64::MAX".to_string(),
                        ));
                    }
                    amounts.push(amount as u64)
                }
            }
        }
        let sender = sender.ok_or_else(|| Error::MissingInput("Sender address".to_string()))?;
        let currency = currency.ok_or_else(|| Error::MissingInput("Currency".to_string()))?;
        Ok(InternalOperation::PayCoin {
            sender,
            recipients,
            amounts,
            currency,
        })
    }

    fn stake_ops_to_internal(self) -> Result<InternalOperation, Error> {
        let mut ops = self
            .0
            .into_iter()
            .filter(|op| op.type_ == OperationType::Stake)
            .collect::<Vec<_>>();
        if ops.len() != 1 {
            return Err(Error::MalformedOperationError(
                "Delegation should only have one operation.".into(),
            ));
        }
        // Checked above, safe to unwrap.
        let op = ops.pop().unwrap();
        let sender = op
            .account
            .ok_or_else(|| Error::MissingInput("Sender address".to_string()))?
            .address;
        let metadata = op
            .metadata
            .ok_or_else(|| Error::MissingInput("Stake metadata".to_string()))?;

        // Total issued SUi is less than u64, safe to cast.
        let amount = if let Some(amount) = op.amount {
            if amount.value.is_positive() {
                return Err(Error::MalformedOperationError(
                    "Stake amount should be negative.".into(),
                ));
            }
            Some(amount.value.unsigned_abs() as u64)
        } else {
            None
        };

        let OperationMetadata::Stake { validator } = metadata else {
            return Err(Error::InvalidInput(
                "Cannot find delegation info from metadata.".into(),
            ));
        };

        Ok(InternalOperation::Stake {
            sender,
            validator,
            amount,
        })
    }

    fn withdraw_stake_ops_to_internal(self) -> Result<InternalOperation, Error> {
        let mut ops = self
            .0
            .into_iter()
            .filter(|op| op.type_ == OperationType::WithdrawStake)
            .collect::<Vec<_>>();
        if ops.len() != 1 {
            return Err(Error::MalformedOperationError(
                "Delegation should only have one operation.".into(),
            ));
        }
        // Checked above, safe to unwrap.
        let op = ops.pop().unwrap();
        let sender = op
            .account
            .ok_or_else(|| Error::MissingInput("Sender address".to_string()))?
            .address;

        let stake_ids = if let Some(metadata) = op.metadata {
            let OperationMetadata::WithdrawStake { stake_ids } = metadata else {
                return Err(Error::InvalidInput(
                    "Cannot find withdraw stake info from metadata.".into(),
                ));
            };
            stake_ids
        } else {
            vec![]
        };

        Ok(InternalOperation::WithdrawStake { sender, stake_ids })
    }

    fn from_transaction(
        tx: IkaTransactionBlockKind,
        sender: IkaAddress,
        status: Option<OperationStatus>,
    ) -> Result<Vec<Operation>, Error> {
        Ok(match tx {
            IkaTransactionBlockKind::ProgrammableTransaction(pt)
                if status != Some(OperationStatus::Failure) =>
            {
                Self::parse_programmable_transaction(sender, status, pt)?
            }
            _ => vec![Operation::generic_op(status, sender, tx)],
        })
    }

    fn parse_programmable_transaction(
        sender: IkaAddress,
        status: Option<OperationStatus>,
        pt: IkaProgrammableTransactionBlock,
    ) -> Result<Vec<Operation>, Error> {
        #[derive(Debug)]
        enum KnownValue {
            GasCoin(u64),
        }
        fn resolve_result(
            known_results: &[Vec<KnownValue>],
            i: u16,
            j: u16,
        ) -> Option<&KnownValue> {
            known_results
                .get(i as usize)
                .and_then(|inner| inner.get(j as usize))
        }
        fn split_coins(
            inputs: &[IkaCallArg],
            known_results: &[Vec<KnownValue>],
            coin: IkaArgument,
            amounts: &[IkaArgument],
        ) -> Option<Vec<KnownValue>> {
            match coin {
                IkaArgument::Result(i) => {
                    let KnownValue::GasCoin(_) = resolve_result(known_results, i, 0)?;
                }
                IkaArgument::NestedResult(i, j) => {
                    let KnownValue::GasCoin(_) = resolve_result(known_results, i, j)?;
                }
                IkaArgument::GasCoin => (),
                // Might not be a IKA coin
                IkaArgument::Input(_) => (),
            };
            let amounts = amounts
                .iter()
                .map(|amount| {
                    let value: u64 = match *amount {
                        IkaArgument::Input(i) => {
                            u64::from_str(inputs.get(i as usize)?.pure()?.to_json_value().as_str()?)
                                .ok()?
                        }
                        IkaArgument::GasCoin
                        | IkaArgument::Result(_)
                        | IkaArgument::NestedResult(_, _) => return None,
                    };
                    Some(KnownValue::GasCoin(value))
                })
                .collect::<Option<_>>()?;
            Some(amounts)
        }
        fn transfer_object(
            aggregated_recipients: &mut HashMap<IkaAddress, u64>,
            inputs: &[IkaCallArg],
            known_results: &[Vec<KnownValue>],
            objs: &[IkaArgument],
            recipient: IkaArgument,
        ) -> Option<Vec<KnownValue>> {
            let addr = match recipient {
                IkaArgument::Input(i) => inputs.get(i as usize)?.pure()?.to_ika_address().ok()?,
                IkaArgument::GasCoin | IkaArgument::Result(_) | IkaArgument::NestedResult(_, _) => {
                    return None
                }
            };
            for obj in objs {
                let value = match *obj {
                    IkaArgument::Result(i) => {
                        let KnownValue::GasCoin(value) = resolve_result(known_results, i, 0)?;
                        value
                    }
                    IkaArgument::NestedResult(i, j) => {
                        let KnownValue::GasCoin(value) = resolve_result(known_results, i, j)?;
                        value
                    }
                    IkaArgument::GasCoin | IkaArgument::Input(_) => return None,
                };
                let aggregate = aggregated_recipients.entry(addr).or_default();
                *aggregate += value;
            }
            Some(vec![])
        }
        fn stake_call(
            inputs: &[IkaCallArg],
            known_results: &[Vec<KnownValue>],
            call: &IkaProgrammableMoveCall,
        ) -> Result<Option<(Option<u64>, IkaAddress)>, Error> {
            let IkaProgrammableMoveCall { arguments, .. } = call;
            let (amount, validator) = match &arguments[..] {
                [_, coin, validator] => {
                    let amount = match coin {
                        IkaArgument::Result(i) =>{
                            let KnownValue::GasCoin(value) = resolve_result(known_results, *i, 0).ok_or_else(||anyhow!("Cannot resolve Gas coin value at Result({i})"))?;
                            value
                        },
                        _ => return Ok(None),
                    };
                    let (some_amount, validator) = match validator {
                        // [WORKAROUND] - this is a hack to work out if the staking ops is for a selected amount or None amount (whole wallet).
                        // We use the position of the validator arg as a indicator of if the rosetta stake
                        // transaction is staking the whole wallet or not, if staking whole wallet,
                        // we have to omit the amount value in the final operation output.
                        IkaArgument::Input(i) => (*i==1, inputs.get(*i as usize).and_then(|input| input.pure()).map(|v|v.to_ika_address()).transpose()),
                        _=> return Ok(None),
                    };
                    (some_amount.then_some(*amount), validator)
                },
                _ => Err(anyhow!("Error encountered when extracting arguments from move call, expecting 3 elements, got {}", arguments.len()))?,
            };
            Ok(validator.map(|v| v.map(|v| (amount, v)))?)
        }

        fn unstake_call(
            inputs: &[IkaCallArg],
            call: &IkaProgrammableMoveCall,
        ) -> Result<Option<ObjectID>, Error> {
            let IkaProgrammableMoveCall { arguments, .. } = call;
            let id = match &arguments[..] {
                [_, stake_id] => {
                    match stake_id {
                        IkaArgument::Input(i) => {
                            let id = inputs.get(*i as usize).and_then(|input| input.object()).ok_or_else(|| anyhow!("Cannot find stake id from input args."))?;
                            // [WORKAROUND] - this is a hack to work out if the withdraw stake ops is for a selected stake or None (all stakes).
                            // this hack is similar to the one in stake_call.
                            let some_id = i % 2 == 1;
                            some_id.then_some(id)
                        },
                        _=> return Ok(None),
                    }
                },
                _ => Err(anyhow!("Error encountered when extracting arguments from move call, expecting 3 elements, got {}", arguments.len()))?,
            };
            Ok(id.cloned())
        }
        let IkaProgrammableTransactionBlock { inputs, commands } = &pt;
        let mut known_results: Vec<Vec<KnownValue>> = vec![];
        let mut aggregated_recipients: HashMap<IkaAddress, u64> = HashMap::new();
        let mut needs_generic = false;
        let mut operations = vec![];
        let mut stake_ids = vec![];
        let mut currency: Option<Currency> = None;
        for command in commands {
            let result = match command {
                IkaCommand::SplitCoins(coin, amounts) => {
                    split_coins(inputs, &known_results, *coin, amounts)
                }
                IkaCommand::TransferObjects(objs, addr) => transfer_object(
                    &mut aggregated_recipients,
                    inputs,
                    &known_results,
                    objs,
                    *addr,
                ),
                IkaCommand::MoveCall(m) if Self::is_stake_call(m) => {
                    stake_call(inputs, &known_results, m)?.map(|(amount, validator)| {
                        let amount = amount.map(|amount| Amount::new(-(amount as i128), None));
                        operations.push(Operation {
                            operation_identifier: Default::default(),
                            type_: OperationType::Stake,
                            status,
                            account: Some(sender.into()),
                            amount,
                            coin_change: None,
                            metadata: Some(OperationMetadata::Stake { validator }),
                        });
                        vec![]
                    })
                }
                IkaCommand::MoveCall(m) if Self::is_unstake_call(m) => {
                    let stake_id = unstake_call(inputs, m)?;
                    stake_ids.push(stake_id);
                    Some(vec![])
                }
                _ => None,
            };
            if let Some(result) = result {
                known_results.push(result)
            } else {
                needs_generic = true;
                break;
            }
        }

        if !needs_generic && !aggregated_recipients.is_empty() {
            let total_paid: u64 = aggregated_recipients.values().copied().sum();
            operations.extend(
                aggregated_recipients
                    .into_iter()
                    .map(|(recipient, amount)| {
                        currency = inputs.iter().last().and_then(|arg| {
                            if let IkaCallArg::Pure(value) = arg {
                                let bytes = value
                                    .value()
                                    .to_json_value()
                                    .as_array()?
                                    .clone()
                                    .into_iter()
                                    .map(|v| v.as_u64().map(|n| n as u8))
                                    .collect::<Option<Vec<u8>>>()?;
                                bcs::from_bytes::<String>(&bytes)
                                    .ok()
                                    .and_then(|bcs_str| serde_json::from_str(&bcs_str).ok())
                            } else {
                                None
                            }
                        });
                        match currency {
                            Some(_) => Operation::pay_coin(
                                status,
                                recipient,
                                amount.into(),
                                currency.clone(),
                            ),
                            None => Operation::pay_ika(status, recipient, amount.into()),
                        }
                    }),
            );
            match currency {
                Some(_) => operations.push(Operation::pay_coin(
                    status,
                    sender,
                    -(total_paid as i128),
                    currency.clone(),
                )),
                _ => operations.push(Operation::pay_ika(status, sender, -(total_paid as i128))),
            }
        } else if !stake_ids.is_empty() {
            let stake_ids = stake_ids.into_iter().flatten().collect::<Vec<_>>();
            let metadata = stake_ids
                .is_empty()
                .not()
                .then_some(OperationMetadata::WithdrawStake { stake_ids });
            operations.push(Operation {
                operation_identifier: Default::default(),
                type_: OperationType::WithdrawStake,
                status,
                account: Some(sender.into()),
                amount: None,
                coin_change: None,
                metadata,
            });
        } else if operations.is_empty() {
            operations.push(Operation::generic_op(
                status,
                sender,
                IkaTransactionBlockKind::ProgrammableTransaction(pt),
            ))
        }
        Ok(operations)
    }

    fn is_stake_call(tx: &IkaProgrammableMoveCall) -> bool {
        tx.package == IKA_SYSTEM_PACKAGE_ID
            && tx.module == IKA_SYSTEM_MODULE_NAME.as_str()
            && tx.function == ADD_STAKE_FUN_NAME.as_str()
    }

    fn is_unstake_call(tx: &IkaProgrammableMoveCall) -> bool {
        tx.package == IKA_SYSTEM_PACKAGE_ID
            && tx.module == IKA_SYSTEM_MODULE_NAME.as_str()
            && tx.function == WITHDRAW_STAKE_FUN_NAME.as_str()
    }

    fn process_balance_change(
        gas_owner: IkaAddress,
        gas_used: i128,
        balance_changes: Vec<(BalanceChange, Currency)>,
        status: Option<OperationStatus>,
        balances: HashMap<(IkaAddress, Currency), i128>,
    ) -> impl Iterator<Item = Operation> {
        let mut balances =
            balance_changes
                .iter()
                .fold(balances, |mut balances, (balance_change, ccy)| {
                    // Rosetta only care about address owner
                    if let Owner::AddressOwner(owner) = balance_change.owner {
                        *balances.entry((owner, ccy.clone())).or_default() += balance_change.amount;
                    }
                    balances
                });
        // separate gas from balances
        *balances.entry((gas_owner, IKA.clone())).or_default() -= gas_used;

        let balance_change = balances.into_iter().filter(|(_, amount)| *amount != 0).map(
            move |((addr, currency), amount)| {
                Operation::balance_change(status, addr, amount, currency)
            },
        );

        let gas = if gas_used != 0 {
            vec![Operation::gas(gas_owner, gas_used)]
        } else {
            // Gas can be 0 for system tx
            vec![]
        };
        balance_change.chain(gas)
    }
}

impl Operations {
    fn try_from_data(
        data: IkaTransactionBlockData,
        status: Option<OperationStatus>,
    ) -> Result<Self, anyhow::Error> {
        let sender = *data.sender();
        Ok(Self::new(Self::from_transaction(
            data.transaction().clone(),
            sender,
            status,
        )?))
    }
}
impl Operations {
    pub async fn try_from_response(
        response: IkaTransactionBlockResponse,
        cache: &CoinMetadataCache,
    ) -> Result<Self, Error> {
        let tx = response
            .transaction
            .ok_or_else(|| anyhow!("Response input should not be empty"))?;
        let sender = *tx.data.sender();
        let effect = response
            .effects
            .ok_or_else(|| anyhow!("Response effects should not be empty"))?;
        let gas_owner = effect.gas_object().owner.get_owner_address()?;
        let gas_summary = effect.gas_cost_summary();
        let gas_used = gas_summary.storage_rebate as i128
            - gas_summary.storage_cost as i128
            - gas_summary.computation_cost as i128;

        let status = Some(effect.into_status().into());
        let ops = Operations::try_from_data(tx.data, status)?;
        let ops = ops.into_iter();

        // We will need to subtract the operation amounts from the actual balance
        // change amount extracted from event to prevent double counting.
        let mut accounted_balances =
            ops.as_ref()
                .iter()
                .fold(HashMap::new(), |mut balances, op| {
                    if let (Some(acc), Some(amount), Some(OperationStatus::Success)) =
                        (&op.account, &op.amount, &op.status)
                    {
                        *balances
                            .entry((acc.address, amount.clone().currency))
                            .or_default() -= amount.value;
                    }
                    balances
                });

        let mut principal_amounts = 0;
        let mut reward_amounts = 0;
        // Extract balance change from unstake events

        if let Some(events) = response.events {
            for event in events.data {
                if is_unstake_event(&event.type_) {
                    let principal_amount = event
                        .parsed_json
                        .pointer("/principal_amount")
                        .and_then(|v| v.as_str())
                        .and_then(|v| i128::from_str(v).ok());
                    let reward_amount = event
                        .parsed_json
                        .pointer("/reward_amount")
                        .and_then(|v| v.as_str())
                        .and_then(|v| i128::from_str(v).ok());
                    if let (Some(principal_amount), Some(reward_amount)) =
                        (principal_amount, reward_amount)
                    {
                        principal_amounts += principal_amount;
                        reward_amounts += reward_amount;
                    }
                }
            }
        }
        let staking_balance = if principal_amounts != 0 {
            *accounted_balances.entry((sender, IKA.clone())).or_default() -= principal_amounts;
            *accounted_balances.entry((sender, IKA.clone())).or_default() -= reward_amounts;
            vec![
                Operation::stake_principle(status, sender, principal_amounts),
                Operation::stake_reward(status, sender, reward_amounts),
            ]
        } else {
            vec![]
        };

        let mut balance_changes = vec![];

        for balance_change in &response
            .balance_changes
            .ok_or_else(|| anyhow!("Response balance changes should not be empty."))?
        {
            if let Ok(currency) = cache.get_currency(&balance_change.coin_type).await {
                if !currency.symbol.is_empty() {
                    balance_changes.push((balance_change.clone(), currency));
                }
            }
        }

        // Extract coin change operations from balance changes
        let coin_change_operations = Self::process_balance_change(
            gas_owner,
            gas_used,
            balance_changes,
            status,
            accounted_balances,
        );

        let ops: Operations = ops
            .into_iter()
            .chain(coin_change_operations)
            .chain(staking_balance)
            .collect();

        // This is a workaround for the payCoin cases that are mistakenly considered to be payIka operations
        // In this case we remove any irrelevant, IKA specific operation entries that sum up to 0 balance changes per address
        // and keep only the actual entries for the right coin type transfers, as they have been extracted from the transaction's
        // balance changes section.
        let mutually_cancelling_balances: HashMap<_, _> = ops
            .clone()
            .into_iter()
            .fold(
                HashMap::new(),
                |mut balances: HashMap<(IkaAddress, Currency), i128>, op| {
                    if let (Some(acc), Some(amount), Some(OperationStatus::Success)) =
                        (&op.account, &op.amount, &op.status)
                    {
                        if op.type_ != OperationType::Gas {
                            *balances
                                .entry((acc.address, amount.clone().currency))
                                .or_default() += amount.value;
                        }
                    }
                    balances
                },
            )
            .into_iter()
            .filter(|balance| {
                let (_, amount) = balance;
                *amount == 0
            })
            .collect();

        let ops: Operations = ops
            .clone()
            .into_iter()
            .filter(|op| {
                if let (Some(acc), Some(amount)) = (&op.account, &op.amount) {
                    return op.type_ == OperationType::Gas
                        || !mutually_cancelling_balances
                            .contains_key(&(acc.address, amount.clone().currency));
                }
                true
            })
            .collect();

        Ok(ops)
    }
}

fn is_unstake_event(tag: &StructTag) -> bool {
    tag.address == IKA_SYSTEM_ADDRESS
        && tag.module.as_ident_str() == ident_str!("validator")
        && tag.name.as_ident_str() == ident_str!("UnstakingRequestEvent")
}

impl TryFrom<TransactionData> for Operations {
    type Error = Error;
    fn try_from(data: TransactionData) -> Result<Self, Self::Error> {
        struct NoOpsModuleResolver;
        impl ModuleResolver for NoOpsModuleResolver {
            type Error = Error;
            fn get_module(&self, _id: &ModuleId) -> Result<Option<Vec<u8>>, Self::Error> {
                Ok(None)
            }
        }
        // Rosetta don't need the call args to be parsed into readable format
        Ok(Operations::try_from_data(
            IkaTransactionBlockData::try_from(data, &&mut NoOpsModuleResolver)?,
            None,
        )?)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Operation {
    operation_identifier: OperationIdentifier,
    #[serde(rename = "type")]
    pub type_: OperationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coin_change: Option<CoinChange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OperationMetadata>,
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        self.operation_identifier == other.operation_identifier
            && self.type_ == other.type_
            && self.account == other.account
            && self.amount == other.amount
            && self.coin_change == other.coin_change
            && self.metadata == other.metadata
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum OperationMetadata {
    GenericTransaction(IkaTransactionBlockKind),
    Stake { validator: IkaAddress },
    WithdrawStake { stake_ids: Vec<ObjectID> },
}

impl Operation {
    fn generic_op(
        status: Option<OperationStatus>,
        sender: IkaAddress,
        tx: IkaTransactionBlockKind,
    ) -> Self {
        Operation {
            operation_identifier: Default::default(),
            type_: (&tx).into(),
            status,
            account: Some(sender.into()),
            amount: None,
            coin_change: None,
            metadata: Some(OperationMetadata::GenericTransaction(tx)),
        }
    }

    pub fn genesis(index: u64, sender: IkaAddress, coin: GasCoin) -> Self {
        Operation {
            operation_identifier: index.into(),
            type_: OperationType::Genesis,
            status: Some(OperationStatus::Success),
            account: Some(sender.into()),
            amount: Some(Amount::new(coin.value().into(), None)),
            coin_change: Some(CoinChange {
                coin_identifier: CoinIdentifier {
                    identifier: CoinID {
                        id: *coin.id(),
                        version: SequenceNumber::new(),
                    },
                },
                coin_action: CoinAction::CoinCreated,
            }),
            metadata: None,
        }
    }

    fn pay_ika(status: Option<OperationStatus>, address: IkaAddress, amount: i128) -> Self {
        Operation {
            operation_identifier: Default::default(),
            type_: OperationType::PayIka,
            status,
            account: Some(address.into()),
            amount: Some(Amount::new(amount, None)),
            coin_change: None,
            metadata: None,
        }
    }

    fn pay_coin(
        status: Option<OperationStatus>,
        address: IkaAddress,
        amount: i128,
        currency: Option<Currency>,
    ) -> Self {
        Operation {
            operation_identifier: Default::default(),
            type_: OperationType::PayCoin,
            status,
            account: Some(address.into()),
            amount: Some(Amount::new(amount, currency)),
            coin_change: None,
            metadata: None,
        }
    }

    fn balance_change(
        status: Option<OperationStatus>,
        addr: IkaAddress,
        amount: i128,
        currency: Currency,
    ) -> Self {
        Self {
            operation_identifier: Default::default(),
            type_: OperationType::IkaBalanceChange,
            status,
            account: Some(addr.into()),
            amount: Some(Amount::new(amount, Some(currency))),
            coin_change: None,
            metadata: None,
        }
    }
    fn gas(addr: IkaAddress, amount: i128) -> Self {
        Self {
            operation_identifier: Default::default(),
            type_: OperationType::Gas,
            status: Some(OperationStatus::Success),
            account: Some(addr.into()),
            amount: Some(Amount::new(amount, None)),
            coin_change: None,
            metadata: None,
        }
    }
    fn stake_reward(status: Option<OperationStatus>, addr: IkaAddress, amount: i128) -> Self {
        Self {
            operation_identifier: Default::default(),
            type_: OperationType::StakeReward,
            status,
            account: Some(addr.into()),
            amount: Some(Amount::new(amount, None)),
            coin_change: None,
            metadata: None,
        }
    }
    fn stake_principle(status: Option<OperationStatus>, addr: IkaAddress, amount: i128) -> Self {
        Self {
            operation_identifier: Default::default(),
            type_: OperationType::StakePrinciple,
            status,
            account: Some(addr.into()),
            amount: Some(Amount::new(amount, None)),
            coin_change: None,
            metadata: None,
        }
    }
}
