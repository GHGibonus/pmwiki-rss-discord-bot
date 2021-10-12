use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
struct SentItem {
    date: String,
}
impl SentItem {
    fn new(date: String) -> Self {
        SentItem { date }
    }
}

#[derive(Deserialize)]
struct Config {
    rss_url: String,
    hook_url: String,
    pass: String,
}

#[derive(Serialize)]
struct DiscordMsg {
    content: String,
}

impl DiscordMsg {
    fn from_item(item: Item) -> (Self, String) {
        let contributor = item.dublin_core_ext().unwrap().contributors.first().unwrap();
        let date = item.pub_date().expect("pm wiki should set pubDate field").to_string();
        let link = item.link().expect("pm wiki should set link field");
        let content = format!("**{}**\nby: {}\n{}", date, contributor, link);
        (DiscordMsg { content }, date)
    }
}

fn discord_msgs(channel: Channel, sent: &[SentItem]) -> Vec<(DiscordMsg, String)> {
    channel
        .items
        .into_iter()
        .filter(|it| {
            let date = it.pub_date().unwrap().to_string();
            !sent.contains(&SentItem { date })
        })
        .map(DiscordMsg::from_item)
        .collect()
}

fn main() -> Result<()> {
    let config = File::open("config.json")?;
    let config: Config = serde_json::from_reader(BufReader::new(config))?;

    let sent_items = File::open("sent.json")?;
    let mut sent_items: Vec<SentItem> = serde_json::from_reader(BufReader::new(sent_items))?;

    let agent = ureq::agent();
    let discord_agent = ureq::agent();

    let rss = agent.post(&config.rss_url).send_form(&[("authpw", &config.pass)])?;
    let reader = BufReader::new(rss.into_reader());
    let channel = Channel::read_from(reader)?;
    let msgs_to_send = discord_msgs(channel, &sent_items);
    for (msg, date) in msgs_to_send.into_iter() {
        discord_agent
            .post(&config.hook_url)
            .set("Content-Type", "application/json")
            .send_bytes(&serde_json::to_vec(&msg)?)?;
        sent_items.push(SentItem::new(date));
    }
    let sent_items_file = File::create("sent.json")?;
    serde_json::to_writer(sent_items_file, &sent_items)?;
    Ok(())
}
