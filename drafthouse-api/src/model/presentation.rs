use serde::{Deserialize, Serialize};

use crate::model::Show;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Presentation {
    pub show: Show,

    pub primary_collection_slug: Option<String>,
}
