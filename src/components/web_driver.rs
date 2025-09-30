use std::time::Duration;

use kenzu::Builder;
use nekoprint::NekoPrint;
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::time::{Instant, sleep};

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn procedure() {
    println!("{message}");
})]
pub struct Args {
    pub binary: String,
    pub headless: bool,
    pub sandbox: bool,
    pub disable_gpu: bool,
    pub user_data_dir: String,
}

impl Args {
    pub fn to_json(&self) -> Value {
        let mut args = Vec::new();
        if self.headless {
            args.push("--headless".to_string());
        }
        if !self.sandbox {
            args.push("--no-sandbox".to_string());
        }
        if self.disable_gpu {
            args.push("--disable-gpu".to_string());
        }
        if !self.user_data_dir.is_empty() {
            args.push(format!("--user-data-dir={}", self.user_data_dir));
        }
        if !self.binary.is_empty() {
            return json!({
                "args": args,
                "binary": self.binary,
            });
        }

        json!({
            "args": args,
        })
    }
}

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn procedure() {
    println!("{message}");
})]
pub struct Capabilities {
    pub browser_version: String,
    pub accept_insecure_certs: bool,
    pub page_load_strategy: String,
    pub unhandled_prompt_behavior: String,
    #[set(value = Args::new())]
    pub args: Args,
}

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

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn procedure() {
    println!("{message}");
})]
pub struct WebDriver {
    #[set(value = "http://localhost:9515")]
    pub url: String,
    pub browser: Browser,
    #[set(value = Capabilities::new())]
    pub capabilities: Capabilities,
    pub flow: Flow,
}

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn procedure() {
    println!("{message}");
})]
pub struct Flow {
    pub url: String,
    pub session_id: String,
    pub element_id: String,
}

pub enum Selector<'a> {
    CSS(&'a str),
}

impl<'a> Selector<'a> {
    fn build(self) -> (&'a str, &'a str) {
        match self {
            Self::CSS(query) => ("css selector", query),
        }
    }
}

impl WebDriver {
    async fn http<'a, P: Into<String>>(&self, body: &'a Value, path: P) -> Result<Response, Error> {
        let path = path.into();
        let client = Client::builder().build()?;
        let base = &self.url.trim_end_matches('/').to_string();
        let endpoint = format!("{base}/{path}");
        let res = client.post(endpoint).json(&body).send().await?;

        Ok(res)
    }

    pub async fn launch(self) -> Result<Self, Error> {
        let capabilities_json = self.browser.to_json(&self.capabilities);
        let body = json!({
            "capabilities": {
                "alwaysMatch": capabilities_json,
                "firstMatch": [{}]
            }
        });
        let res = self.http(&body, "session").await?;
        let data: serde_json::Value = res.json().await?;
        let session_id = data["value"]["sessionId"].as_str().unwrap_or_default();
        Ok(self.flow(Flow::new().session_id(session_id)))
    }

    pub async fn to<U: Into<String>>(self, url: U) -> Result<Self, Error> {
        let url = url.into();
        let session_id = self.flow.session_id.clone();
        let body = json!({"url": url});
        let res = self
            .http(&body, format!("session/{session_id}/url"))
            .await?;
        println!("response to: {res:#?}");
        Ok(self.flow(Flow::new().session_id(session_id).url(url)))
    }

    pub async fn wait(self, ms: u64) -> Result<Self, Error> {
        sleep(Duration::from_millis(ms)).await;
        self.flow
            .print()
            .message(format!("Wait for {ms} ms"))
            .warning()
            .await;
        Ok(self)
    }

    pub async fn find<'a, T: Into<Option<u64>>>(
        self,
        selector: Selector<'a>,
        timeout: T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let timeout = timeout.into().unwrap_or(2);
        let start = Instant::now();
        let (using, value) = selector.build();
        let body = json!({
            "using": using,
            "value": value
        });
        let flow = self.flow.clone();
        let session_id = flow.session_id;
        let url = flow.url;

        loop {
            if let Ok(res) = self
                .http(&body, format!("session/{session_id}/element"))
                .await
            {
                println!("find resp: {res:#?}");
                match res.json().await {
                    Ok(data) => {
                        let data: serde_json::Value = data;
                        println!("find data: {data:#?}");
                        if let Some(element_id) =
                            data["value"]["element-6066-11e4-a52e-4f735466cecf"].as_str()
                        {
                            return Ok(self.flow(
                                Flow::new()
                                    .session_id(session_id)
                                    .element_id(element_id)
                                    .url(url),
                            ));
                        }
                    }
                    Err(err) => {
                        println!("{err:#?}")
                    }
                }
            } else {
                if start.elapsed() >= Duration::from_secs(timeout) {
                    return Err("Timeout error".into());
                }

                sleep(Duration::from_millis(200)).await;
            }
        }
    }

    /*pub async fn filter<'a, T: Into<Option<u64>>>(
        self,
        selector: Selector<'a>,
        timeout: T
    ) -> Result<Self, Box<dyn std::error::Error>> {
       let timeout = timeout.into().unwrap_or(2200);
        let start = Instant::now();
        let (using, value) = selector.build();
        let body = json!({
            "using": using,
            "value": value
        });
        let flow = self.flow.clone();
        let session_id = flow.session_id;
        let url = flow.url;

        loop {
            if let Ok(res) = self
                .http(&body, format!("session/{session_id}/elements"))
                .await
            {
                println!("find resp: {res:#?}");
                match res.json().await {
                    Ok(data) => {
                        let data: serde_json::Value = data;
                        println!("find data: {data:#?}");
                        if let Some(element_id) =
                            data["value"]["element-6066-11e4-a52e-4f735466cecf"].as_str()
                        {
                            return Ok(self.flow(
                                Flow::new()
                                    .session_id(session_id)
                                    .element_id(element_id)
                                    .url(url),
                            ));
                        }
                    }
                    Err(err) => {
                        println!("{err:#?}")
                    }
                }
            } else {
                if start.elapsed() >= Duration::from_secs(timeout) {
                    return Err("Timeout error".into());
                }

                sleep(Duration::from_millis(200)).await;
            }
        }
    }*/
}
