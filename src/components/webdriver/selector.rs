use super::import::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Selector {
    #[default]
    None,
    CSS(String),
}

impl Selector {
    pub fn css(value: &str) -> Self {
        Self::CSS(value.to_string())
    }
    pub fn build<'a>(self) -> (&'a str, String) {
        match self {
            Self::None => todo!(),
            Self::CSS(query) => ("css selector", query),
        }
    }
}
