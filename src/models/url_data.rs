use serde::Deserialize;

#[derive(Deserialize)]
pub struct UrlData {
    pub url: String,
    pub priority: Option<f64>,
}
