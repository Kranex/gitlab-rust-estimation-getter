mod config;
mod cli;
mod gitlab;

fn main() -> Result<(), &'static str> {
    let cfg = config::load().unwrap();
    let client = gitlab::get_client(cfg).unwrap();
    let currentuser = gitlab::get_current_user(&client).unwrap();

    eprintln!("Current User: {}", currentuser.name);

    let  mut issues = gitlab::get_timetracking_for(&client, &currentuser).unwrap();
    issues.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    gitlab::write_issues_as_csv(issues).unwrap();

    return Ok(());
}
