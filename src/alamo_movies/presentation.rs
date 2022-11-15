use serde_json;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Presentation {
    pub show: Show,
}

#[derive(Deserialize, Debug)]
pub struct Show {
    pub slug: String,
    pub title: String,
    pub certification: Option<String>,
}

