use serde_json;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Presentation {
    pub show: Show,

    #[serde(rename = "primaryCollectionSlug")]
    pub primary_collection_slug: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Show {
    pub slug: String,
    pub title: String,
    pub certification: Option<String>,
}

