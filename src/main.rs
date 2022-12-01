use futures_util::StreamExt;

use chrono::{DateTime, Local, TimeZone, Utc};
use error_chain::error_chain;
use reqwest::{self};
use tokio;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

struct Website {
    src: String,
    prev_date: String,
    name: String,
    notify_to: String,
}

trait Feed {
    fn is_update() -> Result<()>;
}

impl Website {
    async fn is_update(&self) -> Result<()> {
        let mut stream = reqwest::get(&self.src).await?.bytes_stream();

        while let Some(item) = stream.next().await {
            let item = item?;
            let xml = String::from_utf8_lossy(&item);

            let target: &str = "pubDate";
            let current_time = String::from("Wed, 30 Nov 2022 16:46:18 +0800");
            let current_time = DateTime::parse_from_rfc2822(&current_time);

            let temp: Vec<&str> = xml.split("\n").collect();
            for t in temp {
                if t.contains(target) {
                    let feed_time = DateTime::parse_from_rfc2822(
                        t.replace("<pubDate>", "")
                            .replace("</pubDate>", "")
                            .replace("\n", "")
                            .trim(),
                    );
                    if feed_time == current_time {
                        println!("eq!");
                    } else if feed_time.unwrap() > current_time.unwrap() {
                        println!("gt!");
                    } else if feed_time.unwrap() < current_time.unwrap() {
                        println!("st!");
                    }
                    println!("feed: {:?}", feed_time.unwrap().to_rfc2822());
                    println!("current: {:?}", current_time.unwrap().to_rfc2822());
                    break;
                }
            }
            break;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut website = Website {
        src: String::from("https://www.readfog.com/feed"),
        prev_date: String::from("123"),
        name: String::from("閱坊"),
        notify_to: String::from("discord"),
    };
    let update_status = website.is_update().await?;
    Ok(())
}
