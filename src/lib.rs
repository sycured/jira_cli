use base64::encode as b64encode;
use serde_json::Value;
use std::process::exit;
use ureq::{delete, get, post, put, Response};

fn b64auth(jira_user: &str, jira_token: &str) -> String {
    return b64encode(format!(
        "{user}:{token}",
        user = jira_user,
        token = jira_token
    ));
}

pub fn delete_request(url: &str, jira_user: &str, jira_token: &str, success_message: &str) {
    let resp = delete(url)
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(jira_user, jira_token)),
        )
        .call();
    match resp {
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
        Ok(_) => {
            println!("{}", success_message);
        }
    }
}

pub fn get_request(url: &str, jira_user: &str, jira_token: &str) -> Response {
    let resp = get(url)
        .set("Accept", "application/json")
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(jira_user, jira_token)),
        )
        .call();
    match resp {
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
        Ok(response) => response,
    }
}

pub fn post_request(
    url: &str,
    payload: Value,
    jira_user: &str,
    jira_token: &str,
    success_message: &str,
) {
    let resp = post(url)
        .set("Accept", "application/json")
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(jira_user, jira_token)),
        )
        .send_json(payload);
    match resp {
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
        Ok(_) => {
            println!("{}", success_message);
        }
    }
}

pub fn put_request(
    url: &str,
    payload: Value,
    jira_user: &str,
    jira_token: &str,
    success_message: &str,
) {
    let resp = put(url)
        .set("Accept", "application/json")
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Basic {b64}", b64 = b64auth(jira_user, jira_token)),
        )
        .send_json(payload);
    match resp {
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
        Ok(_) => {
            println!("{}", success_message);
        }
    }
}
