use hex_literal::hex;
use lazy_static::lazy_static;
use substreams::scalar::BigInt;

use crate::types::Address;

lazy_static! {
    pub static ref BN_ONE: BigInt = BigInt::from(1);
    pub static ref BN_MIN_ONE: BigInt = BigInt::from(-1);
    pub static ref OPENSEA: String = String::from("OPENSEA");
    pub static ref LOOKSRARE: String = String::from("LOOKSRARE");
    pub static ref LOOKSRARE_ADDRESS: Address = hex!("59728544b08ab483533076417fbbb2fd0b17ce3a");
}
