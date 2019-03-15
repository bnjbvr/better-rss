use chrono;
use chrono::prelude::*;
use reqwest;
use rss;
use utils::{Channel, GenResult};

#[derive(Deserialize)]
struct RssBridgeEntry {
    url: String,
    content_html: String,
    title: String,
    date_modified: String,
}

#[derive(Deserialize)]
struct RssBridgeResult {
    items: Vec<RssBridgeEntry>,
}

static BASE: &str = "https://wtf.roflcopter.fr/rss-bridge/index.php?action=display&bridge=Instagram&media_type=all&format=Json";

pub fn make(account_name: &str, num_entries: u32) -> GenResult<Channel> {
    let rss_url = format!("{}&u={}", BASE, account_name);
    eprintln!("Starting to fetch Instagram account {}...", account_name);

    let channel = reqwest::get(&rss_url)?.json::<RssBridgeResult>()?;

    let mut items = Vec::new();
    for (i, entry) in channel.items.iter().enumerate() {
        eprintln!("fetching item #{}...", i + 1);
        let link = entry.url.clone();

        let naive_date = chrono::DateTime::parse_from_rfc3339(&entry.date_modified)?;
        let title = format!("Entry {}", naive_date.timestamp());

        let date = chrono::Utc
            .ymd(naive_date.year(), naive_date.month(), naive_date.day())
            .and_hms(naive_date.hour(), naive_date.minute(), naive_date.second())
            .to_rfc2822();

        let content = format!("{}<p>{}</p>", entry.content_html, entry.title);

        let item = rss::ItemBuilder::default()
            .title(title)
            .link(link)
            .content(content)
            .pub_date(date)
            .build()?;

        items.push(item);
        if items.len() >= num_entries as usize {
            break;
        }
    }

    Ok((
        format!("Instagram - {}", account_name),
        format!("RSS feed for Instagram user account {}.", account_name),
        format!("https://instagram.com/{}", account_name),
        items,
    ))
}
