#![cfg(feature = "ddnet")]

//! Schema-consistency tests for the `DDNet` API.
//!
//! Two layers:
//! - **Fixture tests** (run by default): verify the library structs against
//!   committed snapshots of real API responses. Deterministic, offline-safe.
//! - **Live tests** (`#[ignore]`, run with `cargo test -- --ignored`): fetch the
//!   current API responses and check for upstream schema drift — new fields the
//!   library does not know yet. Optional/defaulted fields are not compared
//!   against, since live data legitimately varies (older servers,
//!   player-dependent payloads). Endpoints too large to commit as fixtures
//!   (master, skins, player, releases) live here.

mod util;

use ddapi_rs::prelude::ddnet::*;
use ddapi_rs::prelude::DDApi;
use serde_json::Value;
use util::{assert_schema_matches, load_fixture};

// ---------------------------------------------------------------- fixtures

#[test]
fn status_schema_matches() {
    assert_schema_matches::<Status>(&load_fixture("ddnet/status.json"));
}

#[test]
fn query_schema_matches() {
    assert_schema_matches::<Vec<Query>>(&load_fixture("ddnet/query.json"));
}

#[test]
fn query_map_schema_matches() {
    assert_schema_matches::<Vec<QueryMap>>(&load_fixture("ddnet/query_map.json"));
}

#[test]
fn query_mapper_schema_matches() {
    assert_schema_matches::<Vec<QueryMapper>>(&load_fixture("ddnet/query_mapper.json"));
}

#[test]
fn map_schema_matches() {
    assert_schema_matches::<Map>(&load_fixture("ddnet/map.json"));
}

#[test]
fn latest_finishes_schema_matches() {
    assert_schema_matches::<Vec<LatestFinishes>>(&load_fixture("ddnet/latest_finishes.json"));
}

// ---------------------------------------------------------------- live API

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

/// Covers `master`, `custom_master`. All four master servers are checked
/// independently; a failure on one does not abort the others.
#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn master_schema_matches_live() {
    let mut failures = Vec::new();
    for master in [
        MasterServer::One,
        MasterServer::Two,
        MasterServer::Three,
        MasterServer::Four,
    ] {
        let url = Master::api(master);
        if let Err(e) = fetch_and_check_result::<Master>(&url).await {
            failures.push(format!("{url}:\n{e}"));
        }
    }
    assert!(
        failures.is_empty(),
        "master schema check failed:\n{}",
        failures.join("\n\n")
    );
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn skins_schema_matches_live() {
    fetch_and_check::<DDSkins>(&DDSkins::api()).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn player_schema_matches_live() {
    fetch_and_check::<Player>(&Player::api("nameless tee")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn releases_map_schema_matches_live() {
    fetch_and_check::<Vec<ReleasesMaps>>(&ReleasesMaps::api()).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn status_schema_matches_live() {
    fetch_and_check::<Status>(&Status::api()).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn query_schema_matches_live() {
    fetch_and_check::<Vec<Query>>(&Query::api("nameless tee")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn query_map_schema_matches_live() {
    fetch_and_check::<Vec<QueryMap>>(&QueryMap::api("multi")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn query_mapper_schema_matches_live() {
    fetch_and_check::<Vec<QueryMapper>>(&QueryMapper::api("Ao")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn map_schema_matches_live() {
    fetch_and_check::<Map>(&Map::api("Fox")).await;
}

#[tokio::test]
#[ignore = "requires network; run with `cargo test -- --ignored`"]
async fn latest_finishes_schema_matches_live() {
    fetch_and_check::<Vec<LatestFinishes>>(&LatestFinishes::api(0)).await;
}
