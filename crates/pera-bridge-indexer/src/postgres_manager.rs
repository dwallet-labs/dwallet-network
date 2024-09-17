// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::models::PeraProgressStore;
use crate::models::TokenTransfer as DBTokenTransfer;
use crate::schema::pera_progress_store::txn_digest;
use crate::schema::{pera_error_transactions, token_transfer_data};
use crate::{schema, schema::token_transfer, ProcessedTxnData};
use diesel::result::Error;
use diesel::BoolExpressionMethods;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use pera_types::digests::TransactionDigest;

pub(crate) type PgPool = Pool<ConnectionManager<PgConnection>>;

const PERA_PROGRESS_STORE_DUMMY_KEY: i32 = 1;

pub fn get_connection_pool(database_url: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build Postgres DB connection pool")
}

// TODO: add retry logic
pub fn write(pool: &PgPool, token_txns: Vec<ProcessedTxnData>) -> Result<(), anyhow::Error> {
    if token_txns.is_empty() {
        return Ok(());
    }
    let (transfers, data, errors) = token_txns.iter().fold(
        (vec![], vec![], vec![]),
        |(mut transfers, mut data, mut errors), d| {
            match d {
                ProcessedTxnData::TokenTransfer(t) => {
                    transfers.push(t.to_db());
                    if let Some(d) = t.to_data_maybe() {
                        data.push(d)
                    }
                }
                ProcessedTxnData::Error(e) => errors.push(e.to_db()),
            }
            (transfers, data, errors)
        },
    );

    let connection = &mut pool.get()?;
    connection.transaction(|conn| {
        diesel::insert_into(token_transfer_data::table)
            .values(&data)
            .on_conflict_do_nothing()
            .execute(conn)?;
        diesel::insert_into(token_transfer::table)
            .values(&transfers)
            .on_conflict_do_nothing()
            .execute(conn)?;
        diesel::insert_into(pera_error_transactions::table)
            .values(&errors)
            .on_conflict_do_nothing()
            .execute(conn)
    })?;
    Ok(())
}

pub fn update_pera_progress_store(
    pool: &PgPool,
    tx_digest: TransactionDigest,
) -> Result<(), anyhow::Error> {
    let mut conn = pool.get()?;
    diesel::insert_into(schema::pera_progress_store::table)
        .values(&PeraProgressStore {
            id: PERA_PROGRESS_STORE_DUMMY_KEY,
            txn_digest: tx_digest.inner().to_vec(),
        })
        .on_conflict(schema::pera_progress_store::dsl::id)
        .do_update()
        .set(txn_digest.eq(tx_digest.inner().to_vec()))
        .execute(&mut conn)?;
    Ok(())
}

pub fn read_pera_progress_store(pool: &PgPool) -> anyhow::Result<Option<TransactionDigest>> {
    let mut conn = pool.get()?;
    let val: Option<PeraProgressStore> = crate::schema::pera_progress_store::dsl::pera_progress_store
        .select(PeraProgressStore::as_select())
        .first(&mut conn)
        .optional()?;
    match val {
        Some(val) => Ok(Some(TransactionDigest::try_from(
            val.txn_digest.as_slice(),
        )?)),
        None => Ok(None),
    }
}

pub fn get_latest_eth_token_transfer(
    pool: &PgPool,
    finalized: bool,
) -> Result<Option<DBTokenTransfer>, Error> {
    use crate::schema::token_transfer::dsl::*;

    let connection = &mut pool.get().unwrap();

    if finalized {
        token_transfer
            .filter(data_source.eq("ETH").and(status.eq("Deposited")))
            .order(block_height.desc())
            .first::<DBTokenTransfer>(connection)
            .optional()
    } else {
        token_transfer
            .filter(status.eq("DepositedUnfinalized"))
            .order(block_height.desc())
            .first::<DBTokenTransfer>(connection)
            .optional()
    }
}
