pub mod response;

use crate::response::ticker::{Future, Response as TickerResponse, Ticker};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response as ReqwestResponse};
use std::error::Error;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use url::Url;

// let api_key: &str = "YW3D9H+GOKvW8xOM5hUhVaAQvvbwcBVC6YEJhPbdwmdE9Bd5x5ulZy/3";

fn request_tickers() -> Result<String, Box<dyn Error>> {
    let base_url: Url = Url::parse("https://futures.kraken.com").unwrap();
    let tickers_path: &str = "derivatives/api/v3/tickers";

    let request_url: Url = base_url.join(tickers_path).unwrap();

    let client: Client = Client::new();
    let response: ReqwestResponse = client.get(request_url.to_string()).send()?;
    let response_body: String = response.text()?;

    Ok(response_body)
}

fn parse_tickers_body(http_body: &str) -> Result<TickerResponse, Box<dyn Error>> {
    let response: TickerResponse = serde_json::from_str(http_body)?;
    Ok(response)
}

fn get_ticker_response() -> Result<TickerResponse, Box<dyn Error>> {
    let http_body = request_tickers()?;
    parse_tickers_body(&http_body)
}

pub fn get_server_time() -> Result<DateTime<Utc>, Box<dyn Error>> {
    let response: TickerResponse = get_ticker_response()?;
    match response {
        TickerResponse::Success(s) => Ok(s.server_time),
        TickerResponse::Error(e) => Ok(e.server_time),
    }
}

pub fn get_xbt_future() -> Result<Future, Box<dyn Error>> {
    let response: TickerResponse = get_ticker_response()?;
    match response {
        TickerResponse::Success(s) => {
            let rez: Option<Future> = s.tickers.iter().find_map(|tic| match &&tic {
                Ticker::Future(f) => {
                    if f.symbol != "pi_xbtusd" {
                        return None;
                    }
                    Some(f.clone())
                }
                _ => None,
            });

            match rez {
                Some(f) => Ok(f),
                None => {
                    let err = IoError::new(IoErrorKind::Other, "Could not find pi_xbtusd future");
                    Err(Box::new(err))
                }
            }
        }
        TickerResponse::Error(e) => Err(Box::new(e)),
    }
}
