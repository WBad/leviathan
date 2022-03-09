use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::error::Error;
use std::fmt;

fn default_optional<T>() -> Option<T> {
    None
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum FutureTagField {
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "semiannual")]
    SemiAnnual,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Future {
    pub tag: FutureTagField,
    pub pair: String,
    pub symbol: String,
    pub mark_price: f64,
    pub bid: f64,
    pub bid_size: u32,
    pub ask: f64,
    pub ask_size: u32,
    pub vol24h: u32,
    pub open_interest: f64,
    pub open24h: f64,
    pub index_price: f64,
    pub last: f64,
    pub last_time: DateTime<Utc>,
    pub last_size: u32,
    pub suspended: bool,
    #[serde(default = "default_optional")]
    pub funding_rate: Option<f64>,
    #[serde(default = "default_optional")]
    pub funding_rate_prediction: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Index {
    pub symbol: String,
    pub last: f64,
    pub last_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Ticker {
    Index(Index),
    Future(Future),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SuccessResponse {
    pub server_time: DateTime<Utc>,
    pub tickers: Vec<Ticker>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ErrorResponse {
    pub server_time: DateTime<Utc>,
    pub error: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, fmter: &mut fmt::Formatter) -> fmt::Result {
        write!(fmter, "{}", self.error)
    }
}

impl Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.error
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, tag = "result")]
pub enum Response {
    #[serde(rename = "success")]
    Success(SuccessResponse),
    #[serde(rename = "error")]
    Error(ErrorResponse),
}

#[cfg(test)]
mod tests {
    // TODO: Move to cucumber
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn it_handles_success_response() -> Result<(), String> {
        let file = File::open("tests/data/tickers_success.json").unwrap();
        let reader = BufReader::new(file);

        let response: Response = serde_json::from_reader(reader).unwrap();
        match response {
            Response::Success(_) => Ok(()),
            Response::Error(_) => {
                Err("Expected a `SuccessResponse` but got a `ErrorResponse` instead".to_owned())
            }
        }
    }

    #[test]
    fn it_handles_error_response() -> Result<(), String> {
        let file = File::open("tests/data/tickers_error.json").unwrap();
        let reader = BufReader::new(file);

        let response: Response = serde_json::from_reader(reader).unwrap();
        match response {
            Response::Success(_) => {
                Err("Expected a `ErrorResponse` but got a `SuccessResponse` instead".to_owned())
            }
            Response::Error(_) => Ok(()),
        }
    }
}
