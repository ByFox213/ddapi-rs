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

/// Candidate players for live schema checks. API payloads are
/// player-dependent, so a single fixed name is fragile: the test passes as
/// soon as one player validates cleanly, and only fails when every candidate
/// reports problems (real upstream drift).
const TEST_PLAYERS: &[&str] = &["Cor", "ByFox"];

async fn fetch_and_check<T>(url: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fetch_and_check_result::<T>(url)
        .await
        .unwrap_or_else(|e| panic!("{e}"));
}

async fn fetch_and_check_result<T>(url: &str) -> Result<(), String>
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
            Err(e) => return Err(format!("GET {url} failed: {e}")),
        }
    }
    util::check_no_extra_data::<T>(&value)
}

/// Checks `T` against every candidate player; succeeds on the first clean
/// payload, otherwise reports all per-player problems.
async fn check_any_player<T, F>(make_url: F)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
    F: Fn(&str) -> String,
{
    let mut failures = Vec::new();
    for player in TEST_PLAYERS {
        let url = make_url(player);
        match fetch_and_check_result::<T>(&url).await {
            Ok(()) => return,
            Err(e) => failures.push(format!("{url}:\n{e}")),
        }
    }
    assert!(
        failures.is_empty(),
        "no test player validated against `{}`:\n{}",
        std::any::type_name::<T>(),
        failures.join("\n\n")
    );
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn player_schema_matches_live() {
    check_any_player::<Player, _>(Player::api).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn maps_schema_matches_live() {
    fetch_and_check::<Vec<StatsMap>>(&StatsMap::api()).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn teero_schema_matches_live() {
    check_any_player::<Teero, _>(Teero::api).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn profile_schema_matches_live() {
    check_any_player::<Profile, _>(Profile::api).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn map_schema_matches_live() {
    fetch_and_check::<Map>(&Map::api("Fox")).await;
}
