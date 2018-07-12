use reqwest;
use rss;
use std;

use select::document::Document;

pub type GenError = Box<std::error::Error>;
pub type GenResult<T> = Result<T, GenError>;
pub type Channel = (&'static str, &'static str, &'static str, Vec<rss::Item>);

pub trait ContentCreator {
    fn create(&self, doc: &Document) -> Option<String>;
}

pub fn from_feed<T: ContentCreator>(
    rss_url: &str,
    num_entries: u32,
    extractor: T,
) -> GenResult<Vec<rss::Item>> {
    let rss_text = reqwest::get(rss_url)?.text()?;
    let channel = rss_text.parse::<rss::Channel>()?;

    let mut items: Vec<rss::Item> = Vec::new();
    for (i, entry) in channel.items().iter().enumerate() {
        eprintln!("fetching item #{}...", i + 1);
        let title = String::from(entry.title().ok_or("missing title")?);
        let link = String::from(entry.link().ok_or("missing link")?);
        let pub_date = String::from(entry.pub_date().ok_or("missing date")?);

        let page = reqwest::get(link.as_str())?;
        let document = Document::from_read(page)?;

        let content = match extractor.create(&document) {
            Some(content) => content,
            None => {
                eprintln!("Didn't get content for link {}", link);
                continue;
            }
        };

        let item = rss::ItemBuilder::default()
            .title(title)
            .link(link)
            .content(content)
            .pub_date(pub_date)
            .build()?;

        items.push(item);
        if items.len() >= num_entries as usize {
            break;
        }
    }

    Ok(items)
}
