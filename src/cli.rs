use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A tool for managing and retrieving gitlab time estimations")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<Command>
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Config {
        #[structopt(short, long)]
        global: bool,
        #[structopt(subcommand)]
        mode: ConfigMode
    },
}

#[derive(Debug, StructOpt)]
pub enum ConfigMode {
    AddAPIKey {
        #[structopt(short, long)]
        api_url: String,

        #[structopt()]
        api_key: String
    },
    Get {},
}

pub fn get_opts() -> Opt {
    return Opt::from_args();
}

pub fn prompt(msg: &str, default: Option<&str>) -> String {
    let mut input = String::new();

    match default {
        Some(value) => print!("{} (default: {}): ", msg, value),
        None        => print!("{}: ", msg),
    }

    let _ = stdout().flush();
    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    if input.trim().is_empty() {
        return default.unwrap_or("").to_string();
    }

    return input.trim().to_string();
}

