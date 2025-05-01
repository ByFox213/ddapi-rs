ddapi is an api from the ddnet and qwik website

## Installation

```bash
  cargo add ddapi-rs -F ddstats
```

## Usage/Examples

```rust
use ddapi_rs::api::DDApi;
use ddapi_rs::api::ddstats::DDstats;

#[tokio::main]
async fn main() {
    let ddapi = DDApi::new();
    let player = "ByFox";
    let result = ddapi.s_player(player).await; // DDstats
    println!("{}: {}", player, result.unwrap().url());
}
```
