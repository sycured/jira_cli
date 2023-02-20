/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::{collections::HashMap, process::exit};

use comfy_table::{Cell, CellAlignment};
use rayon::prelude::*;
use serde_json::{json, Value};

use jira_cli::{confirm, create_and_print_table, delete_request, get_request, post_request};

use crate::urls::URLS;

pub fn add_user(global: &HashMap<&str, &str>, account_id: &str, group_id: &str) {
    let url: String = format!(
        "https://{}{}/user?groupId={group_id}",
        global["domain"], URLS["group"]
    );
    let payload: Value = json!({ "accountId": account_id });
    match post_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Account id {account_id} added to group id {group_id}"),
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}

pub fn create(global: &HashMap<&str, &str>, name: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["group"]);
    let payload: Value = json!({ "name": name });
    match post_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Group {name} created"),
        Err(e) => {
            eprintln!("Impossible to create group {name}: {e}");
            exit(1)
        }
    }
}

#[allow(clippy::unit_arg)]
pub fn delete(global: &HashMap<&str, &str>, group_id: &str) {
    let url: String = format!(
        "https://{}{}?groupId={group_id}",
        global["domain"], URLS["group"]
    );
    if confirm(format!(
        "Are you sure you want to delete the group id: {group_id}?"
    )) {
        match delete_request(&url, global["user"], global["token"]) {
            Ok(_) => println!("Are you sure you want to delete the group id: {group_id}?"),
            Err(e) => {
                eprintln!("Impossible to delete group id {group_id}: {e}");
                exit(1)
            }
        }
    } else {
        println!("Group id {group_id} not deleted.");
    }
}

//noinspection DuplicatedCode
pub fn find(global: &HashMap<&str, &str>, query: &str) {
    let url: String = format!(
        "https://{}{}/picker?query={query}",
        global["domain"], URLS["groups"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to find group {query}: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
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
            );
        }
    }
}

//noinspection DuplicatedCode
pub fn list_groups(global: &HashMap<&str, &str>, start_at: &str, max_results: &str) {
    let url: String = format!(
        "https://{}{}/bulk?startAt={start_at}&maxResults={max_results}",
        global["domain"], URLS["group"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to list groups: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
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
            );
        }
    }
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
        "https://{}{}/member?groupId={group_id}&includeInactiveUsers={include_inactive}&startAt={start_at}&maxResults={max_results}",
        global["domain"], URLS["group"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to list users in group {group_id}: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            if json["values"] == json!(null) {
                println!("No users found.");
            } else {
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
                );
            }
        }
    }
}

#[allow(clippy::unit_arg)]
pub fn remove_user(global: &HashMap<&str, &str>, account_id: &str, group_id: &str) {
    let url: String = format!(
        "https://{}{}/user?groupId={group_id}&accountId={account_id}",
        global["domain"], URLS["group"]
    );
    if confirm(format!(
        "Are you sure you want to remove account id {account_id} from group id: {group_id}?"
    )) {
        match delete_request(&url, global["user"], global["token"]) {
            Err(e) => {
                eprintln!("Impossible to remove user: {e}");
                exit(1)
            }
            Ok(_) => println!("Account id {account_id} removed from group id {group_id}",),
        }
    } else {
        println!("Account id {account_id} not removed from group id {group_id}",);
    }
}
