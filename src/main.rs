#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate json;
extern crate reqwest;
extern crate rss;
extern crate select;

mod config;
mod loadingartist;
mod thenib;
mod utils;
mod xkcd;
mod instagram;

use config::read_config;
use std::collections::HashMap;
use std::fs::File;
use utils::GenResult;

enum Feed {
    Xkcd,
    TheNib,
    LoadingArtist,
    Instagram(String),
}

fn make_channel(feed: &Feed, num_entries: u32) -> GenResult<rss::Channel> {
    let (title, description, link, items) = match feed {
        Feed::Xkcd => xkcd::make(num_entries)?,
        Feed::TheNib => thenib::make(num_entries)?,
        Feed::LoadingArtist => loadingartist::make(num_entries)?,
        Feed::Instagram(account_name) => instagram::make(&account_name, num_entries)?,
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
            "loadingartist" => Feed::LoadingArtist,
            "instagram" => {
                if let Some(name) = entry.account_name {
                    Feed::Instagram(name)
                } else {
                    panic!("Missing instagram account name");
                }
            }
            _ => {
                panic!("Unknown feed name: {}", entry.feed_name);
            }
        };

        let channel = match make_channel(&feed, entry.num_entries) {
            Ok(channel) => channel,
            Err(err) => {
                eprintln!("Error when making channel {}: {}", entry.feed_name, err);
                continue;
            }
        };

        write_feed(&channel, entry.out_filename.as_str()).unwrap();
    }
}
