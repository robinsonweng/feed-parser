use error_chain::error_chain;
// use feed_rs::parser;
use futures_util::StreamExt;
use reqwest;
use std::slice;
use tokio;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url: &str = "https://www.readfog.com/feed";
    let mut stream = reqwest::get(url).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let item = item?;
        let xml = String::from_utf8_lossy(&item);

        let target: &str = "pubDate";

        let slen: usize = xml.len();
        let mut i: usize = 7;

        let temp: Vec<&str> = xml.split("\n").collect();
        for t in temp {
            if t.contains(target) {
                println!("{}", t.replace("<pubDate>", ""));
                break;
            }
        }
        break;
    }

    Ok(())
}
