use serde::{Deserialize, Serialize};

use crate::model::Show;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Presentation {
    pub show: Show,

    /// a slug that identifies the collection such as `terror-tuesday`
    pub primary_collection_slug: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PresentationData {
    pub presentation: Presentation,
}
