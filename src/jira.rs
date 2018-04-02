const OPEN_TICKETS_IN_SPRINT_JQL = r#"
assignee in ({}) and sprint in currentOpenSprints()
"#
