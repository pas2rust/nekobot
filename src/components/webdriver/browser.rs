use super::import::*;
use super::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum Browser {
    #[default]
    Chrome,
    Firefox,
}

impl Browser {
    pub fn to_json(&self, capabilities: &Capabilities) -> Value {
        match self {
            Browser::Chrome => {
                let chrome_options = capabilities.args.to_json();
                json!({
                    "goog:chromeOptions": chrome_options,
                    "browserName": "chrome",
                    "platformName": "linux",
                })
            }
            Browser::Firefox => {
                let firefox_options = capabilities.args.to_json();
                json!({
                    "moz:firefoxOptions": firefox_options,
                    "browserName": "firefox",
                    "platformName": "linux",
                })
            }
        }
    }
}
