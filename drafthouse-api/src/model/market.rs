use serde::{Deserialize, Serialize};

use crate::model::Presentation;

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketData {
    pub market: Vec<Market>,
    pub presentations: Vec<Presentation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketListData {
    pub market_summaries: Vec<Market>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Market {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub is_open_for_business: Option<bool>,
}
