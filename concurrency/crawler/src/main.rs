use std::sync::Arc;

use tokio;
use crawler::{Crawler, Config};

#[tokio::main]
async fn main() {
    let mut receiver = Crawler::new(Config::default()).run("https://blog.logrocket.com/".to_string()).await;
    while let Some(page) = receiver.recv().await {
        println!("{}", page.url);
        println!("{}", page.body);
    }
}