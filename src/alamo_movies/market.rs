use serde_json;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::alamo_movies::presentation::Presentation;

#[derive(Deserialize, Debug)]
pub struct MarketApiResponse {
    pub data: MarketData,
}

#[derive(Deserialize, Debug)]
pub struct MarketListApiResponse {
    pub data: MarketListData,
}

#[derive(Deserialize, Debug)]
pub struct MarketListData {
    #[serde(rename = "marketSummaries")]
    pub market_summaries: Vec<Market>,
}

#[derive(Deserialize, Debug)]
pub struct MarketData {
    market: Vec<Market>,
    presentations: Vec<Presentation>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Market {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub is_open_for_business: Option<bool>,
}

impl Market {
    pub fn list() -> Result<Vec<Market>, Box<dyn Error>> {
        let url = format!("https://drafthouse.com/s/mother/v1/page/cclamp?useUnifiedSchedule=true");

        let body = reqwest::get(&url)?
            .text()?;

        let resp: MarketListApiResponse = serde_json::from_str(&body)?;

        Ok(resp.data.market_summaries)
    }

    pub fn from_calendar_json(json: &serde_json::Value) -> Result<Market, Box<dyn Error>> {
        let name = market_name_from(json).unwrap();
        let id = market_id_from(json).unwrap();
        let slug = market_slug_from(json).unwrap();

        unimplemented!("not yet.");

        // Ok(Market {
            // id: id.to_string(),
            // name: name.to_string(),
            // slug: slug.to_string(),
        // })
    }
}

fn market_name_from(data: &serde_json::Value) -> Option<&str> {
    data["MarketName"].as_str()
}

fn market_id_from(data: &serde_json::Value) -> Option<&str> {
    data["MarketId"].as_str()
}

fn market_slug_from(data: &serde_json::Value) -> Option<&str> {
    data["MarketSlug"].as_str()
}

