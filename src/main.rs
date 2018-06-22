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

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn xkcd(num_entries: u32) -> GenResult<Vec<Entry>> {
    let base_url = String::from("https://xkcd.com");

    let mut entries: Vec<Entry> = Vec::new();

    let mut url: String = base_url.clone();
    for _ in 1..num_entries {
        let text = reqwest::get(url.as_str())?;
        let document = Document::from_read(text)?;

        let comic = document.find(Attr("id", "comic")).next().ok_or("unable to find #comic")?;

        let img = comic.find(Name("img")).next().ok_or("finding <img>")?;
        // src has the following format: //img.xkcd.com/4242
        let src = String::from("https:") + img.attr("src").ok_or("reading img src")?;
        let title = img.attr("alt").ok_or("reading img alt")?;
        let hover = img.attr("title").ok_or("reading img title")?;

        let e = Entry::new(src.into(), title.into(), hover.into());
        entries.push(e);

        let prev = document.find(Attr("rel", "prev")).next().ok_or("finding prev link")?;
        let prev_link = prev.attr("href").ok_or("reading link href")?;
        url = base_url.clone() + prev_link;
    }

    return Ok(entries);
}

fn main() {
    let entries = match xkcd(20) {
        Ok(entries) => { entries }
        Err(err) => {
            println!("An error occurred while making a feed for xkcd, aborting: {}", err);
            Vec::new()
        }
    };

    for entry in &entries {
        println!("{}: {}", entry.title, entry.hover);
        println!("src: {}", entry.src);
        println!();
    }
}
