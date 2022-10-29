use substreams_ethereum::Event;

use crate::abi::looksrare as looksrare_abi;
use crate::constants::LOOKSRARE;
use crate::looksrare::LOOKSRARE_ADDRESS;
use crate::seaport as seaport_abi;
use crate::{constants::OPENSEA, utils::bytes_to_hex};
use substreams_ethereum::pb::eth::v2::Log;

#[path = "eth.seaport.v1.rs"]
#[allow(dead_code)]
pub mod seaport;

#[path = "eth.marketplaces.v1.rs"]
#[allow(dead_code)]
pub mod marketplaces;

impl marketplaces::Trade {
    fn decode_seaport_order(log: &Log) -> Option<Self> {
        if log.address.ne(&seaport_abi::SEAPORT_ADDRESS) {
            return None;
        }

        let seaport_order = match seaport_abi::OrderFulfilled::match_and_decode(log) {
            None => return None,
            Some(order) => order,
        };

        let mut order = seaport::OrderFulfilled {
            order_hash: bytes_to_hex(&seaport_order.order_hash),
            offerer: bytes_to_hex(&seaport_order.offerer),
            zone: bytes_to_hex(&seaport_order.zone),
            recipient: bytes_to_hex(&seaport_order.recipient),
            offer: vec![],
            consideration: vec![],
            ordinal: log.ordinal,
            log_index: log.index,
        };

        for item in &seaport_order.offer {
            let spent_item = seaport::SpentItem {
                item_type: item.item_type.to_u64() as i32,
                token: bytes_to_hex(&item.token),
                identifier: item.identifier.to_string(),
                amount: item.amount.to_string(),
            };

            order.offer.push(spent_item);
        }

        for item in &seaport_order.consideration {
            let received_item = seaport::ReceivedItem {
                item_type: item.item_type.to_u64() as i32,
                token: bytes_to_hex(&item.token),
                identifier: item.identifier.to_string(),
                amount: item.amount.to_string(),
                recipient: bytes_to_hex(&item.recipient),
            };

            order.consideration.push(received_item);
        }
        if let Some(payment) = crate::seaport::helpers::extract_nft_payment(&order) {
            Some(Self {
                marketplace: OPENSEA.clone(),
                marketplace_address: order.zone,
                order_hash: order.order_hash,
                contract_address: payment.item.token,
                token_id: payment.item.identifier,
                amount: payment.item.amount,
                buyer: payment.buyer,
                seller: payment.seller,
                price: payment.payment.amount,
                payment_token: payment.payment.token,
                ordinal: order.ordinal,
                log_index: order.log_index,
            })
        } else {
            None
        }
    }

    fn decode_looksrare_taker_ask(log: &Log) -> Option<Self> {
        if log.address.ne(&LOOKSRARE_ADDRESS) {
            return None;
        }

        // Transform it into seaport order for debugging
        let taker_ask = match looksrare_abi::events::TakerAsk::match_and_decode(log) {
            Some(taker_ask) => taker_ask,
            None => return None,
        };

        if let Some(payment) = crate::looksrare::extract_nft_payment(&taker_ask) {
            Some(Self {
                marketplace: LOOKSRARE.clone(),
                marketplace_address: bytes_to_hex(&taker_ask.strategy),
                order_hash: bytes_to_hex(&taker_ask.order_hash),
                contract_address: payment.item.token,
                token_id: payment.item.identifier,
                amount: payment.item.amount,
                buyer: payment.buyer,
                seller: payment.seller,
                price: payment.payment.amount,
                payment_token: payment.payment.token,
                ordinal: log.ordinal,
                log_index: log.index,
            })
        } else {
            None
        }
    }

    fn decode_log(log: &Log) -> Option<Self> {
        Self::decode_seaport_order(log).or_else(|| Self::decode_looksrare_taker_ask(log))
    }
}

impl marketplaces::Trade {
    pub fn match_and_decode(log: impl AsRef<Log>) -> Option<Self> {
        let log = log.as_ref();
        Self::decode_log(log)
    }
}
