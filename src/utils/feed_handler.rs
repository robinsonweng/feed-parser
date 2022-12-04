use std::error;

use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use reqwest::{self, Error};
use select::document::Document;
use select::predicate::Name;

pub struct Feed {
    pub src: String,
    pub prev_date: Option<String>,
    pub name: Option<String>,
    pub notify_to: Option<String>,
}

#[async_trait]
pub trait Rss {
    fn new(url: &str, webhook: &str) -> Feed {
        Feed {
            src: String::from(url),
            prev_date: None,
            name: String::from("閱坊"),
            notify_to: String::from("discord"),
        }
    }

    // initers
    fn init_name(url: &str) -> String;
    fn init_date(url: &str) -> String;

    // getters
    fn get_src(&self) -> &String;
    fn get_prev_date(&self) -> &String;

    // setters
    fn set_prev_date(&self);
    fn set_name(&self);

    fn text_2_date(&self, context: &String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(context).unwrap()
    }

    async fn send_request(&self) -> Result<String, Error> {
        let xml = reqwest::get(self.get_src()).await?.text().await?;
        Ok(xml)
    }

    async fn is_update(&self) -> Result<bool, Error> {
        // send request to get feed xml
        let xml = self.send_request().await?;
        let document = Document::from(xml.as_str());
        // iterate first item in xml, get it's datetime
        let item = document.find(Name("item")).next().unwrap();
        let item_pubdate = item.find(Name("pubdate")).next().unwrap().text();

        let pubdate = self.text_2_date(&item_pubdate);
        let old_date = self.text_2_date(self.get_prev_date());

        println!("old date: {}", old_date);
        println!("pubdate:  {}", pubdate);
        if pubdate > old_date {
            return Ok(true);
        }
        Ok(false)
    }
}
