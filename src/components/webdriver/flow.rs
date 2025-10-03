use super::import::*;
use super::prelude::*;

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone, PartialEq, Eq)]
#[transporter(async fn trans() {
    transporter(&message).await;
})]
pub struct Flow {
    pub element_id: String,
    pub selector: Selector,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub enum Method {
    #[default]
    To,
    Find,
    Filter,
    Typing,
    Click,
}

#[derive(Debug, Builder, Serialize, Deserialize, NekoPrint, Default, Clone, PartialEq, Eq)]
#[transporter(async fn trans() {
    transporter(&message).await;
})]
pub struct Key {
    pub url: String,
    pub method: Method,
}

pub type WorkFlow = Vec<(Key, Vec<Flow>)>;
