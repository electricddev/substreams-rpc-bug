use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("SEAPORT", "abi/seaport.json")?
        .generate()?
        .write_to_file("src/abi/seaport.rs")?;

    Abigen::new("Looksrare", "abi/looksrare.json")?
        .generate()?
        .write_to_file("src/abi/looksrare.rs")?;

    Abigen::new("erc20", "abi/ERC20.json")?
        .generate()?
        .write_to_file("src/abi/erc20.rs")?;

    Abigen::new(
        "chainlink_feed_registry",
        "abi/chainlink_feed_registry.json",
    )?
    .generate()?
    .write_to_file("src/abi/chainlink_feed_registry.rs")?;

    Abigen::new("curve_calculations", "abi/curve_calculations.json")?
        .generate()?
        .write_to_file("src/abi/curve_calculations.rs")?;

    Abigen::new("sushiswap_calculations", "abi/sushiswap_calculations.json")?
        .generate()?
        .write_to_file("src/abi/sushiswap_calculations.rs")?;

    Abigen::new("yearn_lens_oracle", "abi/yearn_lens_oracle.json")?
        .generate()?
        .write_to_file("src/abi/yearn_lens_oracle.rs")?;
    Ok(())
}
