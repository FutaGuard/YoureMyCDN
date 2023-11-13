use chrono::prelude::*;
use grammers_client::{Client, Config};
use grammers_session::Session;
use grammers_client::InputMessage;

use grammers_tl_types as tl;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::ops::BitAnd;
use std::time::Duration;

mod lib;

#[derive(Serialize, Deserialize, Clone)]
struct BotConfg {
    api_id: i32,
    api_hash: String,
    bot_token: String,
    notify: Vec<i64>,
}

impl BotConfg {
    fn load() -> Option<BotConfg> {
        let files = File::open("./config.yml").expect("Unable to open config.yml");
        serde_yaml::from_reader(files).expect("unable to parse config.yml")
    }
}

#[derive(Default)]
struct Watcher {
    url: String,
    khh: lib::Component,
    tpe: lib::Component,
}

impl Watcher {
    async fn new(mut self) {
        let result = reqwest::get(self.url)
            .await
            .expect("fetch data error")
            .json::<lib::Root>()
            .await
            .expect("parse data error");
        self.khh = result
            .components
            .iter()
            .find(|&x| x.name.starts_with("Kaohsiung"))
            .unwrap()
            .clone();
        self.tpe = result
            .components
            .iter()
            .find(|&x| x.name.starts_with("Taipei"))
            .unwrap()
            .clone();
    }
    async fn fetch(self) -> Option<lib::Root> {
        match reqwest::get(self.url)
            .await
            .expect("fetch data error")
            .json::<lib::Root>()
            .await
        {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }
}

#[tokio::main]
async fn main() {
    let botconf = BotConfg::load().unwrap();
    let mut chatlist = Vec::new();
    let mut notify_chats = Vec::new();
    println!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create("bot.session").unwrap(),
        api_id: botconf.api_id,
        api_hash: botconf.api_hash,
        params: Default::default(),
    })
    .await
    .unwrap();
    if !client.is_authorized().await.unwrap() {
        panic!("Bot Token Error or API hash Error")
    }
    let mut dialogs = client.iter_dialogs();
    while let Some(dialog) = dialogs.next().await? { chatlist.push(dialog.chat().clone()) };
    for notify_id in botconf.notify {
        match chatlist.iter().find(|&x| x.id() == notify_id).unwrap() {
            Ok(c) => { notify_chats.push(c) }
        }
    }

    println!("Connected!");

    // init fetch
    // let watcher = Watcher::new();
    let watcher = Watcher {
        url: "https://www.cloudflarestatus.com/api/v2/summary.json".to_string(),
        khh: Default::default(),
        tpe: Default::default(),
    };
    watcher.new().await;
    loop {
        tokio::time::sleep(Duration::from_millis(60000)).await;
        let result = watcher.fetch().await.unwrap();
        let tpe = result
            .components
            .iter()
            .find(|&&x| x.name.starts_with("Taipei"))
            .unwrap();
        let khh = result
            .components
            .iter()
            .find(|&&x| x.name.starts_with("Kaohsiung"))
            .unwrap();
        if tpe != &watcher.tpe {
            let text = format!("🔔 Cloudflare TPE 狀態變更\n{}", tpe.status);
            for group in botconf.notify {

                client
                    .send_message(group, InputMessage::text(&text))
                    .await
                    .except(format!("when send msg to {} an error occur", group));
            }
        }
        if khh != &watcher.khh {
            let text = format!("🔔 Cloudflare KHH 狀態變更\n{}", khh.status);
            for group in botconf.notify {
                client
                    .send_message(group, InputMessage::text(&text))
                    .await
                    .except(format!("when send msg to {} an error occur", group));
            }
        }
    }
}
