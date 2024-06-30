pub mod grpc;
pub mod utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}
impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Protocol::Http => "http".to_string(),
            Protocol::Https => "https".to_string(),
        }
    }
}
