use crate::{
    abi,
    utils::{self, hex_to_bytes},
};
use substreams::hex;
use substreams::scalar::BigInt;

pub fn get_decimals(token_address: &String) -> Option<BigInt> {
    let token_address = hex_to_bytes(token_address.as_str());

    match utils::get_static_erc20_decimals(&token_address) {
        Some(decimals) => return Some(decimals),
        None => {}
    }

    let request = abi::erc20::functions::Decimals {};

    if let Some(decimals) = request.call(token_address) {
        return Some(decimals);
    }
    return None;
}

pub fn get_weth_name() -> Option<String> {
    abi::erc20::functions::Name {}.call(hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").to_vec())
}
