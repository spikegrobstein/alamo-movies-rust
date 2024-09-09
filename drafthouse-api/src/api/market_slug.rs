pub struct MarketSlug(pub String);

impl MarketSlug {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Self(id.into())
    }
}
