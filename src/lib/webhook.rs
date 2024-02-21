use reqwest::blocking::Client;
use serde::Serialize;
use std::thread;
use std::time::Duration;

use crate::log_error;
use crate::utils::logger::Logger;

pub struct Webhook;

impl Webhook {
    pub fn send<T>(data: T, url: &str) -> Result<(), reqwest::Error>
    where
        T: Serialize,
    {
        let duration = Duration::from_secs(1);
        thread::sleep(duration);

        let client = Client::new();

        // Wandele das generische Objekt in ein JSON-Objekt um
        let json_data = match serde_json::to_value(&data) {
            Ok(json_data) => json_data,
            Err(e) => {
                log_error!("{}", e.to_string());
                return Ok(());
            }
        };

        match client.post(url).form(&json_data).send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Webhook-Anfrage erfolgreich gesendet.");
                    // Hier kannst du die Antwort des Java-Plugins verarbeiten, falls gewünscht.
                } else {
                    println!(
                        "Fehler beim Senden der Webhook-Anfrage: {:?}",
                        response.status()
                    );
                }
            }
            Err(e) => {
                println!("Fehler beim Senden der Webhook-Anfrage: {:?}", e);
            }
        }

        Ok(())
    }
}
