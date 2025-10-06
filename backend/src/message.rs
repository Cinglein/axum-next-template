use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, TS, ToSchema)]
#[ts(export, export_to = "../../frontend/src/bindings/")]
pub struct Message {
    pub data: String,
}
