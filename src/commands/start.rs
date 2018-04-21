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
    let jira_client = jira::Jira::new(&config.jira);
    let mut response = jira_client.get_issue(ticket)?;

    let data = response.json::<json::Value>()?;
    let repo = git2::Repository::discover(std::env::current_dir()?)?;
    let new_branch_name = derive_branch_name(&repo, ticket, &data)?;
    let new_branch = create_branch(&repo, &new_branch_name)?;
    let _ = checkout_branch(&repo, &new_branch)?;
    Ok(())
}

/// Derive the branch name from the ticket.
///
/// If a Code Propagation subtask is found associated to the ticket, that subtask's key will be
/// appended to the branch name. Otherwise, the branch name will just have the ticket's key.
fn derive_branch_name<'repo>(
    repo: &'repo git2::Repository,
    ticket: &str,
    data: &json::Value,
) -> Result<String, Error> {
    let current_branch_name = get_current_branch_name(repo)?;
    if let Some(subtasks) = data["fields"]["subtasks"].as_array() {
        for subtask in subtasks {
            if let Some(issuetype) = subtask["fields"]["issuetype"]["id"].as_str() {
                if issuetype == jira::PROPAGATION_SUBTASK_ISSUETYPE {
                    if let Some(subtask_key) = subtask["key"].as_str() {
                        return Ok(format!(
                            "{}_{}_{}",
                            current_branch_name, ticket, subtask_key
                        ));
                    }
                }
            }
        }
    }
    Ok(format!("{}_{}", current_branch_name, ticket))
}

fn get_current_branch_name<'repo>(repo: &'repo git2::Repository) -> Result<String, Error> {
    let head_ref = repo.head()?;
    if head_ref.is_branch() {
        head_ref
            .shorthand()
            .ok_or_else(|| format_err!("Branch name is not UTF-8"))
            .map(|s| s.to_owned())
    } else {
        return Err(format_err!("HEAD is not a branch"));
    }
}

/// Print the user's current open JIRA issues as a table to STDOUT
fn show_open_issues(config: &config::Config) -> Result<(), Error> {
    let jira_client = jira::Jira::new(&config.jira);
    let jql = jira::OPEN_ISSUES_JQL;
    let mut response = jira_client.search_issues(&jql)?;

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
    let new_branch = repo.find_branch(branch_name, git2::BranchType::Local)
        .or_else(|_| {
            info!("creating branch {}", branch_name);
            repo.branch(branch_name, &head_commit, false)
        })?;
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
