use crate::lib::{get_request, post_request, put_request};
use serde_json::Value;
use ureq::{json, Response};

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

pub fn create_issue(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    reporter_account_id: &str,
    project_key: &str,
    issue_type: &str,
    issue_summary: &str,
    issue_description: &str,
    issue_priority: &str,
) {
    let url: String = format!("https://{domain}/rest/api/3/issue", domain = jira_domain);
    let mut payload: Value = json!({
        "fields":
        {
            "project": { "key": project_key },
            "summary": issue_summary,
            "description": { "type": "doc", "version": 1, "content": [{
                "type": "paragraph", "content": [{
                    "type": "text", "text": issue_description}]
            }]},
            "issuetype": { "name": issue_type },
            "reporter": { "id": reporter_account_id},
        }
    });

    if !issue_priority.is_empty() {
        payload["fields"]["priority"]["name"] = issue_priority.parse().unwrap();
    }

    let success_message = "";
    let resp = post_request(&url, payload, jira_user, jira_token, success_message, true)
        .right()
        .unwrap();
    let json: Value = resp.into_json().unwrap();
    println!("Issue created: {}", json["key"]);
}

pub fn list_issue_priorities(jira_domain: &str, jira_user: &str, jira_token: &str) {
    let url: String = format!("https://{domain}/rest/api/3/priority", domain = jira_domain);
    let resp: Response = get_request(&url, jira_user, jira_token);
    let json: Value = resp.into_json().unwrap();
    json.as_array().unwrap().iter().for_each(|x| {
        println!("{}", x["name"]);
    });
}

pub fn list_issue_types(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_key: &str,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/issue/createmeta?projectKeys={project_key}",
        domain = jira_domain,
        project_key = jira_project_key
    );
    let resp: Response = get_request(&url, jira_user, jira_token);
    let json: Value = resp.into_json().unwrap();
    json["projects"][0]["issuetypes"]
        .as_array()
        .unwrap()
        .iter()
        .for_each(|x| {
            println!("{}", x["name"]);
        });
}
