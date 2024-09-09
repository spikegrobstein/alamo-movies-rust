use serde::{Deserialize, Serialize};

use crate::model::Show;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Presentation {
    pub show: Show,

    #[serde(rename = "primaryCollectionSlug")]
    pub primary_collection_slug: Option<String>,
}
