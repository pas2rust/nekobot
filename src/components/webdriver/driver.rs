use core::panic;

use super::import::*;
use super::prelude::*;

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn trans() {
    transporter(&message).await;
})]
pub struct WebDriver {
    #[set(value = "http://localhost:9515")]
    pub host: String,
    pub url: String,
    pub browser: Browser,
    #[set(value = Capabilities::new())]
    pub capabilities: Capabilities,
    pub flow: WorkFlow,
    pub session_id: String,
}


impl WebDriver {
    async fn post<'a, P: Into<String>>(&self, body: &'a Value, path: P) -> HttpResult {
        let path = path.into();
        let client = Client::builder()
            .build()
            .map_err(|e| (self.clone(), e.to_string()))?;
        let base = &self.host.trim_end_matches('/').to_string();
        let endpoint = format!("{base}/{path}");
        let res = client
            .post(endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| (self.clone(), e.to_string()))?;

        Ok(res)
    }

    async fn get<'a, P: Into<String>>(&self, path: P) -> HttpResult {
        let path = path.into();
        let client = Client::builder()
            .build()
            .map_err(|e| (self.clone(), e.to_string()))?;
        let base = &self.host.trim_end_matches('/').to_string();
        let endpoint = format!("{base}/{path}");
        let res = client
            .get(endpoint)
            .send()
            .await
            .map_err(|e| (self.clone(), e.to_string()))?;

        Ok(res)
    }

    pub fn push_flow(&mut self, key: Key, flow: Flow) {
        if let Some((_, vec)) = self.flow.iter_mut().find(|(k, _)| k == &key) {
            if !vec.contains(&flow) {
                vec.push(flow);
            }
        } else {
            self.flow.push((key, vec![flow]));
        }
    }

    pub async fn launch(self) -> WebDriverResult {
        let capabilities_json = self.browser.to_json(&self.capabilities);
        let body = json!({
            "capabilities": {
                "alwaysMatch": capabilities_json,
                "firstMatch": [{}]
            }
        });
        let res = self.post(&body, "session").await?;
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| (self.clone(), e.to_string()))?;
        let session_id = data["value"]["sessionId"].as_str().unwrap_or_default();
        Ok(self.session_id(session_id))
    }

    pub async fn to<U: Into<String>>(mut self, url: U) -> WebDriverResult {
        let url = url.into();
        let session_id = &self.session_id;
        let body = json!({"url": url});
        let res = self
            .post(&body, format!("session/{session_id}/url"))
            .await?
            .status()
            .is_success();
        if !res {
            return Err((self, "response status must be success".to_string()));
        }
        self.push_flow(Key::new().method(Method::To).url(url.clone()), Flow::new());
        Ok(self.url(url).wait(500).await)
    }

    pub async fn wait(self, ms: u64) -> Self {
        sleep(Duration::from_millis(ms)).await;
        self
    }

    pub async fn find(mut self, selector: Selector) -> WebDriverResult {
        let (using, value) = selector.clone().build();
        let body = json!({
            "using": using,
            "value": value
        });

        let session_id = &self.session_id;
        let url = &self.url;
        let res = self
            .post(&body, format!("session/{session_id}/element"))
            .await?;
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| (self.clone(), e.to_string()))?;
        let element_id = data["value"]["element-6066-11e4-a52e-4f735466cecf"]
            .as_str()
            .unwrap_or_default();
        self.push_flow(
            Key::new().method(Method::Find).url(url),
            Flow::new().element_id(element_id).selector(selector),
        );
        Ok(self)
    }

    pub async fn filter(mut self, selector: Selector) -> WebDriverResult {
        let (using, value) = selector.clone().build();
        let body = json!({
            "using": using,
            "value": value
        });
        let session_id = &self.session_id;
        let res = self
            .post(&body, format!("session/{session_id}/elements"))
            .await?;
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| (self.clone(), e.to_string()))?;

        fn extract_element_id(obj: &serde_json::Value) -> Option<String> {
            if !obj.is_object() {
                return None;
            }
            let map = obj.as_object().unwrap();
            if let Some(v) = map.get("element-6066-11e4-a52e-4f735466cecf") {
                return v.as_str().map(|s| s.to_string());
            }
            if let Some(v) = map.get("ELEMENT") {
                return v.as_str().map(|s| s.to_string());
            }
            for (_k, v) in map.iter() {
                if let Some(s) = v.as_str() {
                    return Some(s.to_string());
                }
            }
            None
        }

        let mut pushed_any = false;

        match &data["value"] {
            serde_json::Value::Array(arr) => {
                for item in arr {
                    if let Some(element_id) = extract_element_id(item) {
                        if !element_id.is_empty() {
                            self.push_flow(
                                Key::new().method(Method::Filter).url(self.url.clone()),
                                Flow::new()
                                    .element_id(&element_id)
                                    .selector(selector.clone()),
                            );
                            pushed_any = true;
                        }
                    }
                }
            }

            serde_json::Value::Object(_) => {
                if let Some(element_id) = extract_element_id(&data["value"]) {
                    if !element_id.is_empty() {
                        self.push_flow(
                            Key::new().method(Method::Filter).url(self.url.clone()),
                            Flow::new()
                                .element_id(&element_id)
                                .selector(selector.clone()),
                        );
                        pushed_any = true;
                    }
                }
            }
            _ => {}
        }

        if !pushed_any {
            return Err((self, "push any".to_string()));
        }

        Ok(self)
    }

    pub async fn _loop(&self) {
        let session_id = &self.session_id;
        loop {
            let res = self
                .get(format!("session/{session_id}/url"))
                .await
                .expect("get currrent url failed");
            if !res.status().is_success() {
                panic!(
                    "{}",
                    format!("unexpected status {} from WebDriver", res.status())
                );
            }
            let data = res
                .json::<Value>()
                .await
                .expect("fail convert data to json");
            let current_url = data.get("value").expect("current url must be some");

            for (key, flows) in &self.flow {
                let url = &key.url;
                let method = &key.method;
                for flow in flows {
                    match method {
                        Method::To if current_url != url => match self.clone().to(url).await {
                            Ok(web) => {
                                web.print().info().await;
                            }
                            Err((web, msg)) => {
                                web.print().message(msg).err().await;
                            }
                        },
                        Method::Filter => {
                            if current_url == url {
                                match self.clone().filter(flow.selector.clone()).await {
                                    Ok(web) => {
                                        web.print().info().await;
                                    }
                                    Err((web, msg)) => {
                                        web.print().message(msg).err().await;
                                    }
                                }
                            }
                        }

                        Method::Find => {
                            if current_url == url {
                                match self.clone().find(flow.selector.clone()).await {
                                    Ok(web) => {
                                        web.print().info().await;
                                    }
                                    Err((web, msg)) => {
                                        web.print().message(msg).err().await;
                                    }
                                }
                            }
                        }
                        Method::Click => todo!(),
                        Method::Typing => todo!(),
                        _ => todo!(),
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        }
    }
}
