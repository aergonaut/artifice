use failure::Error;
use reqwest;
use reqwest::header::{qitem, Accept};
use reqwest::mime;

pub const OPEN_ISSUES_JQL: &'static str = r#"
assignee = currentUser() and Sprint in openSprints() and type not in subTaskIssueTypes()
"#;

pub(crate) fn serach_issues(
    jql: &str,
    host: &str,
    username: &str,
    password: &str,
) -> Result<reqwest::Response, Error> {
    let base_url = reqwest::Url::parse(host)?;
    let request_url = base_url.join("/rest/api/2/search")?;
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .basic_auth(username, Some(password))
        .header(Accept(vec![qitem(mime::APPLICATION_JSON)]))
        .query(&[("jql", jql), ("fields", "key,summary")])
        .send()?;
    Ok(response)
}
