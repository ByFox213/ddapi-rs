#![cfg(feature = "ddstats")]

//! Schema-consistency tests for the `DDStats` API.
//!
//! Same two-layer structure as `schema_ddnet.rs`:
//! fixture tests by default (strict, offline-safe), live tests behind
//! `#[ignore]` (one-way extra-data check, since live payloads vary).

mod util;

use ddapi_rs::prelude::ddstats::*;
use ddapi_rs::prelude::DDApi;
use serde_json::Value;
use util::{assert_schema_matches, load_fixture};

// ---------------------------------------------------------------- fixtures

#[test]
fn profile_schema_matches() {
    assert_schema_matches::<Profile>(&load_fixture("ddstats/profile.json"));
}

#[test]
fn map_schema_matches() {
    assert_schema_matches::<Map>(&load_fixture("ddstats/map.json"));
}

// ---------------------------------------------------------------- live API

async fn fetch_and_check<T>(url: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let api = DDApi::new();
    let mut value: Value = Value::Null;
    for attempt in 1..=3 {
        match api.generator(url).await {
            Ok(v) => {
                value = v;
                break;
            }
            Err(e) if attempt < 3 => {
                eprintln!("GET {url} failed (attempt {attempt}/3): {e}; retrying");
            }
            Err(e) => panic!("GET {url} failed: {e}"),
        }
    }
    util::check_no_extra_data::<T>(&value).unwrap_or_else(|e| panic!("{e}"));
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn player_schema_matches_live() {
    fetch_and_check::<Player>(&Player::api("Cor")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn maps_schema_matches_live() {
    fetch_and_check::<Vec<StatsMap>>(&StatsMap::api()).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn profile_schema_matches_live() {
    fetch_and_check::<Profile>(&Profile::api("Cor")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn map_schema_matches_live() {
    fetch_and_check::<Map>(&Map::api("Fox")).await;
}
