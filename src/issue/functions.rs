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

use jira_cli::{create_and_print_table, delete_request, get_request, post_request, put_request};

use crate::urls::URLS;

pub fn add_label(global: &HashMap<&str, &str>, issue_key: &str, label: &str) {
    let url: String = format!("https://{}{}/{issue_key}", global["domain"], URLS["issue"]);
    let payload: Value = json!({
        "update": {
            "labels": [
                {
                    "add": label
                }
            ]
        }
    });
    match put_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Label {label} added to issue {issue_key}"),
        Err(e) => {
            eprintln!("Impossible to add label {label} to issue {issue_key} {e}");
            exit(1)
        }
    }
}

pub fn add_version(global: &HashMap<&str, &str>, version_name: &str, issue_key: &str) {
    let url: String = format!("https://{}{}/{issue_key}", global["domain"], URLS["issue"]);
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
    match put_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Version {version_name} added to issue {issue_key}"),
        Err(e) => {
            eprintln!("Impossible to add version {version_name} to issue {issue_key} {e}",);
            exit(1)
        }
    }
}

pub fn add_vote(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{issue_key}/votes",
        global["domain"], URLS["issue"]
    );
    let payload: Value = json!({});
    match post_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Vote added to issue {issue_key}"),
        Err(e) => {
            eprintln!("Impossible to add vote to issue {issue_key}: {e}");
        }
    }
}

pub fn assign(
    global: &HashMap<&str, &str>,
    issue_key: &str,
    account_id: &str,
    success_message: &str,
) {
    let url: String = format!(
        "https://{}{}/{issue_key}/assignee",
        global["domain"], URLS["issue"]
    );
    let payload: Value = json!({ "accountId": account_id });
    match put_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("{success_message}"),
        Err(e) => {
            eprintln!("Impossible to assign the issue {issue_key}: {e}");
            exit(1)
        }
    }
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
    let url: String = format!("https://{}{}", global["domain"], URLS["issue"]);
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

    match post_request(&url, &payload, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Failed to create issue: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            println!("Issue created: {}", json["key"]);
        }
    }
}

pub fn create_link_type(global: &HashMap<&str, &str>, name: &str, inward: &str, outward: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["issue_link_types"]);
    let payload: Value = json!({
        "name": name,
        "inward": inward,
        "outward": outward
    });
    match post_request(&url, &payload, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Failed to create issue link type: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            if json["errorMessages"].is_null() {
                println!(
                    "New link type {} (id: {} ) created",
                    json["name"].as_str().unwrap(),
                    json["id"].as_str().unwrap()
                );
            } else {
                println!("{}", json["errorMessages"][0].as_str().unwrap());
            }
        }
    }
}

pub fn delete(global: &HashMap<&str, &str>, issue_key: &str, delete_subtasks: &str) {
    let url: String = format!(
        "https://{}{}/{issue_key}?deleteSubtasks={delete_subtasks}",
        global["domain"], URLS["issue"]
    );
    let mut success_message: String = format!("Issue {issue_key} deleted",);
    if delete_subtasks == "true" {
        success_message += " with its subtasks";
    }
    match delete_request(&url, global["user"], global["token"]) {
        Ok(_) => println!("{success_message}"),
        Err(e) => {
            eprintln!("Impossible to delete issue {issue_key}: {e}");
            exit(1)
        }
    }
}

pub fn delete_link_type(global: &HashMap<&str, &str>, link_type_id: &str) {
    let url: String = format!(
        "https://{}{}/{link_type_id}",
        global["domain"], URLS["issue_link_types"]
    );
    match delete_request(&url, global["user"], global["token"]) {
        Ok(_) => println!("Link type {link_type_id} deleted"),
        Err(e) => {
            eprintln!("Impossible to delete link type id {link_type_id}: {e}");
            exit(1)
        }
    }
}

//noinspection DuplicatedCode
pub fn get_link_type(global: &HashMap<&str, &str>, link_type_id: &str) {
    let url: String = format!(
        "https://{}{}/{link_type_id}",
        global["domain"], URLS["issue_link_types"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to get link type {link_type_id}: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            let mut rows: Vec<Vec<Cell>> = Vec::new();
            let id: &str = json["id"].as_str().unwrap();
            let name: &str = json["name"].as_str().unwrap();
            let inward: &str = json["inward"].as_str().unwrap_or("");
            let outward: &str = json["outward"].as_str().unwrap_or("");
            rows.push(vec![
                Cell::new(id),
                Cell::new(name),
                Cell::new(inward),
                Cell::new(outward),
            ]);
            create_and_print_table(
                vec!["ID", "Name", "Inward", "Outward"],
                &HashMap::from([
                    (0, CellAlignment::Center),
                    (1, CellAlignment::Center),
                    (2, CellAlignment::Center),
                    (3, CellAlignment::Center),
                ]),
                rows,
            );
        }
    }
}

pub fn get_transitions(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{issue_key}/transitions",
        global["domain"], URLS["issue"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Failed to get transitions for issue {issue_key}: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            let rows: Vec<Vec<Cell>> = json["transitions"]
                .as_array()
                .unwrap()
                .par_iter()
                .map(|x| {
                    vec![
                        Cell::new(x["id"].as_str().unwrap()),
                        Cell::new(x["name"].as_str().unwrap()),
                        Cell::new(x["to"]["name"].as_str().unwrap_or("")),
                    ]
                })
                .collect();
            create_and_print_table(
                vec!["ID", "Name", "To Name"],
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

//noinspection DuplicatedCode
pub fn list_link_types(global: &HashMap<&str, &str>) {
    let url: String = format!("https://{}{}", global["domain"], URLS["issue_link_types"]);
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Failed to get issue link types: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            let rows: Vec<Vec<Cell>> = json["issueLinkTypes"]
                .as_array()
                .unwrap()
                .par_iter()
                .map(|x| {
                    vec![
                        Cell::new(x["id"].as_str().unwrap()),
                        Cell::new(x["name"].as_str().unwrap()),
                        Cell::new(x["inward"].as_str().unwrap_or("")),
                        Cell::new(x["outward"].as_str().unwrap_or("")),
                    ]
                })
                .collect();
            create_and_print_table(
                vec!["ID", "Name", "Inward", "Outward"],
                &HashMap::from([
                    (0, CellAlignment::Center),
                    (1, CellAlignment::Center),
                    (2, CellAlignment::Center),
                    (3, CellAlignment::Center),
                ]),
                rows,
            );
        }
    }
}

pub fn list_priorities(global: &HashMap<&str, &str>) {
    let url: String = format!("https://{}{}", global["domain"], URLS["priority"]);
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to get priorities: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            json.as_array().unwrap().par_iter().for_each(|x| {
                println!("{}", x["name"]);
            });
        }
    }
}

pub fn list_types(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/createmeta?projectKeys={project_key}",
        global["domain"], URLS["issue"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to list types: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            json["projects"][0]["issuetypes"]
                .as_array()
                .unwrap()
                .par_iter()
                .for_each(|x| {
                    println!("{}", x["name"]);
                });
        }
    }
}

//noinspection DuplicatedCode
pub fn list_votes(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{issue_key}/votes",
        global["domain"], URLS["issue"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to list votes for issue {issue_key}: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            if json["hasVoted"] == "true" {
                println!("Votes: {}", json["votes"]);
                let rows: Vec<Vec<Cell>> = json["voters"]
                    .as_array()
                    .unwrap()
                    .par_iter()
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
            } else {
                println!("Issue {issue_key} has 0 vote");
            }
        }
    }
}

pub fn remove_label(global: &HashMap<&str, &str>, issue_key: &str, label: &str) {
    let url: String = format!("https://{}{}/{issue_key}", global["domain"], URLS["issue"]);
    let payload: Value = json!({
        "update": {
            "labels": [
                {
                    "remove": label
                }
            ]
        }
    });
    match put_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Label {label} removed from issue {issue_key}"),
        Err(e) => {
            eprintln!("Failed to remove label {label} from issue {issue_key}: {e}",);
            exit(1)
        }
    }
}

pub fn remove_version(global: &HashMap<&str, &str>, version_name: &str, issue_key: &str) {
    let url: String = format!("https://{}{}/{issue_key}", global["domain"], URLS["issue"]);
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

    match put_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Version {version_name} removed from issue {issue_key}"),
        Err(e) => {
            eprintln!("Impossible to remove version {version_name} from issue {issue_key}: {e}",);
            exit(1)
        }
    }
}

pub fn remove_vote(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{issue_key}/votes",
        global["domain"], URLS["issue"]
    );
    match delete_request(&url, global["user"], global["token"]) {
        Ok(_) => println!("Vote removed from issue {issue_key}"),
        Err(e) => {
            eprintln!("Impossible to remove vote from issue {issue_key}: {e}");
            exit(1)
        }
    }
}

pub fn show_fixversions(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!("https://{}{}/{issue_key}", global["domain"], URLS["issue"]);
    match get_request(&url, global["user"], global["token"]) {
        Err(e) => {
            eprintln!("Impossible to list fix versions for issue {issue_key}: {e}");
            exit(1)
        }
        Ok(response) => {
            let json: Value = response.json().unwrap();
            json["fields"]["fixVersions"]
                .as_array()
                .unwrap()
                .par_iter()
                .for_each(|x| {
                    println!("{}", x["name"]);
                });
        }
    }
}

pub fn transition(global: &HashMap<&str, &str>, issue_key: &str, transition_id: &str) {
    let url: String = format!(
        "https://{}{}/{issue_key}/transitions",
        global["domain"], URLS["issue"]
    );
    let payload: Value = json!({
        "transition": {
            "id": transition_id
        }
    });
    match post_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Success"),
        Err(e) => {
            eprintln!("Failed to transition issue {issue_key}: {e}");
            exit(1)
        }
    }
}

pub fn update_link_type(
    global: &HashMap<&str, &str>,
    link_type_id: &str,
    link_type_name: &str,
    link_type_inward: &str,
    link_type_outward: &str,
) {
    let url: String = format!(
        "https://{}{}/{link_type_id}",
        global["domain"], URLS["issue_link_types"]
    );
    let payload: Value = json!({
        "name": link_type_name,
        "inward": link_type_inward,
        "outward": link_type_outward
    });
    match put_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("Link type {link_type_id} updated"),
        Err(e) => {
            eprintln!("Impossible to update link type {link_type_id}: {e}");
            exit(1)
        }
    }
}
