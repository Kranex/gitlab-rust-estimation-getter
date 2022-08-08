use serde::{Deserialize, Serialize};
use std::io::{self, stdout, Write};
use confy;
use std::io::stdin;
use chrono::{DateTime, Utc};
use gitlab::Gitlab;
use gitlab::api::{Pagination, paged, projects, issues, users, Query};

#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    gitlab_url: String,
    api_key: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: 0, gitlab_url: "".into(), api_key: "".into() } }
}

#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Issue {
    id: u64,
    title: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    time_stats: TimeStats, 
    #[serde(skip_deserializing)]
    time_estimate: u64,
    #[serde(skip_deserializing)]
    time_spent: u64,
    web_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeStats {
    time_estimate: u64,
    total_time_spent: u64,
}

fn prompt(msg: &str, default: Option<&str>) -> String {
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

fn main() -> Result<(), &'static str> {
    let mut cfg: MyConfig = confy::load("gitlab-rust-estimation-getter").unwrap();

    if cfg.gitlab_url.is_empty() {
        println!("gitlab url is not configured.");
        cfg.gitlab_url = prompt("enter url", Some("gitlab.com"));
    }

    if cfg.api_key.is_empty() {
        println!("api key is not configured.");
        cfg.api_key = prompt("enter gitlab api key", None);
    }

    if cfg.gitlab_url.is_empty() || cfg.api_key.is_empty() {
        return Err("gitlab url and api key must be provided.");
    }

    confy::store("gitlab-rust-estimation-getter", &cfg).unwrap();

    let client = Gitlab::new(cfg.gitlab_url, cfg.api_key).unwrap();

    let currentuser: User = users::CurrentUser::builder()
        .build().unwrap()
        .query(&client).unwrap();

    eprintln!("Current User: {}", currentuser.name);

    let projects = projects::Projects::builder().build().unwrap(); 

    let mut issues: Vec<Issue> = vec![];

    paged(projects, Pagination::All).iter::<Gitlab, Project>(&client).into_iter().for_each(|project| {
        let project_issues = issues::ProjectIssues::builder()
            .project(project.unwrap().id)
            .assignee_id(currentuser.id)
            .build().unwrap();

        paged(project_issues, Pagination::All).iter::<Gitlab, Issue>(&client).into_iter().for_each(|wissue| {
            let mut issue = wissue.unwrap();

            issue.time_estimate = issue.time_stats.time_estimate;
            issue.time_spent = issue.time_stats.total_time_spent;

            issues.push(issue);
        });
    });

    issues.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for issue in &issues {
        wtr.serialize(issue).unwrap();
    }

    wtr.flush().unwrap();

    Ok(())
}
