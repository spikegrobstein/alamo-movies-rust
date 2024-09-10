use reqwest::Method;

use crate::api::MarketSlug;
use crate::model::{Market, MarketData, MarketListData};
use crate::{Client, Error, Response, Result, V2ErrorBody};

pub struct MarketContext<'alamo, M> {
    pub alamo: &'alamo Client,

    market: M,
}

impl<'alamo> MarketContext<'alamo, ()> {
    pub fn new(alamo: &'alamo Client) -> Self {
        Self { alamo, market: () }
    }

    pub fn list(&self) -> Result<Vec<Market>> {
        let resp = self
            .alamo
            .request(Method::GET, "/v1/page/cclamp?useUnifiedSchedule=true")?
            .send()?;

        if resp.status().is_success() {
            Ok(resp
                .json::<Response<MarketListData>>()?
                .data
                .market_summaries)
        } else {
            Err(Error::HttpError {
                status: resp.status(),
                body: resp.text().unwrap_or_else(|_| "<No body>".to_owned()),
            })
        }
    }
}

impl<'alamo> MarketContext<'alamo, MarketSlug> {
    pub fn new_with_slug(alamo: &'alamo Client, market: MarketSlug) -> Self {
        Self { alamo, market }
    }

    pub fn get(&self) -> Result<MarketData> {
        let resp = self
            .alamo
            .request(Method::GET, &self.market.to_uri())?
            .send()?;

        let status = resp.status();

        if status.is_success() {
            Ok(resp.json::<Response<MarketData>>()?.data)
        } else {
            let error = resp
                .json::<V2ErrorBody>()
                .map_err(|e| Error::HttpError {
                    status,
                    body: e.to_string(),
                })?
                .error;

            Err(Error::ApiError { status, error })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use httpmock::prelude::*;
    use serde_json::json;

    mod listing_markets {
        use super::*;

        #[test]
        fn when_successful() {
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

        #[test]
        fn when_getting_an_error() {
            let server = MockServer::start();

            let mock = server.mock(|when, then| {
                when.method(GET).path("/v1/page/cclamp");
                then.status(404).body("Just a plain-text error.");
            });

            let url = server.url("");
            let client = Client::new(&url).unwrap();

            let err = client.markets().list().err().unwrap();

            eprintln!("{err}");
            assert!(matches!(err, Error::HttpError { .. }));
            assert_eq!(mock.hits(), 1);
        }
    }

    mod get_market {
        use reqwest::StatusCode;

        use super::*;

        #[test]
        fn when_successful() {
            let server = MockServer::start();

            let mock = server.mock(|when, then| {
                when.method(GET).path("/v2/schedule/market/sf");

                then.status(200)
                    .body_from_file("test_data/market-resp-sf.json");
            });

            let url = server.url("");
            let client = Client::new(&url).unwrap();

            let market = client.market(MarketSlug::new("sf")).get().unwrap();

            assert_eq!(market.market.len(), 1);
            assert_eq!(market.presentations.len(), 66);
            assert_eq!(mock.hits(), 1);
        }

        #[test]
        fn when_getting_an_error() {
            let server = MockServer::start();

            let mock = server.mock(|when, then| {
                when.method(GET).path("/v2/schedule/market/sf");

                then.status(404).json_body(json!({
                    "error": {
                        "errorcode": {
                            "category": 101,
                            "code": 404,
                            "description": "Resource unavailable"
                        },
                        "description": "Missing slug",
                        "errorType": "com.drafthouse.core.domain.AdcMiscError"
                    }
                }));
            });

            let url = server.url("");
            let client = Client::new(&url).unwrap();

            let err = client.market(MarketSlug::new("sf")).get().err().unwrap();

            assert_eq!(mock.hits(), 1);
            assert!(matches!(
                err,
                Error::ApiError {
                    status: StatusCode::NOT_FOUND,
                    ..
                }
            ));
        }
    }
}
