#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate json;
extern crate reqwest;
extern crate rss;
extern crate select;

mod config;
mod thenib;
mod utils;
mod xkcd;

use config::read_config;
use std::collections::HashMap;
use std::fs::File;
use utils::GenResult;

enum Feed {
    Xkcd,
    TheNib,
}

fn make_channel(feed: &Feed, num_entries: u32) -> GenResult<rss::Channel> {
    let (title, description, link, items) = match feed {
        Feed::Xkcd => xkcd::make(num_entries)?,
        Feed::TheNib => thenib::make(num_entries)?,
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

fn write_feed(channel: &rss::Channel, outpath: &str) -> GenResult<()> {
    let file = File::create(outpath)?;
    channel.pretty_write_to(file, b' ', 4)?;
    Ok(())
}

fn main() {
    let config = read_config().unwrap();
    for entry in config {
        if entry.num_entries == 0 {
            continue;
        }

        let feed = match entry.feed_name.as_str() {
            "thenib" => Feed::TheNib,
            "xkcd" => Feed::Xkcd,
            _ => {
                panic!("Unknown feed name: {}", entry.feed_name);
            }
        };

        let channel = make_channel(&feed, entry.num_entries).unwrap();

        write_feed(&channel, entry.out_filename.as_str()).unwrap();
    }
}
