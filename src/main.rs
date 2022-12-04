use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use error_chain::error_chain;
use reqwest::{self};
use select::document::Document;
use select::predicate::Name;
use tokio;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

struct Feed {
    src: String,
    prev_date: String,
    name: String,
    notify_to: String,
}

#[async_trait]
pub trait Rss {
    async fn is_update(&self) -> Result<bool>;
    fn text_2_date(&self, context: &String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc2822(context).unwrap()
    }
}

#[async_trait]
impl Rss for Feed {
    async fn is_update(&self) -> Result<bool> {
        // send request to get feed xml
        let xml = reqwest::get(&self.src).await?.text().await?;
        let document = Document::from(xml.as_str());
        // iterate first item in xml, get it's datetime
        let item = document.find(Name("item")).next().unwrap();
        let item_pubdate = item.find(Name("pubdate")).next().unwrap().text();

        let pubdate = self.text_2_date(&item_pubdate);
        let old_date = self.text_2_date(&self.prev_date);

        println!("old date: {}", old_date);
        println!("pubdate:  {}", pubdate);
        if pubdate > old_date {
            return Ok(true);
        }
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let feed = Feed {
        src: String::from("https://www.readfog.com/feed"),
        prev_date: String::from("Thu, 01 Dec 2022 06:37:37 +0000"),
        name: String::from("閱坊"),
        notify_to: String::from("discord"),
    };

    if feed.is_update().await? {
        println!("new post received");
    } else {
        println!("false!");
    }

    Ok(())
}
