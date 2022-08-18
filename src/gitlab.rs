use std::io;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use gitlab::Gitlab;
use gitlab::api::{Pagination, paged, projects, issues, users, Query};

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u64,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    pub id: u64,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub time_stats: TimeStats, 
    #[serde(skip_deserializing)]
    pub time_estimate: u64,
    #[serde(skip_deserializing)]
    pub time_spent: u64,
    pub web_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeStats {
    pub time_estimate: u64,
    pub total_time_spent: u64,
}

pub fn get_client(cfg: crate::config::MyConfig) -> Result<Gitlab, &'static str> {
    // let client = Gitlab::new(cfg.gitlab_url, cfg.api_key).unwrap();

    return Err("to be implemented");
}

pub fn get_current_user(client: &Gitlab) -> Result<User, &'static str> {
    let user = users::CurrentUser::builder()
        .build().unwrap()
        .query(client).unwrap();

    return Ok(user);
}

pub fn get_timetracking_for(client: &Gitlab, user: &User) -> Result<Vec<Issue>, &'static str> {
    let projects = projects::Projects::builder().build().unwrap(); 

    let mut issues: Vec<Issue> = vec![];

    paged(projects, Pagination::All).iter::<Gitlab, Project>(client).into_iter().for_each(|project| {
        let project_issues = issues::ProjectIssues::builder()
            .project(project.unwrap().id)
            .assignee_id(user.id)
            .build().unwrap();

        paged(project_issues, Pagination::All).iter::<Gitlab, Issue>(client).into_iter().for_each(|wissue| {
            let mut issue = wissue.unwrap();

            issue.time_estimate = issue.time_stats.time_estimate;
            issue.time_spent = issue.time_stats.total_time_spent;

            issues.push(issue);
        });
    });
    
    return Ok(issues);
}

pub fn write_issues_as_csv(issues: Vec<Issue>) -> Result<(), &'static str> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    for issue in &issues {
        wtr.serialize(issue).unwrap();
    }

    wtr.flush().unwrap();

    return Ok(());
}
