use crate::cli;
use serde::{Deserialize, Serialize};
use confy;

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub version: u8,
    pub gitlab_url: String,
    pub api_key: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: 0, gitlab_url: "".into(), api_key: "".into() } }
}

pub fn load() -> Result<MyConfig, &'static str> {
    let mut cfg: MyConfig = confy::load("gitlab-rust-estimation-getter").unwrap();

    if cfg.gitlab_url.is_empty() {
        println!("gitlab url is not configured.");
        cfg.gitlab_url = cli::prompt("enter url", Some("gitlab.com"));
    }

    if cfg.api_key.is_empty() {
        println!("api key is not configured.");
        cfg.api_key = cli::prompt("enter gitlab api key", None);
    }

    if cfg.gitlab_url.is_empty() || cfg.api_key.is_empty() {
        return Err("gitlab url and api key must be provided.");
    }

    confy::store("gitlab-rust-estimation-getter", &cfg).unwrap();

    return Ok(cfg);
}
