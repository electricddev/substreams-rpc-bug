use substreams::store::{DeltaBigInt, Deltas};
use substreams_entity_change::pb::entity::{
    entity_change::Operation as EcOperation, EntityChanges,
};

use crate::{keyer, price, rpc, utils::get_ordinal};

pub fn collection_volume_entity_change(
    entity_changes: &mut EntityChanges,
    deltas: Deltas<DeltaBigInt>,
    block_number: u64,
) {
    for delta in deltas.deltas {
        if !delta.key.starts_with("collection:") {
            continue;
        }

        let (contract_address, marketplace, payment_token) =
            keyer::store_volumes::collection::destruct(&delta.key);
        // rpc::get_weth_name();

        let id = format!("{}_{}", contract_address, marketplace);
        let decimals = rpc::get_decimals(&payment_token).map_or(18, |v| v.to_u64());
        let volume_token = delta.new_value.to_decimal(decimals);

        let volumes = price::get_eth_and_usd_amount(block_number, &payment_token, volume_token);

        if let Some((volume_eth, volume_usd)) = volumes {
            entity_changes
                .push_change(
                    "collection_data",
                    &id,
                    get_ordinal(delta.ordinal, block_number),
                    EcOperation::Update,
                )
                .change("volumeEth", volume_eth)
                .change("volumeUsd", volume_usd);
        }
    }
}
