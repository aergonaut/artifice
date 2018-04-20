use config;
use failure::Error;
use failure::err_msg;
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

/// Start working on a ticket.
///
/// This first fetches the ticket's information from JIRA and then tries to derive the branch name
/// from the ticket and subtask keys. It tries to create the branch if it does not exist and then
/// tries to switch the current repository to the branch.
fn start_ticket(ticket: &str, config: &config::Config) -> Result<(), Error> {
    let mut response = jira::get_issue(
        ticket,
        &config.jira.host,
        &config.jira.email,
        &config.jira.token,
    )?;

    let data = response.json::<json::Value>()?;
    let new_branch_name = derive_branch_name(ticket, &data);
    let repo = git2::Repository::discover(std::env::current_dir()?)?;
    let new_branch = create_branch(&repo, &new_branch_name)?;
    let _ = checkout_branch(&repo, &new_branch)?;
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

/// Print the user's current open JIRA issues as a table to STDOUT
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

/// Create a new Git branch with the given `branch_name`
///
/// Returns the `Branch` object.
fn create_branch<'repo>(
    repo: &'repo git2::Repository,
    branch_name: &str,
) -> Result<git2::Branch<'repo>, Error> {
    let head_ref = repo.head()?;
    let head_commit = head_ref.peel_to_commit()?;
    info!("creating {}", branch_name);
    let new_branch = repo.branch(branch_name, &head_commit, false)?;
    Ok(new_branch)
}

/// Checkout the given branch
fn checkout_branch(repo: &git2::Repository, branch: &git2::Branch) -> Result<(), Error> {
    if let Some(branch_name) = branch.name()? {
        let ref_name = format!("refs/heads/{}", branch_name);
        let reference = branch.get();
        let treeish = reference.peel_to_tree()?;
        info!("checking out {}", branch_name);
        repo.checkout_tree(treeish.as_object(), None)?;
        return Ok(repo.set_head(&ref_name)?);
    }
    Err(err_msg("Branch has no name"))
}
