use config;
use failure::Error;
use jira;
use prettytable;
use prettytable::Table;
use serde_json as json;

pub(crate) fn command(ticket: &Option<String>, config: &config::Config) -> Result<(), Error> {
    match *ticket {
        Some(_) => Ok(()),
        None => show_open_issues(config),
    }
}

fn show_open_issues(config: &config::Config) -> Result<(), Error> {
    let jql = jira::OPEN_ISSUES_JQL;
    let mut response = jira::serach_issues(
        &jql,
        &config.jira.host,
        &config.jira.email,
        &config.jira.token,
    )?;

    let mut table = Table::new();
    table.add_row(row!["Key", "Summary"]);
    let data = response.json::<json::Value>()?;
    if let Some(issues) = data["issues"].as_array() {
        for issue in issues {
            if let Some(key) = issue["key"].as_str() {
                if let Some(summary) = issue["fields"]["summary"].as_str() {
                    table.add_row(row![key, summary]);
                }
            }
        }
    }
    table.printstd();

    Ok(())
}
