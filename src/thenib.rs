use reqwest;
use rss;
use select::document::Document;
use select::predicate::Attr;
use utils::{Channel, GenResult};

pub fn make(num_entries: u32) -> GenResult<Channel> {
    let rss_url = String::from("https://thenib.com/feeds/rss");
    let rss_text = reqwest::get(rss_url.as_str())?.text()?;
    let channel = rss_text.parse::<rss::Channel>()?;

    eprintln!("Starting to fetch TheNib...");

    let mut items: Vec<rss::Item> = Vec::new();
    for (i, entry) in channel.items().iter().enumerate() {
        eprintln!("fetching item #{}...", i + 1);
        let title = String::from(entry.title().ok_or("missing title")?);
        let link = String::from(entry.link().ok_or("missing link")?);
        let pub_date = String::from(entry.pub_date().ok_or("missing date")?);

        let page = reqwest::get(link.as_str())?;
        let document = Document::from_read(page)?;

        let content = document
            .find(Attr("class", "Layout-layoutContainer"))
            .next()
            .ok_or("couldn't find content")?
            .text();

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

    Ok((
        "The Nib",
        "RSS feed for The Nib that includes full content.",
        "https://thenib.com",
        items,
    ))
}
