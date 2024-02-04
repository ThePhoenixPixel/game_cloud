use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use serde::Serialize;

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
        let json_data = serde_json::to_value(&data)?;

        match client.post(url).json(&json_data).send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Webhook-Anfrage erfolgreich gesendet.");
                    // Hier kannst du die Antwort des Java-Plugins verarbeiten, falls gewünscht.
                } else {
                    println!("Fehler beim Senden der Webhook-Anfrage: {:?}", response.status());
                }
            }
            Err(e) => {
                println!("Fehler beim Senden der Webhook-Anfrage: {:?}", e);
            }
        }

        Ok(())
    }

}
