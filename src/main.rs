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

fn unwrap_res<R, T: std::fmt::Display>(result: Result<R, T>, msg: &str)
    -> Result<R, ()>
{
    match result {
        Ok(x) => {
            Ok(x)
        }
        Err(err) => {
            println!("Encountered an error while {}: {}", msg.to_string(), err);
            Err(())
        }
    }
}

fn unwrap_opt<R>(opt: Option<R>, msg: &str)
    -> Result<R, ()>
{
    match opt {
        Some(x) => {
            Ok(x)
        }
        None => {
            println!("Expected something while {}, got nothing.", msg);
            Err(())
        }
    }
}

fn xkcd(num_entries: u32) -> Result<Vec<Entry>, ()> {
    let base_url = String::from("https://xkcd.com");

    let mut entries: Vec<Entry> = Vec::new();

    let mut url: String = base_url.clone();
    for _ in 1..num_entries {
        let text = unwrap_res(reqwest::get(url.as_str()), "retrieving the http content")?;
        let document = unwrap_res(Document::from_read(text), "parsing HTML")?;

        let comic = unwrap_opt(document.find(Attr("id", "comic")).next(), "finding #comic")?;

        let img = unwrap_opt(comic.find(Name("img")).next(), "finding <img>")?;
        // src has the following format: //img.xkcd.com/4242
        let src = String::from("https:") + unwrap_opt(img.attr("src"), "reading img src")?;
        let title = unwrap_opt(img.attr("alt"), "reading img alt")?;
        let hover = unwrap_opt(img.attr("title"), "reading img title")?;

        let e = Entry::new(src.into(), title.into(), hover.into());
        entries.push(e);

        let prev = unwrap_opt(document.find(Attr("rel", "prev")).next(), "finding prev link")?;
        let prev_link = unwrap_opt(prev.attr("href"), "reading link href")?;
        url = base_url.clone() + prev_link;
    }

    return Ok(entries);
}

fn main() {
    let entries = match xkcd(20) {
        Ok(entries) => { entries }
        Err(_) => {
            println!("An error occurred while making a feed for xkcd, aborting.");
            Vec::new()
        }
    };

    for entry in &entries {
        println!("{}: {}", entry.title, entry.hover);
        println!("src: {}", entry.src);
        println!();
    }
}
