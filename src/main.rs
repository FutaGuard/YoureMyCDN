use chrono::prelude::*;
use teloxide::prelude::*;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::ops::BitAnd;
use std::sync::Arc;
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

#[derive(Default, Clone)]
struct Watcher {
    url: String,
    khh: lib::Component,
    tpe: lib::Component,
}

impl Watcher {
    async fn new(&mut self) {
        let result = reqwest::get(&self.url)
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
    async fn fetch(&self) -> Option<lib::Root> {
        match reqwest::get(&self.url)
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
    let botconf = Arc::new(BotConfg::load().unwrap());

    println!("Connecting to Telegram...");
    let client = Bot::new(&botconf.bot_token);
    match client.get_me().await {
        Ok(me) => { println!("Running, {:?}", me.username.as_ref()) }
        Err(e) => {panic!("{}", e)}
    }
    println!("Connected!");

    // init fetch
    let mut watcher = Watcher {
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
            .find(|&x| x.name.starts_with("Taipei"))
            .unwrap();
        let khh = result
            .components
            .iter()
            .find(|&x| x.name.starts_with("Kaohsiung"))
            .unwrap();

        if tpe != &watcher.tpe {
            let text = format!("🔔 Cloudflare TPE 狀態變更\n{}", tpe.status);
            for group in &botconf.notify {
                client
                    .send_message(ChatId(group.clone()), &text)
                    .await
                    .expect("when send msg to an error occur");
            }
        }

        if khh != &watcher.khh {
            let text = format!("🔔 Cloudflare KHH 狀態變更\n{}", khh.status);
            for group in &botconf.notify {
                client
                    .send_message(ChatId(group.clone()), &text)
                    .await
                    .expect("when send msg to an error occur");
            }
        }
    }
}
