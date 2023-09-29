/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use comfy_table::{Cell, CellAlignment};
use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    confirm, create_and_print_table, delete_request, generate_url, get_request,
    handle_error_and_exit, post_request, print_output, Global,
};

pub fn add_user(global: &Global, account_id: &str, group_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "group",
        Some(&format!("/user?groupId={group_id}")),
    );
    let payload: Value = json!({ "accountId": account_id });
    match post_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!(
            "Account id {account_id} added to group id {group_id}"
        )),
        Err(e) => handle_error_and_exit(&format!("{e}")),
    }
}

pub fn create(global: &Global, name: &str) {
    let url: String = generate_url(&global.domain, "group", None);
    let payload: Value = json!({ "name": name });
    match post_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!("Group {name} created")),
        Err(e) => handle_error_and_exit(&format!("Impossible to create group {name}: {e}")),
    }
}

#[allow(clippy::unit_arg)]
pub fn delete(global: &Global, group_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "group",
        Some(&format!("?groupId={group_id}")),
    );
    if confirm(format!(
        "Are you sure you want to delete the group id: {group_id}?"
    )) {
        match delete_request(&url, global.b64auth()) {
            Ok(_) => print_output(&format!("The group id {group_id} has been deleted.")),
            Err(e) => {
                handle_error_and_exit(&format!("Impossible to delete group id {group_id}: {e}"))
            }
        }
    } else {
        print_output(&format!("Group id {group_id} not deleted."));
    }
}

//noinspection DuplicatedCode
#[allow(clippy::missing_panics_doc)]
pub fn find(global: &Global, query: &str) {
    let url: String = generate_url(
        &global.domain,
        "groups",
        Some(&format!("/picker?query={query}")),
    );
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Impossible to find group {query}: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
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
#[allow(clippy::missing_panics_doc)]
pub fn list_groups(global: &Global, start_at: &str, max_results: &str) {
    let url: String = generate_url(
        &global.domain,
        "group",
        Some(&format!(
            "/bulk?startAt={start_at}&maxResults={max_results}"
        )),
    );
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Impossible to list groups: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
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
#[allow(clippy::missing_panics_doc)]
pub fn list_users(
    global: &Global,
    group_id: &str,
    include_inactive: bool,
    start_at: &str,
    max_results: &str,
) {
    let url: String = generate_url(
        &global.domain, "group", Some(&format!("/member?groupId={group_id}&includeInactiveUsers={include_inactive}&startAt={start_at}&maxResults={max_results}")));
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to list users in group {group_id}: {e}"
        )),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            if json["values"] == json!(null) {
                print_output("No users found.");
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
pub fn remove_user(global: &Global, account_id: &str, group_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "group",
        Some(&format!("/user?groupId={group_id}&accountId={account_id}")),
    );
    if confirm(format!(
        "Are you sure you want to remove account id {account_id} from group id: {group_id}?"
    )) {
        match delete_request(&url, global.b64auth()) {
            Err(e) => handle_error_and_exit(&format!("Impossible to remove user: {e}")),
            Ok(_) => print_output(&format!(
                "Account id {account_id} removed from group id {group_id}"
            )),
        }
    } else {
        print_output(&format!(
            "Account id {account_id} not removed from group id {group_id}"
        ));
    }
}
