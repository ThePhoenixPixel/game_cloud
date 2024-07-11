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
}