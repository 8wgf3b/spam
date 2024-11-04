use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Weekday;
use rand::distributions::Alphanumeric;
use rand::thread_rng as trng;
use rand::Rng;
use reqwest::Client;
use serde_json::{json, Value};
use tracing::error;
use tracing::{debug, info, instrument};

mod config;

pub use config::Cmap;

#[derive(Debug)]
pub struct Daybreak {
    pub cmap: Cmap,
    date: NaiveDate,
    boolar: Vec<bool>,
    token: String,
}

impl Daybreak {
    pub fn new(rd: &str, rmsg: &str, rcfg: &str, token: &str) -> Self {
        let mut d = NaiveDate::parse_from_str(rd, "%Y.%m.%d").expect("Wrong date format bro!");
        d = Self::next_sunday(d);
        info!("Start date: {d}");
        debug!("Finishing creating a Daybreak object");
        let c = Cmap::build(rcfg);
        let msg = rmsg.chars().fold(String::new(), |mut a, c| {
            a.push(c);
            a.push(' ');
            a
        });
        Daybreak {
            date: d,
            boolar: c.boolgen(&msg),
            cmap: c,
            token: token.to_owned(),
        }
    }

    fn next_sunday(d: NaiveDate) -> NaiveDate {
        d.iter_days()
            .filter(|x| x.weekday() == Weekday::Sun)
            .next()
            .unwrap()
    }

    #[instrument(skip(self))]
    pub fn checkdate(&self, d: &str) -> bool {
        if let Ok(d) = NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            let diff = (self.date - d).num_days();
            debug!("{diff} since start");
            return diff >= 0 && self.boolar[diff as usize % self.boolar.len()];
        }
        false
    }

    pub fn simulate(&self) {
        let n = self.boolar.len() / 7;
        let mut canvas = vec![String::with_capacity(n); 7];
        for i in 0..7 {
            for j in 0..n {
                canvas[i].push(match self.boolar[j * 7 + i] {
                    true => '*',
                    _ => ' ',
                })
            }
        }
        for s in canvas {
            println!("{s}")
        }
    }

    pub async fn burnice(&self) {
        let owner = "8wgf3b";
        let repo = "spam";
        let file = "burnice.txt";
        let mut content = trng()
            .sample_iter(Alphanumeric)
            .take(36)
            .map(char::from)
            .collect();
        content = STANDARD.encode(content);
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            owner, repo, file
        );
        debug!("Finished creating git vars");
        let clt = Client::new();
        let resp = clt
            .get(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "request")
            .send()
            .await;
        let r: Value = resp
            .expect("Error burnice get")
            .json()
            .await
            .expect("Failed json parsing");
        let shav = r.get("sha").expect("No sha");
        info!("burnice SHA: {shav}");
        let payload = json!({
            "message": "NLB",
            "content": content,
            "sha": shav
        });
        info!("content: '{content}'");
        let resp = clt
            .put(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "request")
            .json(&payload)
            .send();
        match resp.await {
            Ok(r) if r.status().is_success() => info!("Preem! Lets go!"),
            _ => error!("Sry choom. Request zeroed"),
        }
    }
}
