use git_config::File;
use git_discover::repository::Path;
use serde::{Deserialize, Serialize};
use confy;

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    version: u8,
    pub api_urls: Vec<APIUrl>,
}

#[derive(Serialize, Deserialize)]
pub struct APIUrl {
    pub api_url: String,
    pub api_keys: Vec<String>,
}


impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: 0, api_urls: vec![] } }
}

pub fn load() -> Result<MyConfig, confy::ConfyError> {
    let mut cfg: MyConfig = confy::load("greg")?;

    let (path,  _) = git_discover::upwards(".").unwrap();
    let (repo, _) = path.into_repository_and_work_tree_directories();

    
    println!("{}", repo.into_os_string().into_string().unwrap());

    // let mut local_cfg: MyConfig = confy::load_path()?;


    return Ok(cfg);
}

pub fn get_api_url(cfg: MyConfig) -> Option<String> {
    let (repo, _) = git_discover::upwards(".").unwrap();
    let (dir, _) = repo.into_repository_and_work_tree_directories();
    let config: File = git_config::File::from_git_dir(dir).unwrap();
    let url = config.string("remote", Some("origin"), "url").unwrap();

    return Some(url.to_string());
}
