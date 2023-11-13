use grammers_client::{Client, Config};
use grammers_session::Session;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
mod lib;

#[derive(Serialize, Deserialize, Clone)]
struct BotConfg {
    api_id: i32,
    api_hash: String,
    bot_token: String,
}

impl BotConfg {
    fn load() -> Option<BotConfg> {
        let files = File::open("./config.yml").expect("Unable to open config.yml");
        serde_yaml::from_reader(files).expect("unable to parse config.yml")
    }
}

struct Watcher {
    khh: String,
    tpe: String,
}

impl Watcher {
    async fn fetch() -> Option<lib::Root> {
        let url = "https://www.cloudflarestatus.com/api/v2/summary.json";
        match reqwest::get(url)
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
    println!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create("bot.session").unwrap(),
        api_id: botconf.api_id,
        api_hash: botconf.api_hash,
        params: Default::default(),
    })
    .await
    .unwrap();
    println!("Connected!");

    // init fetch
}
