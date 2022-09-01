/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use attohttpc::Response;
use comfy_table::{Cell, CellAlignment};
use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    lib::{confirm, create_and_print_table, delete_request, get_request, post_request},
    urls::URLS,
};

pub fn add_user(global: &HashMap<&str, &str>, account_id: &str, group_id: &str) {
    let url: String = format!(
        "https://{}{}/user?groupId={}",
        global["domain"], URLS["group"], group_id
    );
    let payload: Value = json!({ "accountId": account_id });
    if post_request(&url, &payload, global["user"], global["token"], false).unwrap_left() {
        println!("Account id {} added to group id {}", account_id, group_id);
    }
}

pub fn create(global: &HashMap<&str, &str>, name: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["group"]);
    let payload: Value = json!({ "name": name });
    if post_request(&url, &payload, global["user"], global["token"], false).unwrap_left() {
        println!("Group {} created", name);
    }
}

#[allow(clippy::unit_arg)]
pub fn delete(global: &HashMap<&str, &str>, group_id: &str) {
    let url: String = format!(
        "https://{}{}?groupId={}",
        global["domain"], URLS["group"], group_id
    );
    let success_message: String = format!("Group id {} deleted", group_id);
    match confirm(format!(
        "Are you sure you want to delete the group id: {}?",
        group_id
    )) {
        true => delete_request(&url, global["user"], global["token"], &success_message),
        false => println!("Group id {} not deleted.", group_id),
    }
}

//noinspection DuplicatedCode
pub fn find(global: &HashMap<&str, &str>, query: &str) {
    let url: String = format!(
        "https://{}{}/picker?query={}",
        global["domain"], URLS["groups"], query
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    let rows: Vec<Vec<Cell>> = json["groups"]
        .as_array()
        .unwrap()
        .par_iter()
        .map(|x| {
            vec![
                Cell::new(x["name"].as_str().unwrap()),
                Cell::new(x["groupId"].as_str().unwrap()),
            ]
        })
        .collect();
    create_and_print_table(
        vec!["Group Name", "Group ID"],
        &HashMap::from([(0, CellAlignment::Center), (1, CellAlignment::Center)]),
        rows,
    )
}

//noinspection DuplicatedCode
pub fn list_groups(global: &HashMap<&str, &str>, start_at: &str, max_results: &str) {
    let url: String = format!(
        "https://{}{}/bulk?startAt={}&maxResults={}",
        global["domain"], URLS["group"], start_at, max_results
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    let rows: Vec<Vec<Cell>> = json["values"]
        .as_array()
        .unwrap()
        .par_iter()
        .map(|x| {
            vec![
                Cell::new(x["name"].as_str().unwrap()),
                Cell::new(x["groupId"].as_str().unwrap()),
            ]
        })
        .collect();
    create_and_print_table(
        vec!["Group Name", "Group ID"],
        &HashMap::from([(0, CellAlignment::Center), (1, CellAlignment::Center)]),
        rows,
    )
}

//noinspection DuplicatedCode
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
    if json["values"] != json!(null) {
        let rows: Vec<Vec<Cell>> = json["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                vec![
                    Cell::new(x["name"].as_str().unwrap_or("")),
                    Cell::new(x["accountId"].as_str().unwrap()),
                    Cell::new(x["displayName"].as_str().unwrap_or("")),
                ]
            })
            .collect();
        create_and_print_table(
            vec!["Name", "Account ID", "Display Name"],
            &HashMap::from([
                (0, CellAlignment::Center),
                (1, CellAlignment::Center),
                (2, CellAlignment::Center),
            ]),
            rows,
        )
    } else {
        println!("No users found.");
    }
}

#[allow(clippy::unit_arg)]
pub fn remove_user(global: &HashMap<&str, &str>, account_id: &str, group_id: &str) {
    let url: String = format!(
        "https://{}{}/user?groupId={}&accountId={}",
        global["domain"], URLS["group"], group_id, account_id
    );
    let success_message: String = format!(
        "Account id {} removed from group id {}",
        account_id, group_id
    );
    match confirm(format!(
        "Are you sure you want to remove account id {} from group id: {}?",
        account_id, group_id
    )) {
        true => delete_request(&url, global["user"], global["token"], &success_message),
        false => println!(
            "Account id {} not removed from group id {}",
            account_id, group_id
        ),
    }
}
