use std::ops::Mul;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    ParseBigDecimal(#[from] bigdecimal::ParseBigDecimalError),
}

pub fn decimal_from_str(price_str: &str) -> Result<BigDecimal, Error> {
    Ok(BigDecimal::from_str(price_str)?.with_prec(100))
}

pub fn exponent_to_big_decimal(decimals: u8) -> BigDecimal {
    let mut result = BigDecimal::one();
    let big_decimal_ten: BigDecimal = BigDecimal::from(10i32);
    for _i in 0..decimals {
        result = result.mul(big_decimal_ten.clone());
    }

    result
}
