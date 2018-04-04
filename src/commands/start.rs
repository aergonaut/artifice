use failure::Error;
use serde_json as json;
use config;
use jira;

pub(crate) fn command(ticket: &Option<String>, config: &config::Config) -> Result<(), Error> {
    let jql = jira::OPEN_ISSUES_JQL;
    let mut response = jira::serach_issues(
        &jql,
        &config.jira.host,
        &config.jira.email,
        &config.jira.token,
    )?;
    info!("{:?}", response.text());
    Ok(())
}
