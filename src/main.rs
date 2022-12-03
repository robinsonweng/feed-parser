use async_trait::async_trait;
use chrono::DateTime;
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
trait Rss {
    async fn is_update(&self) -> Result<bool>;
}

#[async_trait]
impl Rss for Feed {
    async fn is_update(&self) -> Result<bool> {
        let xml = reqwest::get(&self.src).await?.text().await?;
        let document = Document::from(xml.as_str());
        let item = document.find(Name("item")).next().unwrap();
        let item_pubdate = item.find(Name("pubdate")).next().unwrap().text();

        let pubdate = DateTime::parse_from_rfc2822(&item_pubdate).unwrap();
        let old_date = DateTime::parse_from_rfc2822(&self.prev_date).unwrap();

        println!("old date:     {}", old_date);
        println!("pubdate:      {}", pubdate);
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
