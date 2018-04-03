use failure::Error;
use reqwest;

const OPEN_TICKETS_IN_SPRINT_JQL: &'static str = r#"
assignee in ({}) and Sprint in openSprints() and type not in subTaskIssueTypes()
"#;

pub(crate) fn serach_issues(jql: &str, host: &str, username: &str, password: &str) -> Result<reqwest::Response, Error> {
    let base_url = reqwest::Url::parse(host)?;
    let request_url = base_url.join("/rest/api/2/search")?;
    let client = reqwest::Client::new();
    let response = client.get(request_url)
        .basic_auth(username, Some(password));
    Ok(response)
}
