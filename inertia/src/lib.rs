use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct InertiaAppResponse {
    pub head: Vec<String>,
    pub body: String
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct InertiaPage {
    pub component: String,
    pub props: Map<String, Value>,
    pub url: String,
    pub version: Option<String>,
}
