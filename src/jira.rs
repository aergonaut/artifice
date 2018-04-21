use failure::Error;
use reqwest;
use reqwest::header::{qitem, Accept};
use reqwest::mime;
use config;

pub const OPEN_ISSUES_JQL: &'static str = r#"
assignee = currentUser() and Sprint in openSprints() and type not in subTaskIssueTypes()
"#;

pub const PROPAGATION_SUBTASK_ISSUETYPE: &'static str = "10600";

pub(crate) struct Jira<'a> {
    config: &'a config::JiraConfig,
}

impl<'a> Jira<'a> {
    pub(crate) fn new(config: &'a config::JiraConfig) -> Jira<'a> {
        Jira { config: config }
    }

    pub(crate) fn search_issues(&self, jql: &str) -> Result<reqwest::Response, Error> {
        let base_url = reqwest::Url::parse(&self.config.host)?;
        let request_url = base_url.join("/rest/api/2/search")?;
        let client = reqwest::Client::new();
        let response = client
            .get(request_url)
            .basic_auth(self.config.email.clone(), Some(self.config.token.clone()))
            .header(Accept(vec![qitem(mime::APPLICATION_JSON)]))
            .query(&[("jql", jql), ("fields", "key,summary")])
            .send()?;
        Ok(response)
    }

    pub(crate) fn get_issue(&self, key: &str) -> Result<reqwest::Response, Error> {
        let base_url = reqwest::Url::parse(&self.config.host)?;
        let request_url = base_url.join(&format!("/rest/api/2/issue/{}", key))?;
        let client = reqwest::Client::new();
        let response = client
            .get(request_url)
            .basic_auth(self.config.email.clone(), Some(self.config.token.clone()))
            .header(Accept(vec![qitem(mime::APPLICATION_JSON)]))
            .send()?;
        Ok(response)
    }
}
