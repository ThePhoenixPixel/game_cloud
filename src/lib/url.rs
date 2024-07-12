use reqwest::{Client, Error, Response};
use serde::Serialize;

pub struct Url {
    url: String,
}

impl Url {
    pub fn new(url: &String) -> Url {
        Url {
            url: url.clone()
        }
    }

    pub fn to_string(&self) -> String {
        self.url.clone()
    }

    pub fn push(&mut self, str: &str) {
        self.url = format!("{}/{}", self.url, str)
    }

    pub fn join(&self, str: &str) -> Url {
        Url {
            url: format!("{}/{}", self.url, str)
        }
    }

    pub async fn post<T: Serialize>(&self, body: &T) -> Result<Response, Error> {
        let client = Client::new();
        return client.post(&self.url)
            .json(&body)
            .send().await;
    }
}