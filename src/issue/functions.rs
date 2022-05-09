use std::collections::HashMap;

use serde_json::Value;
use ureq::{json, Response};

use crate::lib::{get_request, post_request, put_request};

pub fn add_label(global: &HashMap<&str, &str>, issue_key: &str, label: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/issue/{}",
        global["domain"], issue_key
    );
    let payload: Value = json!({
        "update": {
            "labels": [
                {
                    "add": label
                }
            ]
        }
    });
    let success_message: String = format!("Label {} added to issue {}", label, issue_key);
    put_request(
        &url,
        payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn add_version(global: &HashMap<&str, &str>, version_name: &str, issue_key: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/issue/{}",
        global["domain"], issue_key
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
    let success_message: String = format!("Version {} added to issue {}", version_name, issue_key);
    put_request(
        &url,
        payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn create(
    global: &HashMap<&str, &str>,
    reporter_account_id: &str,
    project_key: &str,
    issue_type: &str,
    summary: &str,
    description: &str,
    priority: &str,
) {
    let url: String = format!("https://{}/rest/api/3/issue", global["domain"]);
    let mut payload: Value = json!({
        "fields":
        {
            "project": { "key": project_key },
            "summary": summary,
            "description": { "type": "doc", "version": 1, "content": [{
                "type": "paragraph", "content": [{
                    "type": "text", "text": description}]
            }]},
            "issuetype": { "name": issue_type },
            "reporter": { "id": reporter_account_id},
        }
    });

    if !priority.is_empty() {
        payload["fields"]["priority"] = json!({ "name": priority });
    }

    let success_message = "";
    let resp = post_request(
        &url,
        payload,
        global["user"],
        global["token"],
        success_message,
        true,
    )
    .right()
    .unwrap();
    let json: Value = resp.into_json().unwrap();
    println!("Issue created: {}", json["key"]);
}

pub fn list_priorities(global: &HashMap<&str, &str>) {
    let url: String = format!("https://{}/rest/api/3/priority", global["domain"]);
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    json.as_array().unwrap().iter().for_each(|x| {
        println!("{}", x["name"]);
    });
}

pub fn list_types(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/issue/createmeta?projectKeys={}",
        global["domain"], project_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    json["projects"][0]["issuetypes"]
        .as_array()
        .unwrap()
        .iter()
        .for_each(|x| {
            println!("{}", x["name"]);
        });
}

pub fn remove_label(global: &HashMap<&str, &str>, issue_key: &str, label: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/issue/{}",
        global["domain"], issue_key
    );
    let payload: Value = json!({
        "update": {
            "labels": [
                {
                    "remove": label
                }
            ]
        }
    });
    let success_message: String = format!("Label {} removed from issue {}", label, issue_key);
    put_request(
        &url,
        payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn remove_version(global: &HashMap<&str, &str>, version_name: &str, issue_key: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/issue/{}",
        global["domain"], issue_key
    );
    let payload: Value = json!({
        "update": {
            "fixVersions": [
                {
                    "remove": {
                        "name": version_name
                    }
                }
            ]
        }
    });
    let success_message: String =
        format!("Version {} removed from issue {}", version_name, issue_key);
    put_request(
        &url,
        payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn show_fixversions(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/issue/{}",
        global["domain"], issue_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    json["fields"]["fixVersions"]
        .as_array()
        .unwrap()
        .iter()
        .for_each(|x| {
            println!("{}", x["name"]);
        });
}
