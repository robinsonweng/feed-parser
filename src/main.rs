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
    // initers
    fn init_name(url: &str) -> String {
        !todo!()
    }
    fn init_date(url: &str) -> String {
        !todo!()
    }

    // gettrers
    fn get_src(&self) -> &String {
        &self.src
    }
    fn get_prev_date(&self) -> &String {
        &self.prev_date
    }

    // setters
    fn set_prev_date(&self) {
        !todo!()
    }
    fn set_name(&self) {
        !todo!()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.readfog.com/feed";
    let webhook = "123";

    let feed = Feed::new(&url, &webhook);
    if feed.is_update().await? {
        println!("new post received");
    } else {
        println!("false!");
    }

    Ok(())
}
