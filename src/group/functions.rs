use std::collections::HashMap;

use attohttpc::Response;
use comfy_table::{Cell, CellAlignment};
use serde_json::{json, Value};

use crate::{
    lib::{confirm, create_table, delete_request, get_request, post_request},
    urls::URLS,
};

pub fn add_user(global: &HashMap<&str, &str>, account_id: &str, group_id: &str) {
    let url: String = format!(
        "https://{}{}/user?groupId={}",
        global["domain"], URLS["group"], group_id
    );
    let payload: Value = json!({ "accountId": account_id });
    let success_message: String =
        format!("Account id {} added to group id {}", account_id, group_id);
    post_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        &success_message,
        false,
    );
}

pub fn create(global: &HashMap<&str, &str>, name: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["group"]);
    let payload: Value = json!({ "name": name });
    let success_message: String = format!("Group {} created", name);
    post_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        &success_message,
        false,
    );
}

pub fn delete(global: &HashMap<&str, &str>, group_id: &str) {
    let url: String = format!(
        "https://{}{}?groupId={}",
        global["domain"], URLS["group"], group_id
    );
    let success_message: String = format!("Group id {} deleted", group_id);
    confirm(
        format!(
            "Are you sure you want to delete the group id: {}?",
            group_id
        ),
        delete_request(&url, global["user"], global["token"], &success_message),
        println!("Group id {} not deleted.", group_id),
    );
}

pub fn find(global: &HashMap<&str, &str>, query: &str) {
    let url: String = format!(
        "https://{}{}/picker?query={}",
        global["domain"], URLS["groups"], query
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    json["groups"].as_array().unwrap().iter().for_each(|x| {
        let name: &str = x["name"].as_str().unwrap();
        let group_id: &str = x["groupId"].as_str().unwrap();
        rows.push(vec![Cell::new(name), Cell::new(group_id)]);
    });
    let table = create_table(
        vec!["Group Name", "Group ID"],
        &HashMap::from([(0, CellAlignment::Center), (1, CellAlignment::Center)]),
        rows,
    );
    println!("{}", table);
}

pub fn list_groups(global: &HashMap<&str, &str>, start_at: &str, max_results: &str) {
    let url: String = format!(
        "https://{}{}/bulk?startAt={}&maxResults={}",
        global["domain"], URLS["group"], start_at, max_results
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    json["values"].as_array().unwrap().iter().for_each(|x| {
        let name: &str = x["name"].as_str().unwrap();
        let group_id: &str = x["groupId"].as_str().unwrap();
        rows.push(vec![Cell::new(name), Cell::new(group_id)]);
    });
    let table = create_table(
        vec!["Group Name", "Group ID"],
        &HashMap::from([(0, CellAlignment::Center), (1, CellAlignment::Center)]),
        rows,
    );
    println!("{}", table);
}

pub fn list_users(
    global: &HashMap<&str, &str>,
    group_id: &str,
    include_inactive: bool,
    start_at: &str,
    max_results: &str,
) {
    let url: String = format!(
        "https://{}{}/member?groupId={}&includeInactiveUsers={}&startAt={}&maxResults={}",
        global["domain"], URLS["group"], group_id, include_inactive, start_at, max_results
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    if json["values"] != json!(null) {
        json["values"].as_array().unwrap().iter().for_each(|x| {
            let name: &str = x["name"].as_str().unwrap_or("");
            let account_id: &str = x["accountId"].as_str().unwrap();
            let display_name: &str = x["displayName"].as_str().unwrap_or("");
            rows.push(vec![
                Cell::new(name),
                Cell::new(account_id),
                Cell::new(display_name),
            ]);
        });
        let table = create_table(
            vec!["Name", "Account ID", "Display Name"],
            &HashMap::from([
                (0, CellAlignment::Center),
                (1, CellAlignment::Center),
                (2, CellAlignment::Center),
            ]),
            rows,
        );
        println!("{}", table);
    } else {
        println!("No users found.");
    }
}

pub fn remove_user(global: &HashMap<&str, &str>, account_id: &str, group_id: &str) {
    let url: String = format!(
        "https://{}{}/user?groupId={}&accountId={}",
        global["domain"], URLS["group"], group_id, account_id
    );
    let success_message: String = format!(
        "Account id {} removed from group id {}",
        account_id, group_id
    );
    confirm(
        format!(
            "Are you sure you want to remove account id {} from group id: {}?",
            account_id, group_id
        ),
        delete_request(&url, global["user"], global["token"], &success_message),
        println!(
            "Account id {} not removed from group id {}",
            account_id, group_id
        ),
    );
}
