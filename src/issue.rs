use crate::lib::put_request;
use clap::{Arg, Command};
use serde_json::Value;
use ureq::json;

pub fn add_version(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    version_name: &str,
    jira_issue: &str,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/issue/{issue_key}",
        domain = jira_domain,
        issue_key = jira_issue
    );
    let payload: Value = json!({
        "update": {
            "fixVersions": [
                {
                    "add": {
                        "name": version_name
                    }
                }
            ]
        }
    });
    let success_message: String = format!("Version {} added to issue {}", version_name, jira_issue);
    put_request(&url, payload, jira_user, jira_token, &success_message);
}

pub fn cli_add_version() -> Command<'static> {
    Command::new("add_version")
        .about("Add version to Jira issue")
        .arg_required_else_help(true)
        .arg(
            Arg::new("issue_key")
                .help("Issue key")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("version_name")
                .help("Version name")
                .env("JIRA_VERSION_NAME")
                .takes_value(true)
                .required(true),
        )
}
