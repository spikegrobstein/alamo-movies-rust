use reqwest::Method;

use crate::api::PresentationSlug;
use crate::model::{Presentation, PresentationData, Show};
use crate::{Client, Error, Response, Result, V2ErrorBody};

pub struct PresentationContext<'alamo, P> {
    pub alamo: &'alamo Client,

    presentation: P,
}

impl<'alamo> PresentationContext<'alamo, ()> {
    pub fn new(alamo: &'alamo Client) -> Self {
        Self {
            alamo,
            presentation: (),
        }
    }
}

impl<'alamo> PresentationContext<'alamo, PresentationSlug> {
    pub fn new_with_slug(alamo: &'alamo Client, presentation: PresentationSlug) -> Self {
        Self {
            alamo,
            presentation,
        }
    }

    pub fn get(&self) -> Result<Presentation> {
        let resp = self
            .alamo
            .request(Method::GET, &self.presentation.to_uri())?
            .send()?;

        let status = resp.status();

        if status.is_success() {
            Ok(resp.json::<Response<PresentationData>>()?.data.presentation)
        } else {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use httpmock::prelude::*;
    use serde_json::json;

    #[test]
    fn get_presentation() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/v2/schedule/presentation/beetlejuice-beetlejuice");

            then.status(200)
                .body_from_file("test_data/presentation-resp-beetlejuice-beetlejuice.json");
        });

        let url = server.url("");
        let client = Client::new(&url).unwrap();

        client
            .presentation(PresentationSlug::new("beetlejuice-beetlejuice"))
            .get()
            .unwrap();

        assert_eq!(mock.hits(), 1);
    }
}
