A simple Rust library to get data from DDNet and DDStats APIs

# What is this?

This library lets you easily access:
- Player stats and rankings
- Server information
- Game data from DDNet
- Statistics from DDStats

## Installation

```bash
  cargo add ddapi-rs -F ddstats
```

## Quick Example

```rust
use ddapi_rs::prelude::*;

#[tokio::main]
async fn main() {
    let ddapi = DDApi::new();
    let player = "ByFox";
    let result = ddapi.s_player(player).await; // ddstats.tw
    println!("{}: {}", player, result.unwrap().url());
}
```

# Features
- ddstats - Get data from ddstats.tw
- ddnet - Get data from ddnet.org
