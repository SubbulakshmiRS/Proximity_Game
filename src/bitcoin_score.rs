use dotenv;
use reqwest::*;
use serde::{Deserialize, Serialize};
use std::env;
// use serde_json::Result;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct BitJsonResp {
    code: String,
    rate: String,
    description: String,
    rate_float: f64,
}

#[derive(Deserialize, Debug)]
struct BitGeneralJsonResp {
    time: HashMap<String, String>,
    disclaimer: String,
    bpi: HashMap<String, BitJsonResp>,
}

fn to_my_type(value: serde_json::Value) -> BitGeneralJsonResp {
    serde_json::from_value(value).unwrap()
}

#[tokio::main]
// M-2: via API
pub async fn get_bit_score() -> Result<()> {
    dotenv::dotenv().ok();
    // dotenv::from_path("./.env").expect(".ENV file is present");
    let url_token = std::env::var("BIT_URL").expect("BIT_URL must be set.");
    let val = reqwest::get(url_token)
        .await?
        .json::<BitGeneralJsonResp>()
        .await?;
    // println!("{:#?}", resp);

    // let val: BitGeneralJsonResp = serde_json::from_value(resp).unwrap();
    // let val = resp.trim().parse::<BitGeneralJsonResp>().expect("Error in parsing");
    println!("value = {:?}", val);
    let score = val.bpi["USD"].rate_float;
    println!("value of bit score = {:?}", score);

    Ok(())
}
