use select::document::Document;
use select::predicate::Attr;
use utils::{from_feed, Channel, ContentCreator, GenResult};

struct TheNib;

impl ContentCreator for TheNib {
    fn create(&self, doc: &Document) -> Option<String> {
        match doc.find(Attr("class", "Layout-layoutContainer")).next() {
            Some(div) => Some(div.text()),
            None => None,
        }
    }
}

pub fn make(num_entries: u32) -> GenResult<Channel> {
    let rss_url = String::from("https://thenib.com/feeds/rss");
    eprintln!("Starting to fetch TheNib...");

    let items = from_feed(&rss_url, num_entries, TheNib {})?;

    Ok((
        "The Nib",
        "RSS feed for The Nib that includes full content.",
        "https://thenib.com",
        items,
    ))
}
