use reqwest::{Client, Response, Error, header::{HeaderMap, HeaderValue, HeaderName}};
use serde_json::{json, Value};
use comrak::{markdown_to_html, ComrakOptions};
use std::str::FromStr;
use crate::utils::Mail;
use log::info;

pub struct MailRelay{
    account: String,
    token: String,
}

impl MailRelay{

    pub fn new(account: &str, token: &str) -> Self {
        MailRelay{
            account: account.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn send(&self, mail: &Mail) -> Result<Response, Error>{
        let html_part = markdown_to_html(&mail.body, &ComrakOptions::default());
        let mail = json!({
            "from": {
                "email": mail.from
            },
            "to": [
                {
                    "email": mail.to
                }
            ],
            "subject": mail.subject,
            "html_part": html_part,
            "text_part": mail.body
        });
        info!("Mail: {}", mail);
        self.post(Some(mail)).await
    }

    async fn post(&self, body: Option<Value>)->Result<Response, Error>{
        let url = format!("https://{}/api/v1/send_emails", self.account);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Accept").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.insert(HeaderName::from_str("Content-Type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.insert(HeaderName::from_str("X-AUTH-TOKEN").unwrap(),
                          HeaderValue::from_str(&self.token).unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        match body{
            Some(value) => {
                let content = serde_json::to_string(&value).unwrap();
                let res = client.post(url).body(content).send().await?;
                Ok(res)
            },
            None => {
                let res = client.post(url).send().await?;
                Ok(res)
            },
        }
    }
}

pub struct Account{
    email: String,
    name: Option<String>,
}

impl Account{
    pub fn new(email: &str, name: Option<&str>) -> Account{
        Account{
            email: email.to_string(),
            name: match name {
                Some(name) => Some(name.to_string()),
                None => None,
            }
        }
    }
}

