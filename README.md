# Substreams RPC Error

This substream hangs after block 14961464, because of an error in the RPC. The substream indexes Looksrare and SeaPort trades

## Run

```
make stream
```

## Debugging

## The problem is in the RPC request

When commenting out the RPC request, the code works fine, but that means important data is missing.

You can see that it's an RPC error by two ways:

### Remove the price RPC request

Go to [price.rs](src/price.rs) and change the block on `line 46` to:

```
pub fn get_eth_and_usd_amount(
    block_number: u64,
    token_address: &String,
    amount: BigDecimal,
) -> Option<(BigDecimal, BigDecimal)> {

	return None;

    let usd_price_eth = get_price(
        types::Network::Ethereum,
        block_number,
        hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").to_vec(),
    )
    .ok()?;

    match token_address.as_str() {
        "0x0000000000000000000000000000000000000000"
        | "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" => {
            Some((amount.clone(), amount.mul(usd_price_eth)))
        }
        token_address => {
            let usd_price_token = get_price(
                types::Network::Ethereum,
                block_number,
                utils::hex_to_bytes(token_address),
            )
            .ok()?;
            let usd_amount = amount.clone().mul(usd_price_token);
            Some((usd_amount.clone().div(usd_price_eth), usd_amount))
        }
    }
}
```

You will notice the code just runs now. The first RPC request, with the hardcoded WETH address, already causes a problem.

## Add RPC name request

Uncomment `line 90` on [lib.rs](src/lib.rs) and you'll notice the substream won't run, regardless if it's Looksrare or Seaport.
<br/>
<br/>
Comment the above line out again and then uncomment `line 20` in [db/map_collection_data_entity_changes.rs](src/db/map_collection_data_entity_changes.rs). Disable the price RPC request like explained earlier above and you will notice that running the substream will work if you're only doing Seaport, but won't work if you're doing Looksrare (too).
<br/>

## The problem is also Looksrare-only

Removing Looksrare will make this substream run. Remove looksrare in the [/pb/mod.rs file](/src/pb/mod.rs) on line `114`.

Change

```
fn decode_log(log: &Log) -> Option<Self> {
        Self::decode_seaport_order(log).or_else(|| Self::decode_looksrare_taker_ask(log))
}
```

to

```
fn decode_log(log: &Log) -> Option<Self> {
        Self::decode_seaport_order(log)
}
```

You should then run the substream a little longer to hit a block with seaport orders.
<br/>
or change it to

```
fn decode_log(log: &Log) -> Option<Self> {
        Self::decode_looksrare_taker_ask(log)
}
```

to see that it's exclusive to looksrare.
