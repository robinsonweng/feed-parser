use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use reqwest::{self, Error};
use select::document::Document;
use select::predicate::Name;

pub struct Feed {
    pub src: String,
    pub prev_date: String,
    pub name: String,
    pub notify_to: String,
}

#[async_trait]
pub trait Rss {
    fn get_src(&self) -> &String;
    fn get_prev_date(&self) -> &String;
    fn text_2_date(&self, context: &String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(context).unwrap()
    }

    async fn is_update(&self) -> Result<bool, Error> {
        // send request to get feed xml
        let xml = reqwest::get(self.get_src()).await?.text().await?;
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
