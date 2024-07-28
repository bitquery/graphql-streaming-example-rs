# Bitquery Graphql Streaming API Example on Rust

## Usage

```bash
API_KEY=ory_... cargo run
```

## Output

```
 Ok(
    Response {
        data: Some(
            ResponseData {
                solana: Some(
                    DexTradesSolana {
                        dex_trades: Some(
                            [
                                DexTradesSolanaDexTrades {
                                    block: Some(
                                        DexTradesSolanaDexTradesBlock {
                                            slot: Some(
                                                "280120788",
                                            ),
                                            time: Some(
                                                DateTime(
                                                    "2024-07-28T08:01:26Z",
                                                ),
                                            ),
                                        },
                                    ),
                                    trade: Some(
                                        DexTradesSolanaDexTradesTrade {
                                            price_asymmetry: Some(
                                                1.0,
                                            ),
                                            buy: Some(
                                                DexTradesSolanaDexTradesTradeBuy {
                                                    amount: Some(
                                                        "1.537152157",
                                                    ),
                                                    account: Some(
                                                        DexTradesSolanaDexTradesTradeBuyAccount {
                                                            address: Some(
                                                                "AZPEmek7FnZqFRUgzW8mpvaJbNhVDdnHfF73aYbFHrei",
                                                            ),
                                                        },
                                                    ),
                                                    price: Some(
                                                        7522075.820553268,
                                                    ),
                                                    currency: Some(
                                                        DexTradesSolanaDexTradesTradeBuyCurrency {
                                                            name: Some(
                                                                "Solana",
                                                            ),
                                                            mint_address: Some(
                                                                "11111111111111111111111111111111",
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),

...
```
