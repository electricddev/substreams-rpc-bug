use crate::abi::looksrare::events::TakerAsk;
use crate::{
    pb::seaport::{ItemType, SpentItem},
    seaport::helpers::NftPayment,
    types::Address,
    utils::bytes_to_hex,
};
use hex_literal::hex;

pub const LOOKSRARE_ADDRESS: Address = hex!("59728544b08ab483533076417fbbb2fd0b17ce3a");

pub fn extract_nft_payment(order: &TakerAsk) -> Option<NftPayment> {
    Some(NftPayment {
        item: SpentItem {
            item_type: ItemType::Erc721 as i32,
            token: bytes_to_hex(&order.collection),
            identifier: order.token_id.to_string(),
            amount: order.amount.to_string(),
        },
        payment: SpentItem {
            item_type: ItemType::Erc20 as i32,
            token: bytes_to_hex(&order.currency),
            identifier: order.token_id.to_string(),
            amount: order.amount.to_string(),
        },
        buyer: bytes_to_hex(&order.maker),
        seller: bytes_to_hex(&order.taker),
    })
}
