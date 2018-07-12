#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate reqwest;
extern crate rss;
extern crate select;

mod config;
mod utils;
mod xkcd;

use config::read_config;
use std::collections::HashMap;
use utils::GenResult;

enum Feed {
    Xkcd,
}

fn make_channel(feed: &Feed, num_entries: u32) -> GenResult<rss::Channel> {
    let (title, description, link, items) = match feed {
        Feed::Xkcd => xkcd::make(num_entries)?,
    };

    let mut channel = rss::ChannelBuilder::default()
        .title(title)
        .description(description)
        .link(link)
        .build()?;

    channel.set_items(items);

    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert(
        "content".to_string(),
        "http://purl.org/rss/1.0/modules/content/".to_string(),
    );
    channel.set_namespaces(namespaces);

    Ok(channel)
}

fn main() {
    let config = read_config().unwrap();
    for entry in config {
        let feed = match entry.feed_name.as_str() {
            "xkcd" => Feed::Xkcd,
            _ => {
                panic!("Unknown feed name: {}", entry.feed_name);
            }
        };

        let channel = make_channel(&feed, entry.num_entries).unwrap();
        println!("{}", channel.to_string());
    }
}
