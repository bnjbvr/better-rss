use select::document::Document;
use select::predicate::{Attr, Descendant, Name};
use utils::{from_feed, Channel, ContentCreator, GenResult};

const BASE_URL: &str = "https://loadingartist.com";

struct LoadingArtist {}

impl ContentCreator for LoadingArtist {
    fn create(&self, doc: &Document) -> Option<String> {
        let img = doc
            .find(Descendant(Attr("class", "comic"), Name("img")))
            .next();
        let img_src = match img {
            Some(node) => match node.attr("src") {
                Some(src) => src,
                None => {
                    eprintln!("couldn't find img src");
                    return None;
                }
            },
            None => {
                return None;
            }
        };
        Some(format!("<img src=\"{}{}\" />", BASE_URL, img_src))
    }
}

pub fn make(num_entries: u32) -> GenResult<Channel> {
    let rss_url = String::from("https://loadingartist.com/feed/");
    eprintln!("Starting to fetch LoadingArtist...");

    let items = from_feed(&rss_url, num_entries, LoadingArtist {})?;

    Ok((
        "Loading Artist".into(),
        "RSS feed for Loading Artist that includes full content.".into(),
        "https://loadingartist.com".into(),
        items,
    ))
}
