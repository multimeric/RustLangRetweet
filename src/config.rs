use serde_json::{json, Value};
use std::future::Future;
use std::{fs, io};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String
}

pub(crate) fn load_config(path: &str) -> Result<Config>{
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

#[test]
fn test_config(){
    let cfg = load_config("config.json".into()).unwrap();
    dbg!(cfg);
}