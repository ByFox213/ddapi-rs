use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

/// Verifies that a raw API JSON payload matches the library's `T` schema exactly:
///
/// 1. `T` must deserialize from the payload — catches fields that are missing,
///    renamed or type-mismatched in the API response.
/// 2. The parsed value is re-serialized and every object key is compared
///    recursively against the original payload — catches two drift directions:
///    - keys in the API response that the library struct does not know
///      (extra data silently ignored by serde);
///    - keys declared in the struct but absent from the API response
///      (serde `default` would silently hide a dropped field).
///
/// Scalar values are not compared: their types are already validated by
/// deserialization, and float/int formatting differences are irrelevant.
pub fn assert_schema_matches<T>(raw: &str)
where
    T: DeserializeOwned + Serialize,
{
    let value: Value =
        serde_json::from_str(raw).unwrap_or_else(|e| panic!("payload is not valid JSON: {e}"));
    assert_schema_value::<T>(&value);
}

/// Same as [`assert_schema_matches`], but takes an already-parsed JSON value
/// (e.g. fetched live via `DDApi::generator::<Value>`).
pub fn assert_schema_value<T>(value: &Value)
where
    T: DeserializeOwned + Serialize,
{
    let parsed: T = serde_json::from_value(value.clone()).unwrap_or_else(|e| {
        panic!(
            "payload does not deserialize into `{}`: {e}",
            std::any::type_name::<T>()
        )
    });
    let schema_value = serde_json::to_value(&parsed)
        .unwrap_or_else(|e| panic!("reserializing must not fail: {e}"));

    let mut problems = Vec::new();
    diff_keys("$", value, &schema_value, &mut problems);
    assert!(
        problems.is_empty(),
        "schema mismatch with `{}`:\n{}",
        std::any::type_name::<T>(),
        problems.join("\n")
    );
}

/// One-way variant of [`assert_schema_value`] for live API data.
///
/// Live responses legitimately omit optional/defaulted fields (older servers,
/// player-dependent data), so the "declared but absent" direction is not
/// checked. Only two things are verified:
///
/// 1. `T` deserializes from the payload — catches missing required fields and
///    type mismatches;
/// 2. every key in the API response is known to `T` — catches upstream drift
///    (new fields the library does not model yet).
///
/// Same check as `assert_no_extra_data`, but reports problems as `Err` instead of panicking.
pub fn check_no_extra_data<T>(value: &Value) -> Result<(), String>
where
    T: DeserializeOwned + Serialize,
{
    let parsed: T = serde_json::from_value(value.clone()).map_err(|e| {
        format!(
            "payload does not deserialize into `{}`: {e}",
            std::any::type_name::<T>()
        )
    })?;
    let schema_value = serde_json::to_value(&parsed)
        .unwrap_or_else(|e| panic!("reserializing must not fail: {e}"));

    let mut problems = Vec::new();
    diff_extra_keys("$", value, &schema_value, &mut problems);
    if problems.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "unknown fields in live API response for `{}`:\n{}",
            std::any::type_name::<T>(),
            problems.join("\n")
        ))
    }
}

fn diff_extra_keys(path: &str, api: &Value, schema: &Value, problems: &mut Vec<String>) {
    match (api, schema) {
        (Value::Object(api_obj), Value::Object(schema_obj)) => {
            for key in api_obj.keys() {
                if !schema_obj.contains_key(key) {
                    problems.push(format!(
                        "{path}.{key}: present in API response but absent from library struct (extra data)"
                    ));
                }
            }
            for key in api_obj.keys().filter(|k| schema_obj.contains_key(*k)) {
                diff_extra_keys(
                    &format!("{path}.{key}"),
                    &api_obj[key],
                    &schema_obj[key],
                    problems,
                );
            }
        }
        (Value::Array(api_arr), Value::Array(schema_arr)) => {
            for (i, (api_item, schema_item)) in api_arr.iter().zip(schema_arr.iter()).enumerate() {
                diff_extra_keys(&format!("{path}[{i}]"), api_item, schema_item, problems);
            }
        }
        _ => {}
    }
}

fn diff_keys(path: &str, api: &Value, schema: &Value, problems: &mut Vec<String>) {
    match (api, schema) {
        (Value::Object(api_obj), Value::Object(schema_obj)) => {
            for key in api_obj.keys() {
                if !schema_obj.contains_key(key) {
                    problems.push(format!(
                        "{path}.{key}: present in API response but absent from library struct (extra data)"
                    ));
                }
            }
            for key in schema_obj.keys() {
                if !api_obj.contains_key(key) {
                    problems.push(format!(
                        "{path}.{key}: declared in library struct but absent from API response"
                    ));
                }
            }
            for key in api_obj.keys().filter(|k| schema_obj.contains_key(*k)) {
                diff_keys(
                    &format!("{path}.{key}"),
                    &api_obj[key],
                    &schema_obj[key],
                    problems,
                );
            }
        }
        (Value::Array(api_arr), Value::Array(schema_arr)) => {
            for (i, (api_item, schema_item)) in api_arr.iter().zip(schema_arr.iter()).enumerate() {
                diff_keys(&format!("{path}[{i}]"), api_item, schema_item, problems);
            }
            if api_arr.len() != schema_arr.len() {
                problems.push(format!(
                    "{path}: array length differs (API: {}, library: {})",
                    api_arr.len(),
                    schema_arr.len()
                ));
            }
        }
        // Scalars and anything else: no keys to compare, types already
        // validated by deserialization.
        _ => {}
    }
}

/// Reads a fixture file from `tests/fixtures/`.
pub fn load_fixture(rel_path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(rel_path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read fixture `{}`: {e}", path.display()))
}
