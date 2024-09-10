pub struct PresentationSlug(pub String);

impl PresentationSlug {
    pub fn new<T: Into<String>>(slug: T) -> Self {
        Self(slug.into())
    }

    pub fn to_uri(&self) -> String {
        format!("/v2/schedule/presentation/{}", self.0)
    }
}
