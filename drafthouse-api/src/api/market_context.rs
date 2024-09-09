use reqwest::Method;

use crate::api::MarketSlug;
use crate::model::{Market, MarketListData};
use crate::{Client, Response, Result};

pub struct MarketContext<'alamo, M> {
    pub alamo: &'alamo Client,

    market: M,
}

impl<'alamo> MarketContext<'alamo, ()> {
    pub fn new(alamo: &'alamo Client) -> Self {
        Self { alamo, market: () }
    }

    pub fn list(&self) -> Result<Vec<Market>> {
        Ok(self
            .alamo
            .request(Method::GET, "/v1/page/cclamp?useUnifiedSchedule=true")?
            .send()?
            .json::<Response<MarketListData>>()?
            .data
            .market_summaries)
    }
}

impl<'alamo> MarketContext<'alamo, MarketSlug> {
    pub fn new_with_slug(alamo: &'alamo Client, market: MarketSlug) -> Self {
        Self { alamo, market }
    }

    pub fn get(&self) -> Result<Market> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn list_markets() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/v1/page/cclamp");
            then.status(200).json_body(json!({
                "data": {
                    "marketSummaries": [
                        {
                            "id": "100",
                            "name": "San Francisco",
                            "slug": "sf",
                            "is_open_for_business": true
                        },
                        {
                            "id": "200",
                            "name": "Austin",
                            "slug": "atx",
                            "is_open_for_business": true
                        },
                    ]
                }
            }));
        });

        let url = server.url("");
        let client = Client::new(&url).unwrap();

        let markets = client.markets().list().unwrap();

        assert_eq!(markets.len(), 2);
        assert_eq!(mock.hits(), 1);
    }
}
