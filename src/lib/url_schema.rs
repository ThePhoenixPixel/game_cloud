use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UrlSchema {
    Http,
    Https,
}

impl UrlSchema {
    pub fn to_string(&self) -> String {
        return match self {
            UrlSchema::Http => String::from("http"),
            UrlSchema::Https => String::from("https"),
        };
    }
}