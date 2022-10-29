use std::str;
use substreams::hex;
use substreams::scalar::BigInt;

use crate::types::Address;

pub fn get_ordinal(ordinal: u64, block_number: u64) -> u64 {
    ordinal + block_number
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string.split("0x").nth(1).unwrap()).unwrap()
}

const NATIVE_TOKEN_ADDRESS: Address = hex!("0000000000000000000000000000000000000000");
const WETH_TOKEN_ADDRESS: Address = hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const DAI_TOKEN_ADDRESS: Address = hex!("6b175474e89094c44da98b954eedeac495271d0f");
const USDC_TOKEN_ADDRESS: Address = hex!("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
const USDT_TOKEN_ADDRESS: Address = hex!("dac17f958d2ee523a2206206994597c13d831ec7");
const TUSD_TOKEN_ADDRESS: Address = hex!("0000000000085d4780b73119b644ae5ecd22b376");
const FEI_TOKEN_ADDRESS: Address = hex!("956f47f50a910163d8bf957cf5846d573e7f87ca");

/// Get static decimals for tokens with weird behaviour or they appear a lot
pub fn get_static_erc20_decimals(token_address: &[u8]) -> Option<BigInt> {
    match token_address {
        x if x == NATIVE_TOKEN_ADDRESS => Some(BigInt::from(18_i32)),
        x if x == WETH_TOKEN_ADDRESS => Some(BigInt::from(18_i32)),
        x if x == DAI_TOKEN_ADDRESS => Some(BigInt::from(18_i32)),
        x if x == USDC_TOKEN_ADDRESS => Some(BigInt::from(6_i32)),
        x if x == USDT_TOKEN_ADDRESS => Some(BigInt::from(6_i32)),
        x if x == TUSD_TOKEN_ADDRESS => Some(BigInt::from(18_i32)),
        x if x == FEI_TOKEN_ADDRESS => Some(BigInt::from(18_i32)),

        _ => None,
    }
}
