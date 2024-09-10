pub struct MarketSlug(pub String);

impl MarketSlug {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Self(id.into())
    }

    pub fn to_uri(&self) -> String {
        format!("/v2/schedule/market/{}", self.0)
    }
}
