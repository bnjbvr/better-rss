use chrono;
use chrono::prelude::*;
use json;
use reqwest;
use rss;

use utils::{Channel, GenResult};

pub fn make(num_entries: u32) -> GenResult<Channel> {
    let mut items: Vec<rss::Item> = Vec::new();
    let mut url = String::from("https://xkcd.com/info.0.json");

    eprintln!("Starting to fetch XKCD...");

    for i in 1..(num_entries + 1) {
        eprintln!("fetching item #{}...", i);
        let text = reqwest::get(url.as_str())?.text()?;
        let response = json::parse(&text)?;

        let title = String::from(response["safe_title"].as_str().ok_or("missing title")?);
        let xkcd_id = response["num"].as_u32().ok_or("missing id")?;
        let link = format!("https://xkcd.com/{}", xkcd_id);

        let src = response["img"].as_str().ok_or("missing src")?;
        let hover = response["alt"].as_str().ok_or("missing alt")?;
        let content = format!("<img src='{}' /><p>{}</p>", src, hover);

        let year = response["year"]
            .as_str()
            .ok_or("missing year in date")?
            .parse::<i32>()?;
        let month = response["month"]
            .as_str()
            .ok_or("missing month in date")?
            .parse::<u32>()?;
        let day = response["day"]
            .as_str()
            .ok_or("missing day in date")?
            .parse::<u32>()?;
        let date = chrono::Utc
            .ymd(year, month, day)
            .and_hms(0, 0, 0)
            .to_rfc2822();

        let item = rss::ItemBuilder::default()
            .title(title)
            .link(link)
            .content(content)
            .pub_date(date)
            .build()?;

        items.push(item);

        url = format!("https://xkcd.com/{}/info.0.json", xkcd_id - 1);
    }

    Ok((
        "XKCD feed".into(),
        "RSS feed for xkcd that includes hover links in plain text.".into(),
        "http://xkcd.com".into(),
        items,
    ))
}
