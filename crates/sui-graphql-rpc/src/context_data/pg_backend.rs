// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{
    db_backend::{BalanceQuery, Explain, Explained, GenericQueryBuilder},
    db_data_provider::{DbValidationError, PageLimit, TypeFilterError},
};
use crate::{
    context_data::db_data_provider::PgManager,
    error::Error,
    types::{
        event::EventFilter, object::ObjectFilter, sui_address::SuiAddress,
        transaction_block::TransactionBlockFilter,
    },
};
use async_trait::async_trait;
use diesel::{
    pg::Pg,
    query_builder::{AstPass, QueryFragment},
    BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
    TextExpressionMethods,
};
use std::str::FromStr;
use sui_indexer::{
    schema_v2::{
        display, events, objects, transactions, tx_calls, tx_changed_objects, tx_input_objects,
        tx_recipients, tx_senders,
    },
    types_v2::OwnerType,
};
use sui_types::parse_sui_struct_tag;
use tap::TapFallible;
use tracing::{info, warn};

pub(crate) const EXPLAIN_COSTING_LOG_TARGET: &str = "gql-explain-costing";

pub(crate) struct PgQueryBuilder;

impl GenericQueryBuilder<Pg> for PgQueryBuilder {
    fn get_tx_by_digest(digest: Vec<u8>) -> transactions::BoxedQuery<'static, Pg> {
        transactions::dsl::transactions
            .filter(transactions::dsl::transaction_digest.eq(digest))
            .into_boxed()
    }
    fn get_obj(address: Vec<u8>, version: Option<i64>) -> objects::BoxedQuery<'static, Pg> {
        let mut query = objects::dsl::objects.into_boxed();
        query = query.filter(objects::dsl::object_id.eq(address));

        if let Some(version) = version {
            query = query.filter(objects::dsl::object_version.eq(version));
        }
        query
    }
    fn get_obj_by_type(object_type: String) -> objects::BoxedQuery<'static, Pg> {
        objects::dsl::objects
            .filter(objects::dsl::object_type.eq(object_type))
            .limit(1) // Fetches for a single object and as such has a limit of 1
            .into_boxed()
    }

    fn get_display_by_obj_type(object_type: String) -> display::BoxedQuery<'static, Pg> {
        display::dsl::display
            .filter(display::dsl::object_type.eq(object_type))
            .limit(1)
            .into_boxed()
    }

    fn multi_get_txs(
        cursor: Option<i64>,
        descending_order: bool,
        limit: PageLimit,
        filter: Option<TransactionBlockFilter>,
        after_tx_seq_num: Option<i64>,
        before_tx_seq_num: Option<i64>,
    ) -> Result<transactions::BoxedQuery<'static, Pg>, Error> {
        let mut query = transactions::dsl::transactions.into_boxed();

        if let Some(cursor_val) = cursor {
            if descending_order {
                let filter_value =
                    before_tx_seq_num.map_or(cursor_val, |b| std::cmp::min(b, cursor_val));
                query = query.filter(transactions::dsl::tx_sequence_number.lt(filter_value));
            } else {
                let filter_value =
                    after_tx_seq_num.map_or(cursor_val, |a| std::cmp::max(a, cursor_val));
                query = query.filter(transactions::dsl::tx_sequence_number.gt(filter_value));
            }
        } else {
            if let Some(av) = after_tx_seq_num {
                query = query.filter(transactions::dsl::tx_sequence_number.gt(av));
            }
            if let Some(bv) = before_tx_seq_num {
                query = query.filter(transactions::dsl::tx_sequence_number.lt(bv));
            }
        }

        if descending_order {
            query = query.order(transactions::dsl::tx_sequence_number.desc());
        } else {
            query = query.order(transactions::dsl::tx_sequence_number.asc());
        }

        query = query.limit(limit.value() + 1);

        if let Some(filter) = filter {
            // Filters for transaction table
            // at_checkpoint mutually exclusive with before_ and after_checkpoint
            if let Some(checkpoint) = filter.at_checkpoint {
                query = query
                    .filter(transactions::dsl::checkpoint_sequence_number.eq(checkpoint as i64));
            }
            if let Some(transaction_ids) = filter.transaction_ids {
                let digests: Vec<_> = transaction_ids.iter().map(|d| d.to_vec()).collect();
                query = query.filter(transactions::dsl::transaction_digest.eq_any(digests));
            }

            // Queries on foreign tables
            match (filter.package, filter.module, filter.function) {
                (Some(p), None, None) => {
                    let subquery = tx_calls::dsl::tx_calls
                        .filter(tx_calls::dsl::package.eq(p.into_vec()))
                        .select(tx_calls::dsl::tx_sequence_number);

                    query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
                }
                (Some(p), Some(m), None) => {
                    let subquery = tx_calls::dsl::tx_calls
                        .filter(tx_calls::dsl::package.eq(p.into_vec()))
                        .filter(tx_calls::dsl::module.eq(m))
                        .select(tx_calls::dsl::tx_sequence_number);

                    query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
                }
                (Some(p), Some(m), Some(f)) => {
                    let subquery = tx_calls::dsl::tx_calls
                        .filter(tx_calls::dsl::package.eq(p.into_vec()))
                        .filter(tx_calls::dsl::module.eq(m))
                        .filter(tx_calls::dsl::func.eq(f))
                        .select(tx_calls::dsl::tx_sequence_number);

                    query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
                }
                _ => {}
            }

            if let Some(signer) = filter.sign_address {
                if let Some(sender) = filter.sent_address {
                    let subquery = tx_senders::dsl::tx_senders
                        .filter(
                            tx_senders::dsl::sender
                                .eq(signer.into_vec())
                                .or(tx_senders::dsl::sender.eq(sender.into_vec())),
                        )
                        .select(tx_senders::dsl::tx_sequence_number);

                    query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
                } else {
                    let subquery = tx_senders::dsl::tx_senders
                        .filter(tx_senders::dsl::sender.eq(signer.into_vec()))
                        .select(tx_senders::dsl::tx_sequence_number);

                    query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
                }
            } else if let Some(sender) = filter.sent_address {
                let subquery = tx_senders::dsl::tx_senders
                    .filter(tx_senders::dsl::sender.eq(sender.into_vec()))
                    .select(tx_senders::dsl::tx_sequence_number);

                query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
            }
            if let Some(recipient) = filter.recv_address {
                let subquery = tx_recipients::dsl::tx_recipients
                    .filter(tx_recipients::dsl::recipient.eq(recipient.into_vec()))
                    .select(tx_recipients::dsl::tx_sequence_number);

                query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
            }
            if filter.paid_address.is_some() {
                return Err(Error::Internal(
                    "Paid address filter not supported".to_string(),
                ));
            }

            if let Some(input_object) = filter.input_object {
                let subquery = tx_input_objects::dsl::tx_input_objects
                    .filter(tx_input_objects::dsl::object_id.eq(input_object.into_vec()))
                    .select(tx_input_objects::dsl::tx_sequence_number);

                query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
            }
            if let Some(changed_object) = filter.changed_object {
                let subquery = tx_changed_objects::dsl::tx_changed_objects
                    .filter(tx_changed_objects::dsl::object_id.eq(changed_object.into_vec()))
                    .select(tx_changed_objects::dsl::tx_sequence_number);

                query = query.filter(transactions::dsl::tx_sequence_number.eq_any(subquery));
            }
        };

        Ok(query)
    }
    fn multi_get_coins(
        before: Option<Vec<u8>>,
        after: Option<Vec<u8>>,
        limit: PageLimit,
        address: Option<Vec<u8>>,
        coin_type: String,
    ) -> objects::BoxedQuery<'static, Pg> {
        let mut query = order_objs(before, after, &limit);
        query = query.limit(limit.value() + 1);

        if let Some(address) = address {
            query = query
                .filter(objects::dsl::owner_id.eq(address))
                // Leverage index on objects table
                .filter(objects::dsl::owner_type.eq(OwnerType::Address as i16));
        }
        query = query.filter(objects::dsl::coin_type.eq(coin_type));

        query
    }
    fn multi_get_objs(
        before: Option<Vec<u8>>,
        after: Option<Vec<u8>>,
        limit: PageLimit,
        filter: Option<ObjectFilter>,
        owner_type: Option<OwnerType>,
    ) -> Result<objects::BoxedQuery<'static, Pg>, Error> {
        let mut query = order_objs(before, after, &limit);
        query = query.limit(limit.value() + 1);

        let Some(filter) = filter else {
            return Ok(query);
        };

        if let Some(object_ids) = filter.object_ids {
            query = query.filter(
                objects::dsl::object_id.eq_any(
                    object_ids
                        .into_iter()
                        .map(|id| id.into_vec())
                        .collect::<Vec<_>>(),
                ),
            );
        }

        if let Some(owner) = filter.owner {
            query = query.filter(objects::dsl::owner_id.eq(owner.into_vec()));

            match owner_type {
                Some(OwnerType::Address) => {
                    query = query.filter(objects::dsl::owner_type.eq(OwnerType::Address as i16));
                }
                Some(OwnerType::Object) => {
                    query = query.filter(objects::dsl::owner_type.eq(OwnerType::Object as i16));
                }
                None => {
                    query = query.filter(
                        objects::dsl::owner_type
                            .eq(OwnerType::Address as i16)
                            .or(objects::dsl::owner_type.eq(OwnerType::Object as i16)),
                    );
                }
                _ => Err(DbValidationError::InvalidOwnerType)?,
            }
        }

        if let Some(object_type) = filter.type_ {
            let format = "package[::module[::type[<type_params>]]]";
            let parts: Vec<_> = object_type.splitn(3, "::").collect();

            if parts.iter().any(|&part| part.is_empty()) {
                return Err(DbValidationError::InvalidType(
                    TypeFilterError::MissingComponents(object_type, format).to_string(),
                ))?;
            }

            if parts.len() == 1 {
                // We check for a leading 0x to determine if it is an address
                // And otherwise process it as a primitive type
                if parts[0].starts_with("0x") {
                    let package = SuiAddress::from_str(parts[0])
                        .map_err(|e| DbValidationError::InvalidType(e.to_string()))?;
                    query = query.filter(objects::dsl::object_type.like(format!("{}::%", package)));
                } else {
                    query = query.filter(objects::dsl::object_type.eq(parts[0].to_string()));
                }
            } else if parts.len() == 2 {
                // Only package addresses are allowed if there are two or more parts
                let package = SuiAddress::from_str(parts[0])
                    .map_err(|e| DbValidationError::InvalidType(e.to_string()))?;
                query = query.filter(
                    objects::dsl::object_type.like(format!("{}::{}::%", package, parts[1])),
                );
            } else if parts.len() == 3 {
                let validated_type = parse_sui_struct_tag(&object_type)
                    .map_err(|e| DbValidationError::InvalidType(e.to_string()))?;

                if validated_type.type_params.is_empty() {
                    query = query.filter(
                        objects::dsl::object_type
                            .like(format!(
                                "{}<%",
                                validated_type.to_canonical_string(/* with_prefix */ true)
                            ))
                            .or(objects::dsl::object_type
                                .eq(validated_type.to_canonical_string(/* with_prefix */ true))),
                    );
                } else {
                    query = query.filter(
                        objects::dsl::object_type
                            .eq(validated_type.to_canonical_string(/* with_prefix */ true)),
                    );
                }
            } else {
                return Err(DbValidationError::InvalidType(
                    TypeFilterError::TooManyComponents(object_type, 3, format).to_string(),
                )
                .into());
            }
        }

        Ok(query)
    }
    fn multi_get_balances(address: Vec<u8>) -> BalanceQuery<'static, Pg> {
        let query = objects::dsl::objects
            .group_by(objects::dsl::coin_type)
            .select((
                diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>>(
                    "CAST(SUM(coin_balance) AS BIGINT)",
                ),
                diesel::dsl::sql::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>>(
                    "COUNT(*)",
                ),
                objects::dsl::coin_type,
            ))
            .filter(objects::dsl::owner_id.eq(address))
            .filter(objects::dsl::owner_type.eq(OwnerType::Address as i16))
            .filter(objects::dsl::coin_type.is_not_null())
            .into_boxed();

        query
    }
    fn get_balance(address: Vec<u8>, coin_type: String) -> BalanceQuery<'static, Pg> {
        let query = PgQueryBuilder::multi_get_balances(address);
        query.filter(objects::dsl::coin_type.eq(coin_type))
    }
    fn multi_get_events(
        before: Option<(i64, i64)>,
        after: Option<(i64, i64)>,
        limit: PageLimit,
        filter: Option<EventFilter>,
    ) -> Result<events::BoxedQuery<'static, Pg>, Error> {
        let mut query = order_events(before, after, &limit);
        query = query.limit(limit.value() + 1);

        let Some(filter) = filter else {
            return Ok(query);
        };

        if let Some(sender) = filter.sender {
            // Construct a subquery to filter on senders - this is because we do not have an index on the senders column.
            let subquery = tx_senders::dsl::tx_senders
                .filter(tx_senders::dsl::sender.eq(sender.into_vec()))
                .select(tx_senders::dsl::tx_sequence_number);

            query = query.filter(events::dsl::tx_sequence_number.eq_any(subquery));
        }

        if let Some(digest) = filter.transaction_digest {
            let subquery = transactions::dsl::transactions
                .filter(transactions::dsl::transaction_digest.eq(digest.to_vec()))
                .select(transactions::dsl::tx_sequence_number);

            query = query.filter(events::dsl::tx_sequence_number.eq_any(subquery));
        }

        // Filters on the package and/ or module that emitted some event
        if let Some(pm) = filter.emitting_module {
            let format = "package[::module]";
            let parts: Vec<_> = pm.splitn(2, "::").collect();

            if parts.iter().any(|&part| part.is_empty()) {
                return Err(DbValidationError::InvalidType(
                    TypeFilterError::MissingComponents(pm, format).to_string(),
                ))?;
            }

            let p = SuiAddress::from_str(parts[0])
                .map_err(|e| DbValidationError::InvalidType(e.to_string()))?;

            match parts.len() {
                1 => {
                    query = query.filter(events::dsl::package.eq(p.into_vec()));
                }
                2 => {
                    query = query.filter(events::dsl::package.eq(p.into_vec()));
                    query = query.filter(events::dsl::module.eq(parts[1].to_string()));
                }
                _ => {
                    return Err(DbValidationError::InvalidType(
                        TypeFilterError::TooManyComponents(pm, 2, format).to_string(),
                    )
                    .into());
                }
            }
        }

        // Filters on the event type
        if let Some(event_type) = filter.event_type {
            let parts: Vec<_> = event_type.splitn(3, "::").collect();

            if parts.iter().any(|&part| part.is_empty()) {
                return Err(DbValidationError::InvalidType(
                    TypeFilterError::MissingComponents(
                        event_type,
                        "package[::module[::type[<type_params>]]]",
                    )
                    .to_string(),
                ))?;
            }

            let p = SuiAddress::from_str(parts[0])
                .map_err(|e| DbValidationError::InvalidType(e.to_string()))?;

            match parts.len() {
                1 => query = query.filter(events::dsl::event_type.like(format!("{}::%", p))),
                2 => {
                    query = query
                        .filter(events::dsl::event_type.like(format!("{}::{}::%", p, parts[1])))
                }
                3 => {
                    let validated_type = parse_sui_struct_tag(&event_type)
                        .map_err(|e| DbValidationError::InvalidType(e.to_string()))?;

                    if validated_type.type_params.is_empty() {
                        query = query.filter(
                            events::dsl::event_type
                                .like(format!(
                                    "{}<%",
                                    validated_type.to_canonical_string(/* with_prefix */ true)
                                ))
                                .or(events::dsl::event_type
                                    .eq(validated_type
                                        .to_canonical_string(/* with_prefix */ true))),
                        );
                    } else {
                        query = query.filter(
                            events::dsl::event_type
                                .eq(validated_type.to_canonical_string(/* with_prefix */ true)),
                        );
                    }
                }
                _ => {
                    return Err(DbValidationError::InvalidType(
                        TypeFilterError::TooManyComponents(
                            event_type,
                            3,
                            "package[::module[::type[<type_params>]]]",
                        )
                        .to_string(),
                    )
                    .into());
                }
            }
        }

        Ok(query)
    }
}

/// Allows methods like load(), get_result(), etc. on an Explained query
impl<T> RunQueryDsl<PgConnection> for Explained<T> {}

/// Implement logic for prefixing queries with "EXPLAIN"
impl<T> QueryFragment<Pg> for Explained<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("EXPLAIN (FORMAT JSON) ");
        self.query.walk_ast(out.reborrow())?;
        Ok(())
    }
}

#[async_trait]
pub trait PgQueryExecutor {
    async fn run_query_async<T, E, F>(&self, query: F) -> Result<T, Error>
    where
        F: FnOnce(&mut PgConnection) -> Result<T, E> + Send + 'static,
        E: From<diesel::result::Error> + std::error::Error + Send + 'static,
        T: Send + 'static;

    async fn run_query_async_with_cost<T, Q, QResult, EF, E, F>(
        &self,
        mut query_builder_fn: Q,
        execute_fn: EF,
    ) -> Result<T, Error>
    where
        Q: FnMut() -> Result<QResult, Error> + Send + 'static,
        QResult: diesel::query_builder::QueryFragment<diesel::pg::Pg>
            + diesel::query_builder::Query
            + diesel::query_builder::QueryId
            + Send
            + 'static,
        EF: FnOnce(QResult) -> F + Send + 'static,
        F: FnOnce(&mut PgConnection) -> Result<T, E> + Send + 'static,
        E: From<diesel::result::Error> + std::error::Error + Send + 'static,
        T: Send + 'static;
}

#[async_trait]
impl PgQueryExecutor for PgManager {
    async fn run_query_async<T, E, F>(&self, query: F) -> Result<T, Error>
    where
        F: FnOnce(&mut PgConnection) -> Result<T, E> + Send + 'static,
        E: From<diesel::result::Error> + std::error::Error + Send + 'static,
        T: Send + 'static,
    {
        self.inner
            .run_query_async(query)
            .await
            .map_err(|e| Error::Internal(e.to_string()))
    }

    /// Takes a query_builder_fn that returns Result<QueryFragment> and a lambda to execute the query
    /// Spawns a blocking task that determines the cost of the query fragment
    /// And if within limits, then executes the query
    async fn run_query_async_with_cost<T, Q, QResult, EF, E, F>(
        &self,
        mut query_builder_fn: Q,
        execute_fn: EF,
    ) -> Result<T, Error>
    where
        Q: FnMut() -> Result<QResult, Error> + Send + 'static,
        QResult: diesel::query_builder::QueryFragment<diesel::pg::Pg>
            + diesel::query_builder::Query
            + diesel::query_builder::QueryId
            + Send
            + 'static,
        EF: FnOnce(QResult) -> F + Send + 'static,
        F: FnOnce(&mut PgConnection) -> Result<T, E> + Send + 'static,
        E: From<diesel::result::Error> + std::error::Error + Send + 'static,
        T: Send + 'static,
    {
        let max_db_query_cost = self.limits.max_db_query_cost;
        self.inner
            .spawn_blocking(move |this| {
                let query = query_builder_fn()?;
                let explain_result: Option<String> = this
                    .run_query(|conn| query.explain().get_result(conn))
                    .tap_err(|e| {
                        warn!(
                            target: EXPLAIN_COSTING_LOG_TARGET,
                            "Failed to get explain result: {}", e
                        )
                    })
                    .ok(); // Fine to not propagate this error as explain-based costing is not critical today

                if let Some(explain_result) = explain_result {
                    let cost = extract_cost(&explain_result)
                        .tap_err(|e| {
                            warn!(
                                target: EXPLAIN_COSTING_LOG_TARGET,
                                "Failed to get cost from explain result: {}", e
                            )
                        })
                        .ok(); // Fine to not propagate this error as explain-based costing is not critical today

                    if let Some(cost) = cost {
                        if cost > max_db_query_cost as f64 {
                            warn!(
                                target: EXPLAIN_COSTING_LOG_TARGET,
                                cost,
                                max_db_query_cost,
                                exceeds = true
                            );
                        } else {
                            info!(
                                target: EXPLAIN_COSTING_LOG_TARGET,
                                cost,
                            );
                        }
                    }
                }

                let query = query_builder_fn()?;
                let execute_closure = execute_fn(query);
                this.run_query(execute_closure)
                    .map_err(|e| Error::Internal(e.to_string()))
            })
            .await
    }
}

pub fn extract_cost(explain_result: &str) -> Result<f64, Error> {
    let parsed: serde_json::Value =
        serde_json::from_str(explain_result).map_err(|e| Error::Internal(e.to_string()))?;
    if let Some(cost) = parsed
        .get(0)
        .and_then(|entry| entry.get("Plan"))
        .and_then(|plan| plan.get("Total Cost"))
        .and_then(|cost| cost.as_f64())
    {
        Ok(cost)
    } else {
        Err(Error::Internal(
            "Failed to get cost from query plan".to_string(),
        ))
    }
}

fn order_objs(
    before: Option<Vec<u8>>,
    after: Option<Vec<u8>>,
    limit: &PageLimit,
) -> objects::BoxedQuery<'static, Pg> {
    let mut query = objects::dsl::objects.into_boxed();
    match limit {
        PageLimit::First(_) => {
            if let Some(after) = after {
                query = query.filter(objects::dsl::object_id.gt(after));
            }
            query = query.order(objects::dsl::object_id.asc());
        }
        PageLimit::Last(_) => {
            if let Some(before) = before {
                query = query.filter(objects::dsl::object_id.lt(before));
            }
            query = query.order(objects::dsl::object_id.desc());
        }
    }
    query
}

fn order_events(
    before: Option<(i64, i64)>,
    after: Option<(i64, i64)>,
    limit: &PageLimit,
) -> events::BoxedQuery<'static, Pg> {
    let mut query = events::dsl::events.into_boxed();
    match limit {
        PageLimit::First(_) => {
            if let Some(after) = after {
                query = query.filter(
                    events::dsl::tx_sequence_number
                        .gt(after.0)
                        .or(events::dsl::tx_sequence_number
                            .eq(after.0)
                            .and(events::dsl::event_sequence_number.gt(after.1))),
                );
            }
            query = query
                .order(events::dsl::tx_sequence_number.asc())
                .then_order_by(events::dsl::event_sequence_number.asc());
        }
        PageLimit::Last(_) => {
            if let Some(before) = before {
                query = query.filter(
                    events::dsl::tx_sequence_number.lt(before.0).or(
                        events::dsl::tx_sequence_number
                            .eq(before.0)
                            .and(events::dsl::event_sequence_number.lt(before.1)),
                    ),
                );
            }
            query = query
                .order(events::dsl::tx_sequence_number.desc())
                .then_order_by(events::dsl::event_sequence_number.desc());
        }
    }
    query
}

pub(crate) type QueryBuilder = PgQueryBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_json() {
        let explain_result = "invalid json";
        let result = extract_cost(explain_result);
        assert!(matches!(result, Err(Error::Internal(_))));
    }

    #[test]
    fn test_missing_entry_at_0() {
        let explain_result = "[]";
        let result = extract_cost(explain_result);
        assert!(matches!(result, Err(Error::Internal(_))));
    }

    #[test]
    fn test_missing_plan() {
        let explain_result = r#"[{}]"#;
        let result = extract_cost(explain_result);
        assert!(matches!(result, Err(Error::Internal(_))));
    }

    #[test]
    fn test_missing_total_cost() {
        let explain_result = r#"[{"Plan": {}}]"#;
        let result = extract_cost(explain_result);
        assert!(matches!(result, Err(Error::Internal(_))));
    }

    #[test]
    fn test_failure_on_conversion_to_f64() {
        let explain_result = r#"[{"Plan": {"Total Cost": "string_instead_of_float"}}]"#;
        let result = extract_cost(explain_result);
        assert!(matches!(result, Err(Error::Internal(_))));
    }

    #[test]
    fn test_happy_scenario() {
        let explain_result = r#"[{"Plan": {"Total Cost": 1.0}}]"#;
        let result = extract_cost(explain_result).unwrap();
        assert_eq!(result, 1.0);
    }
}
