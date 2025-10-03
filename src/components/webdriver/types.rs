use reqwest::Response;

use crate::components::prelude::WebDriver;

pub type WebDriverError = (WebDriver, String);
pub type HttpResult = Result<Response, WebDriverError>;
pub type WebDriverResult = Result<WebDriver, WebDriverError>;
pub type MainWebDriverResult = Result<(), WebDriverError>;