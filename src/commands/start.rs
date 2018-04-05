use config;
use failure::Error;
use jira;
use prettytable::Table;
use serde_json as json;
use std;
use git2;

pub(crate) fn command(ticket: &Option<String>, config: &config::Config) -> Result<(), Error> {
    match *ticket {
        Some(ref key) => start_ticket(key, config),
        None => show_open_issues(config),
    }
}

/// Start working on a ticket
fn start_ticket(ticket: &str, config: &config::Config) -> Result<(), Error> {
    let mut response = jira::get_issue(
        ticket,
        &config.jira.host,
        &config.jira.email,
        &config.jira.token,
    )?;

    let data = response.json::<json::Value>()?;
    let new_branch_name = derive_branch_name(ticket, &data);
    info!("creating {}", new_branch_name);

    Ok(())
}

/// Derive the branch name from the ticket.
///
/// If a Code Propagation subtask is found associated to the ticket, that subtask's key will be
/// appended to the branch name. Otherwise, the branch name will just have the ticket's key.
fn derive_branch_name(ticket: &str, data: &json::Value) -> String {
    if let Some(subtasks) = data["fields"]["subtasks"].as_array() {
        for subtask in subtasks {
            if let Some(issuetype) = subtask["fields"]["issuetype"]["id"].as_str() {
                if issuetype == jira::PROPAGATION_SUBTASK_ISSUETYPE {
                    if let Some(subtask_key) = subtask["key"].as_str() {
                        return format!("master_{}_{}", ticket, subtask_key);
                    }
                }
            }
        }
    }
    format!("master_{}", ticket)
}

fn show_open_issues(config: &config::Config) -> Result<(), Error> {
    let jql = jira::OPEN_ISSUES_JQL;
    let mut response = jira::search_issues(
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
