use tokio::{spawn, sync::{Semaphore, mpsc::{ channel, Receiver, Sender}}};
use std::sync::{ Arc, Mutex };
use std::collections::{ HashSet };
use linkify;

#[derive(Clone, Default)]
pub struct Config {
    pub concurrent_requests: Option<usize>,
}

pub struct Page {
    pub url: String,
    pub body: String,
}

pub struct Crawler {
    config: Config,
}

impl Crawler {
    pub fn new(config: Config) -> Self {
        Crawler { config }
    }

    pub fn run(&mut self, site: String) -> Receiver<Page> {
        let (sender, receiver) = channel::<Page>(10);
        let visited_urls = Arc::new(Mutex::new(HashSet::<String>::from([site.clone()])));
        let connection_permits = Arc::new(Semaphore::new(self.config.concurrent_requests.unwrap_or(1)));
        Self::spawn_crawle(site.clone(), site, sender, visited_urls, connection_permits);
        receiver
    }

    async fn crawle(site: String, url: String, sender: Sender<Page>, visited_urls: Arc<Mutex<HashSet<String>>>, connection_permits: Arc<Semaphore>) {
        let body = {
            let _permit = connection_permits.acquire().await.unwrap();
            reqwest::get(&url).await.unwrap().text().await.unwrap()
        };

        let links = linkify::LinkFinder::new()
            .links(&body)
            .filter(|link| link.as_str().starts_with(&site))
            .map(|link| link.as_str().to_string())
            .collect::<Vec<_>>();
        if sender.send(Page { url, body } ).await.is_ok() {
            for url in links.into_iter() {
                if visited_urls.lock().unwrap().insert(url.clone()) {
                    Self::spawn_crawle(site.clone(), url, sender.clone(), visited_urls.clone(), connection_permits.clone());
                }
            }
        }
    }

    fn spawn_crawle(site: String, url: String, sender: Sender<Page>, visited_urls: Arc<Mutex<HashSet<String>>>, connection_permits: Arc<Semaphore>) {
        spawn( Crawler::crawle(site, url, sender, visited_urls, connection_permits));
    }
}
