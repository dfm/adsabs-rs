use adsabs::models;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_simple_load() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/data/emcee.json");
    let data = fs::read_to_string(d).unwrap();
    let response: models::Document = serde_json::from_str(&data).unwrap();
    assert_eq!(response.doi.unwrap()[0], "10.1086/670067");
}
