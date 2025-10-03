use super::import::*;
use super::prelude::*;

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone)]
#[transporter(async fn trans() {
    transporter(&message).await;
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
