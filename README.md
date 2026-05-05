ddapi-rs
=======

A small async Rust library for working with the DDNet and DDStats public APIs.

- Crates.io: https://crates.io/crates/ddapi-rs
- Docs.rs: https://docs.rs/ddapi-rs

Features
--------

- `ddnet` (default) - DDNet API (`ddnet.org`)
- `ddstats` - DDStats API (`ddstats.tw`)
- `cache` - in-memory cache for responses (uses `moka`)
- `full` - enables `ddnet`, `ddstats`, `cache`

Installation
------------

Default (DDNet only):

```bash
cargo add ddapi-rs
```

Enable DDStats too:

```bash
cargo add ddapi-rs -F ddstats
```

Enable caching:

```bash
cargo add ddapi-rs -F cache
```

Everything:

```bash
cargo add ddapi-rs -F full
```

Usage
-----

The crate is async and uses `tokio`.

DDNet example (enabled by default):

```rust
use ddapi_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let api = DDApi::new();

    // ddnet: player points
    let player = api.player("nameless tee").await?;
    println!("{}: {}", player.player, player.points.points.unwrap_or(0));
    Ok(())
}
```

DDStats example (requires `-F ddstats`):

```rust
use ddapi_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let api = DDApi::new();

    // ddstats: profile info
    let profile = api.profile("ByFox").await?;
    println!("{} ({})", profile.name, profile.clan.unwrap_or_default());
    Ok(())
}
```

Optional: only use one API
--------------------------

If you do not want a combined client, you can use `DDnetClient` / `DDstatsClient`.

```rust
use ddapi_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let ddnet = DDnetClient::new();
    let p = ddnet.player("nameless tee").await?;
    println!("{}: {}", p.player, p.points.points.unwrap_or(0));
    Ok(())
}
```

Caching (feature `cache`)
-------------------------

```rust
use ddapi_rs::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut api = DDApi::new();
    api.set_cache(1000, Duration::from_secs(60 * 5));

    // Will be cached based on URL + TTL
    let _ = api.status().await?;
    Ok(())
}
```

Custom reqwest client
---------------------

```rust
use ddapi_rs::prelude::*;
use reqwest::Client;

fn main() {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let _api = DDApi::new_with_client(client);
}
```
