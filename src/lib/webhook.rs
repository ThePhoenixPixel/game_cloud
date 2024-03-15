use reqwest::blocking::Client;
use serde::Serialize;
use std::thread;
use std::time::Duration;

pub struct Webhook;

impl Webhook {
    //send a response on a server
    pub fn send<T>(data: T, url: &str) -> Result<(), String>
    where
        T: Serialize,
    {
        thread::sleep(Duration::from_secs(1));

        // change the generic object into a json object
        let json_data = match serde_json::to_value(&data) {
            Ok(json_data) => json_data,
            Err(e) => return Err(e.to_string()),
        };

        let client = Client::new();

        // send the response
        return match client.post(url).form(&json_data).send() {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(response.status().to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        };
    }
}
