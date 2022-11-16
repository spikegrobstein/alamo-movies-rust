use serde_json;
use std::error::Error;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::alamo_movies::presentation::Presentation;
use crate::alamo_movies::db;

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

    pub fn sync_file(slug: &str) -> Result<(Self, Vec<Presentation>), Box<dyn Error>> {
        let body = Self::get_calendar_data(slug)?;

        // validate it.
        let resp: MarketApiResponse = serde_json::from_str(&body)?;

        db::write_calendar_file(slug, &body)?;

        let market = resp.data.market[0].clone();
        let presentations = resp.data.presentations;

        Ok((market, presentations))
    }

    pub fn get_calendar_data(slug: &str) -> Result<String, Box<dyn Error>>  {
        let url = format!("https://drafthouse.com/s/mother/v2/schedule/market/{slug}");

        eprintln!("requesting file...");

        let body = reqwest::get(&url)?
            .text()?;

        Ok(body)
    }

    pub fn from_calendar_file(path: &Path) -> Result<(Self, Vec<Presentation>), Box<dyn Error>> {
        let body = fs::read_to_string(path.to_str().unwrap())?;

        let resp: MarketApiResponse = serde_json::from_str(&body)?;

        let market = resp.data.market[0].clone();
        let presentations = resp.data.presentations;

        Ok((market, presentations))
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

