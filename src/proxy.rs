use super::http::HTTP;
use std::collections::HashMap;
use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;

use crate::Result;

pub struct ZAP {
    key: String,
    url: String,
    target: String
}

impl ZAP {
    pub fn new(key: &str, target: &str, url: &str) -> ZAP {
        ZAP {
            key: key.to_string(),
            target: target.to_string(),
            url: url.to_string()
        }
    }

    async fn continue_from_break(&self) -> Result<()> {
        let api_url = format!("{}/JSON/break/action/continue/", self.url);
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("apikey", &self.key);
        let api_req_url = reqwest::Url::parse_with_params(&api_url, params)?;

        let res = reqwest::get(api_req_url).await?
            .json::<HashMap<String, String>>().await?;

        println!("CONTINUE...{:?}", res.get("Result").unwrap());

        Ok(())
    }

    pub async fn add_break(&self) -> Result<()> {
        let api_url = format!("{}/JSON/break/action/addHttpBreakpoint/", self.url);

        let mut params = HashMap::<&str, &str>::new();

        params.insert("apikey", self.key.as_str());
        params.insert("string", self.target.as_str());
        params.insert("location", "url");
        params.insert("match", "contains");
        params.insert("inverse", "false");
        params.insert("ignorecase", "false");

        let req_url = reqwest::Url::parse_with_params(&api_url, &params)?;

        let res = reqwest::get(req_url).await?
            .json::<HashMap<String, String>>().await?;

        println!("MK BREAK {:?}", res.get("Result").unwrap());

        Ok(())
    }

    pub async fn remove_break(&self) -> Result<()> {
        let api_url = format!("{}/JSON/break/action/removeHttpBreakpoint/", self.url);


        let mut params = HashMap::<&str, &str>::new();

        // Continue breakpoint twice, once for request, and once for response
        for _ in 0..2 {
            self.continue_from_break().await?;
        }

        params.insert("apikey", self.key.as_str());
        params.insert("string", self.target.as_str());
        params.insert("location", "url");
        params.insert("match", "contains");
        params.insert("inverse", "false");
        params.insert("ignorecase", "false");

        let req_url = reqwest::Url::parse_with_params(&api_url, &params)?;

        let res = reqwest::get(req_url).await?
            .json::<HashMap<String, String>>().await?;

        println!("RM BREAK {:?}", res.get("Result").unwrap());

        Ok(())
    }

    pub async fn get_http(&self) -> Result<HTTP> {
        let api_url = format!("{}/JSON/break/view/httpMessage/", self.url);
        let mut http_message = HTTP::new();

        let mut params = HashMap::<&str, &str>::new();
        params.insert("apikey", self.key.as_str());

        let req_url = reqwest::Url::parse_with_params(&api_url, &params)?;

        let res = reqwest::get(req_url).await?
            .json::<HashMap<String, String>>().await?;

        let mut http_strings = res.get("httpMessage").unwrap()
            .split("\r\n")
            .filter(|s| !s.is_empty());

        http_message.request = http_strings.next().unwrap().to_string();
        http_message.headers = http_strings.map(|s| s.to_string()).collect();

        Ok(http_message)
    }

    pub async fn inject_sql(&self, req: &HTTP, param: &str, cmd: &str) -> Result<()> {
        let req_fragments: Vec<String> = req.request.split_inclusive(param)
            .map(|s| String::from(s))
            .collect();

        let mut injected_req = HTTP {
            request: format!("{}' {cmd}--%20{}", req_fragments[0], req_fragments[1]),
            headers: req.headers.clone()
        };

        injected_req.request = injected_req.request.strip_prefix("GET ").unwrap().to_string();
        injected_req.request = injected_req.request.strip_suffix(" HTTP/1.1").unwrap().to_string();

        let client = Client::builder()
            .proxy(reqwest::Proxy::http(self.url.clone())?)
            .build()?;
        
        let mut header_map = HeaderMap::new();

        for header in injected_req.headers {
            let fragments: Vec<&str> = header.splitn(2, ": ").collect();

            header_map.insert(
                HeaderName::from_str(fragments[0])?,
                HeaderValue::from_str(fragments[1])?
            );
        }

        let res = client.get(injected_req.request)
            .headers(header_map)
            .send().await?;

        println!("INJECT: {:?}", res);

        webbrowser::open(&res.url().as_str())?;

        Ok(())
    }
}
