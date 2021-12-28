use crate::{AdsError, Result};
use std::env;
use std::fs;

pub fn get_token() -> Result<String> {
    get_token_from_env_vars().or_else(|_| get_token_from_home_dir())
}

fn get_token_from_env_vars() -> Result<String> {
    if let Ok(token) = env::var("ADS_API_TOKEN") {
        Ok(token)
    } else if let Ok(token) = env::var("ADS_DEV_KEY") {
        Ok(token)
    } else {
        Err(AdsError::Token)
    }
}

fn get_token_from_home_dir() -> Result<String> {
    if let Some(mut ads_dir) = dirs::home_dir() {
        ads_dir.push(".ads");
        if let Ok(token) = fs::read_to_string(ads_dir.join("token")) {
            return Ok(token.trim().to_owned());
        } else if let Ok(token) = fs::read_to_string(ads_dir.join("dev_key")) {
            return Ok(token.trim().to_owned());
        }
    }
    Err(AdsError::Token)
}
