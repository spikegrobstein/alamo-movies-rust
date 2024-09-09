use reqwest::{
    blocking::{self, RequestBuilder},
    Method,
};
use url::Url;

use crate::api::{MarketContext, MarketSlug};
use crate::Result;

pub const DEFAULT_ENDPOINT: &str = "https://drafthouse.com/s/mother";

pub struct Client {
    base_url: Url,
    client: blocking::Client,
}

impl Client {
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url)?;

        Ok(Self {
            base_url,
            client: blocking::Client::builder()
                .user_agent("Drafthouse Client 0.0")
                .build()?,
        })
    }

    pub fn request(&self, method: Method, uri: &str) -> Result<RequestBuilder> {
        let url = self.base_url.join(uri)?;

        Ok(self.client.request(method, url.as_str()))
    }

    pub fn markets(&self) -> MarketContext<'_, ()> {
        MarketContext::new(self)
    }

    pub fn market(&self, slug: MarketSlug) -> MarketContext<'_, MarketSlug> {
        MarketContext::new_with_slug(self, slug)
    }
}
