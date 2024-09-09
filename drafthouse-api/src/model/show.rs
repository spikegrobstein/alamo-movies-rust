use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Show {
    pub slug: String,
    pub title: String,
    pub certification: Option<String>,
}
