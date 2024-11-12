use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub code: String,
}