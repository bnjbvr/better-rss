extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Name};

struct Entry {
    src: String,
    title: String,
    hover: String,
}

impl Entry {
    fn new(src: String, title: String, hover: String) -> Self {
        Entry { src, title, hover }
    }
}

fn main() {
    let base_url = String::from("https://xkcd.com");

    let mut entries: Vec<Entry> = Vec::new();

    let mut url: String = base_url.clone();
    for _ in 1..20 {
        let text = reqwest::get(url.as_str()).unwrap();
        let document = Document::from_read(text).unwrap();

        let comic = document.find(Attr("id", "comic")).next().unwrap();

        let img = comic.find(Name("img")).next().unwrap();
        // src has the following format: //img.xkcd.com/4242
        let src = String::from("https:") + img.attr("src").unwrap();
        let title = img.attr("alt").unwrap();
        let hover = img.attr("title").unwrap();

        let e = Entry::new(src.into(), title.into(), hover.into());
        entries.push(e);

        let prev = document.find(Attr("rel", "prev")).next().unwrap();
        let prev_link = prev.attr("href").unwrap();
        url = base_url.clone() + prev_link;
    }

    for entry in &entries {
        println!("{}: {}", entry.title, entry.hover);
        println!("src: {}", entry.src);
        println!();
    }
}