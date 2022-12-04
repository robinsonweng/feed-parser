mod utils;
use async_trait::async_trait;
use error_chain::error_chain;
use tokio;
use utils::feed_handler::{Feed, Rss};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[async_trait]
impl Rss for Feed {
    fn get_src(&self) -> &String {
        &self.src
    }
    fn get_prev_date(&self) -> &String {
        &self.prev_date
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
