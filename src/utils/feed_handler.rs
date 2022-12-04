use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use reqwest::{self, Error};
use select::document::{self, Document};
use select::predicate::Name;

pub struct Feed {
    pub src: String,
    pub prev_date: String,
    pub name: String,
    pub notify_to: String,
}

async fn send_request(url: &String) -> Result<String, Error> {
    let xml = reqwest::get(url).await?.text().await?;
    Ok(xml)
}

#[async_trait]
pub trait Rss {
    async fn new(url: &str, webhook: &str) -> Result<Feed, Error> {
        let xml = send_request(&String::from(url)).await?;
        let document = Document::from(xml.as_str());

        let channel = document.find(Name("channel")).next().unwrap();
        let title = channel.find(Name("title")).next().unwrap().text();

        let item = document.find(Name("item")).next().unwrap();
        let new_date = item.find(Name("pubdate")).next().unwrap().text();

        Ok(Feed {
            src: String::from(url),
            prev_date: new_date,
            name: title,
            notify_to: String::from(webhook),
        })
    }

    // getters
    fn get_src(&self) -> &String;
    fn get_prev_date(&self) -> &String;

    // setters
    fn set_prev_date(&self);
    fn set_name(&self, value: &str);

    fn text_2_date(&self, context: &String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(context).unwrap()
    }

    async fn is_update(&self) -> Result<bool, Error> {
        // send request to get feed xml
        let xml = send_request(&self.get_src()).await?;
        let document = Document::from(xml.as_str());
        // iterate first item in xml, get it's datetime
        let item = document.find(Name("item")).next().unwrap();
        let item_pubdate = item.find(Name("pubdate")).next().unwrap().text();

        let pubdate = self.text_2_date(&item_pubdate);
        let old_date = self.text_2_date(&self.get_prev_date());

        println!("old date: {}", old_date);
        println!("pubdate:  {}", pubdate);

        if pubdate > old_date {
            return Ok(true);
        }
        Ok(false)
    }
}
