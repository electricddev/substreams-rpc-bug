pub mod helpers;
use hex_literal::hex;

use crate::types::Address;

const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";

pub const SEAPORT_ADDRESS: Address = hex!("00000000006c3852cbef3e08e8df289169ede581");

#[derive(Debug, Clone, PartialEq)]
pub struct OrderFulfilled {
    pub order_hash: Vec<u8>,
    pub offerer: Vec<u8>,
    pub zone: Vec<u8>,
    pub recipient: Vec<u8>,
    pub offer: Vec<SpentItem>,
    pub consideration: Vec<ReceivedItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpentItem {
    pub item_type: substreams::scalar::BigInt,
    pub token: Vec<u8>,
    pub identifier: substreams::scalar::BigInt,
    pub amount: substreams::scalar::BigInt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReceivedItem {
    pub item_type: substreams::scalar::BigInt,
    pub token: Vec<u8>,
    pub identifier: substreams::scalar::BigInt,
    pub amount: substreams::scalar::BigInt,
    pub recipient: Vec<u8>,
}

impl From<ethabi::Token> for SpentItem {
    fn from(token: ethabi::Token) -> Self {
        let mut values = token.into_tuple().expect(INTERNAL_ERR);
        values.reverse();
        Self {
            item_type: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                v.into()
            },
            token: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec(),
            identifier: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                v.into()
            },
            amount: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                v.into()
            },
        }
    }
}

impl From<ethabi::Token> for ReceivedItem {
    fn from(token: ethabi::Token) -> Self {
        let mut values = token.into_tuple().expect(INTERNAL_ERR);
        values.reverse();
        Self {
            item_type: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                v.into()
            },
            token: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec(),
            identifier: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                v.into()
            },
            amount: {
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                v.into()
            },
            recipient: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec(),
        }
    }
}

impl OrderFulfilled {
    const TOPIC_ID: [u8; 32] =
        hex!("9d9af8e38d66c62e2c12f0225249fd9d721c54b83f48d9352c97c6cacdcb6f31");

    pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        if log.topics.len() != 3usize {
            return false;
        }
        return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
    }

    pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
        let mut values = ethabi::decode(
            &[
                ethabi::ParamType::FixedBytes(32usize),
                ethabi::ParamType::Address,
                ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                    ethabi::ParamType::Uint(8usize),
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                ]))),
                ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                    ethabi::ParamType::Uint(8usize),
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Address,
                ]))),
            ],
            log.data.as_ref(),
        )
        .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
        values.reverse();
        Ok(Self {
            order_hash: {
                let mut result = [0u8; 32];
                let v = values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                result.copy_from_slice(&v);
                result.into()
            },
            offerer: ethabi::decode(&[ethabi::ParamType::Address], log.topics[1usize].as_ref())
                .map_err(|e| {
                    format!(
                        "unable to decode param 'offerer' from topic of type 'address': {:?}",
                        e
                    )
                })?
                .pop()
                .expect(INTERNAL_ERR)
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec(),
            zone: ethabi::decode(&[ethabi::ParamType::Address], log.topics[2usize].as_ref())
                .map_err(|e| {
                    format!(
                        "unable to decode param 'zone' from topic of type 'address': {:?}",
                        e
                    )
                })?
                .pop()
                .expect(INTERNAL_ERR)
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec(),
            recipient: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec(),
            offer: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_array()
                .expect(INTERNAL_ERR)
                .into_iter()
                .map(|spent_item| SpentItem::from(spent_item))
                .collect(),
            consideration: values
                .pop()
                .expect(INTERNAL_ERR)
                .into_array()
                .expect(INTERNAL_ERR)
                .into_iter()
                .map(|item| ReceivedItem::from(item))
                .collect(),
        })
    }
}

impl substreams_ethereum::Event for OrderFulfilled {
    const NAME: &'static str = "OrderFulfilled";
    fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
        Self::match_log(log)
    }
    fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
        Self::decode(log)
    }
}
