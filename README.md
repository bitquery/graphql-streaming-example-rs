# Bitquery Graphql Streaming API Example on Rust

## Usage

```bash
API_KEY=ory_... cargo run
```

## Output

```
[src/main.rs:69:9] &response = Ok(
    Response {
        data: Some(
            ResponseData {
                solana: Some(
                    DextradesOnPumpfunSolana {
                        dex_trades: Some(
                            [
                                DextradesOnPumpfunSolanaDexTrades {
                                    block: Some(
                                        DextradesOnPumpfunSolanaDexTradesBlock {
                                            slot: Some(
                                                "280033880",
                                            ),
                                            time: Some(
                                                DateTime(
                                                    "2024-07-27T21:09:32Z",
                                                ),
                                            ),
                                        },
                                    ),
...
```
