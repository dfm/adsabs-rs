use dirs;
use std::env;
use std::fs;

pub fn get_token() -> Option<String> {
    if let Some(token) = token_from_env_vars() {
        Some(token)
    } else if let Some(token) = token_from_home_dir() {
        Some(token)
    } else {
        None
    }
}

pub fn token_from_env_vars() -> Option<String> {
    if let Ok(token) = env::var("ADS_API_TOKEN") {
        Some(token)
    } else if let Ok(token) = env::var("ADS_DEV_KEY") {
        Some(token)
    } else {
        None
    }
}

pub fn token_from_home_dir() -> Option<String> {
    let mut ads_dir = dirs::home_dir()?;
    ads_dir.push(".ads");
    if let Ok(token) = fs::read_to_string(ads_dir.join("token")) {
        Some(token.trim().to_string())
    } else if let Ok(token) = fs::read_to_string(ads_dir.join("dev_key")) {
        Some(token.trim().to_string())
    } else {
        None
    }
}
