extern crate core;

extern crate lazy_static;

mod abi;
mod constants;
mod db;
mod keyer;
mod looksrare;
mod macros;
mod math;
mod pb;
mod price;
mod rpc;
mod seaport;
mod types;
mod utils;

use std::str::FromStr;

use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreAddBigInt};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::pb::eth::v2 as ethpb;
use utils::bytes_to_hex;

use pb::marketplaces::{Trade, Transaction, Transactions};

#[substreams::handlers::map]
fn map_marketplace_orders(block: ethpb::Block) -> Result<Transactions, substreams::errors::Error> {
    let mut out = Transactions {
        transactions: vec![],
    };

    for tx in block.transactions() {
        if tx.status != 1 {
            continue;
        }

        let mut transaction = Transaction {
            tx_hash: bytes_to_hex(&tx.hash),
            block_number: block.number,
            block_hash: bytes_to_hex(&block.hash),
            block_timestamp: block.timestamp_seconds(),
            origin: bytes_to_hex(&tx.from),
            trades: vec![],
        };

        for log in tx.receipt().logs() {
            if let Some(trade) = Trade::match_and_decode(log) {
                transaction.trades.push(trade);
            }
        }
        if !transaction.trades.is_empty() {
            out.transactions.push(transaction);
        }
    }

    Ok(out)
}

#[substreams::handlers::store]
fn store_volumes(transactions: Transactions, out: StoreAddBigInt) {
    for transaction in transactions.transactions {
        // Add volumes for seaport orders
        for trade in transaction.trades {
            let collection_data_key = keyer::store_volumes::collection::construct(
                &trade.contract_address,
                &trade.marketplace,
                &trade.payment_token,
            );

            out.add_many(
                trade.ordinal,
                &vec![collection_data_key],
                BigInt::from_str(&trade.price).unwrap(),
            )
        }
    }
}

#[substreams::handlers::map]
fn map_collection_data_entity_changes(
    clock: Clock,
    volume_deltas: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut entity_changes: EntityChanges = Default::default();
    //	rpc::get_weth_name();
    db::collection_volume_entity_change(&mut entity_changes, volume_deltas, clock.number);

    return Ok(entity_changes);
}
