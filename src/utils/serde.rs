pub struct Serde;

impl Serde {
    pub fn string_to_json_value(json_string: &str) -> serde_json::Value {
        serde_json::from_str(json_string).expect("Fehler beim Deserialisieren des JSON-Strings")
    }
}
