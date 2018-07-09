extern crate reqwest;
extern crate select;
extern crate rss;
extern crate regex;

use std::collections::HashMap;

use select::document::Document;
use select::predicate::{Attr, Name};

use regex::Regex;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn xkcd(num_entries: u32) -> GenResult<Vec<rss::Item>> {
    let base_url = String::from("https://xkcd.com");
    let number_re = Regex::new(r"(?P<id>\d{1,})")?;

    let mut items: Vec<rss::Item> = Vec::new();

    let mut url: String = base_url.clone();
    for i in 1..(num_entries + 1) {
        eprintln!("fetching item #{}...", i);

        let text = reqwest::get(url.as_str())?;
        let document = Document::from_read(text)?;

        let comic = document.find(Attr("id", "comic")).next().ok_or("unable to find #comic")?;

        let img = comic.find(Name("img")).next().ok_or("finding <img>")?;
        // src has the following format: //img.xkcd.com/4242
        let src = String::from("https:") + img.attr("src").ok_or("reading img src")?;
        let title = img.attr("alt").ok_or("reading img alt")?.to_string();
        let hover = img.attr("title").ok_or("reading img title")?;

        let prev = document.find(Attr("rel", "prev")).next().ok_or("finding prev link")?;
        let prev_link = prev.attr("href").ok_or("reading link href")?;

        let prev_id = number_re
            .captures(prev_link)
            .ok_or("couldn't find previous link numeric id")?
            .get(0)
            .ok_or("couldn't find previous link numeric id")?
            .as_str();

        let prev_id_num = prev_id.parse::<u32>()?;
        let link = String::from(format!("https://xkcd.com/{}", prev_id_num + 1));

        let item = rss::ItemBuilder::default()
            .title(title)
            .link(link)
            .content(format!("<img src='{}' /><p>{}</p>", src, hover))
            .build()?;

        items.push(item);

        // Now move on to the next item.
        url = base_url.clone() + prev_link;
    }

    return Ok(items);
}

fn main() {
    let items = match xkcd(5) {
        Ok(items) => { items }
        Err(err) => {
            println!("An error occurred while making a feed for xkcd, aborting: {}", err);
            Vec::new()
        }
    };

    let mut channel = rss::ChannelBuilder::default()
        .title("XKCD feed")
        .link("http://xkcd.com")
        .description("A better rss feed for xkcd.")
        .build()
        .unwrap();

    channel.set_items(items);

    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert("content".to_string(), "http://purl.org/rss/1.0/modules/content/".to_string());
    channel.set_namespaces(namespaces);

    println!("{}", channel.to_string());
}
