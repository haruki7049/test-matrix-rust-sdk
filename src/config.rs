use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    homeserver: Url,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            homeserver: Url::parse("https://matrix.org").expect("Failed to parse homeserver Url"),
        }
    }
}

impl Configuration {
    pub fn homeserver(&self) -> Url {
        self.homeserver.clone()
    }
}
