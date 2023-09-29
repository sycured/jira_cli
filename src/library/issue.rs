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
    create_and_print_table, delete_request, generate_url, get_request, handle_error_and_exit,
    post_request, print_output, put_request, Global,
};

pub fn add_label(global: &Global, issue_key: &str, label: &str) {
    let url: String = generate_url(&global.domain, "issue", Some(&format!("/{issue_key}")));
    let payload: Value = json!({
        "update": {
            "labels": [
                {
                    "add": label
                }
            ]
        }
    });
    match put_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!("Label {label} added to issue {issue_key}")),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to add label {label} to issue {issue_key} {e}"
        )),
    }
}

pub fn add_version(global: &Global, version_name: &str, issue_key: &str) {
    let url: String = generate_url(&global.domain, "issue", Some(&format!("/{issue_key}")));
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
    match put_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!(
            "Version {version_name} added to issue {issue_key}"
        )),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to add version {version_name} to issue {issue_key} {e}",
        )),
    }
}

pub fn add_vote(global: &Global, issue_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}/votes")),
    );
    let payload: Value = json!({});
    match post_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!("Vote added to issue {issue_key}")),
        Err(e) => {
            handle_error_and_exit(&format!("Impossible to add vote to issue {issue_key}: {e}"));
        }
    }
}

pub fn assign(global: &Global, issue_key: &str, account_id: &str, success_message: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}/assignee")),
    );
    let payload: Value = json!({ "accountId": account_id });
    match put_request(&url, &payload, global.b64auth()) {
        Ok(_) => println!("{success_message}"),
        Err(e) => {
            handle_error_and_exit(&format!("Impossible to assign the issue {issue_key}: {e}"));
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn create(
    global: &Global,
    reporter_account_id: &str,
    project_key: &str,
    issue_type: &str,
    summary: &str,
    description: &str,
    priority: &str,
) {
    let url: String = generate_url(&global.domain, "issue", None);
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

    match post_request(&url, &payload, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Failed to create issue: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            print_output(&format!("Issue created: {}", json["key"]));
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn create_link_type(global: &Global, name: &str, inward: &str, outward: &str) {
    let url: String = generate_url(&global.domain, "issue_link_types", None);
    let payload: Value = json!({
        "name": name,
        "inward": inward,
        "outward": outward
    });
    match post_request(&url, &payload, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Failed to create issue link type: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            if json["errorMessages"].is_null() {
                print_output(&format!(
                    "New link type {} (id: {} ) created",
                    json["name"].as_str().unwrap(),
                    json["id"].as_str().unwrap()
                ));
            } else {
                print_output(json["errorMessages"][0].as_str().unwrap());
            }
        }
    }
}

pub fn delete(global: &Global, issue_key: &str, delete_subtasks: bool) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}?deleteSubtasks={delete_subtasks}")),
    );
    let success_message: String = if delete_subtasks {
        format!("Issue {issue_key} deleted with its subtasks")
    } else {
        format!("Issue {issue_key} deleted")
    };
    match delete_request(&url, global.b64auth()) {
        Ok(_) => print_output(&success_message),
        Err(e) => handle_error_and_exit(&format!("Impossible to delete issue {issue_key}: {e}")),
    }
}

pub fn delete_link_type(global: &Global, link_type_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue_link_types",
        Some(&format!("/{link_type_id}")),
    );
    match delete_request(&url, global.b64auth()) {
        Ok(_) => print_output(&format!("Link type {link_type_id} deleted")),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to delete link type id {link_type_id}: {e}"
        )),
    }
}

//noinspection DuplicatedCode
#[allow(clippy::missing_panics_doc)]
pub fn get_link_type(global: &Global, link_type_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue_link_types",
        Some(&format!("/{link_type_id}")),
    );
    match get_request(&url, global.b64auth()) {
        Err(e) => {
            handle_error_and_exit(&format!("Impossible to get link type {link_type_id}: {e}"));
        }
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            let mut rows: Vec<Vec<Cell>> = Vec::new();
            let id: &str = json["id"].as_str().expect("Failed to get id");
            let name: &str = json["name"].as_str().expect("Failed to get name");
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

#[allow(clippy::missing_panics_doc)]
pub fn get_transitions(global: &Global, issue_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}/transitions")),
    );
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!(
            "Failed to get transitions for issue {issue_key}: {e}"
        )),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
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
#[allow(clippy::missing_panics_doc)]
pub fn list_link_types(global: &Global) {
    let url: String = generate_url(&global.domain, "issue_link_types", None);
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Failed to get issue link types: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
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

#[allow(clippy::missing_panics_doc)]
pub fn list_priorities(global: &Global) {
    let url: String = generate_url(&global.domain, "priority", None);
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Impossible to get priorities: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            json.as_array()
                .unwrap()
                .par_iter()
                .for_each(|x| print_output(&format!("{}", x["name"])));
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn list_types(global: &Global, project_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/createmeta?projectKeys={project_key}")),
    );
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!("Impossible to list types: {e}")),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
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
#[allow(clippy::missing_panics_doc)]
pub fn list_votes(global: &Global, issue_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}/votes")),
    );
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to list votes for issue {issue_key}: {e}"
        )),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
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
                print_output(&format!("Issue {issue_key} has 0 vote"));
            }
        }
    }
}

pub fn remove_label(global: &Global, issue_key: &str, label: &str) {
    let url: String = generate_url(&global.domain, "issue", Some(&format!("/{issue_key}")));
    let payload: Value = json!({
        "update": {
            "labels": [
                {
                    "remove": label
                }
            ]
        }
    });
    match put_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!("Label {label} removed from issue {issue_key}")),
        Err(e) => handle_error_and_exit(&format!(
            "Failed to remove label {label} from issue {issue_key}: {e}",
        )),
    }
}

pub fn remove_version(global: &Global, version_name: &str, issue_key: &str) {
    let url: String = generate_url(&global.domain, "issue", Some(&format!("/{issue_key}")));
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

    match put_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!(
            "Version {version_name} removed from issue {issue_key}"
        )),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to remove version {version_name} from issue {issue_key}: {e}",
        )),
    }
}

pub fn remove_vote(global: &Global, issue_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}/votes")),
    );
    match delete_request(&url, global.b64auth()) {
        Ok(_) => print_output(&format!("Vote removed from issue {issue_key}")),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to remove vote from issue {issue_key}: {e}"
        )),
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn show_fixversions(global: &Global, issue_key: &str) {
    let url: String = generate_url(&global.domain, "issue", Some(&format!("/{issue_key}")));
    match get_request(&url, global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to list fix versions for issue {issue_key}: {e}"
        )),
        Ok(response) => {
            let json: Value = response.json().expect("Failed to parse json");
            json["fields"]["fixVersions"]
                .as_array()
                .unwrap()
                .par_iter()
                .for_each(|x| print_output(&format!("{}", x["name"])));
        }
    }
}

pub fn transition(global: &Global, issue_key: &str, transition_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "issue",
        Some(&format!("/{issue_key}/transitions")),
    );
    let payload: Value = json!({
        "transition": {
            "id": transition_id
        }
    });
    match post_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output("Success"),
        Err(e) => handle_error_and_exit(&format!("Failed to transition issue {issue_key}: {e}")),
    }
}

pub fn update_link_type(
    global: &Global,
    link_type_id: &str,
    link_type_name: &str,
    link_type_inward: &str,
    link_type_outward: &str,
) {
    let url: String = generate_url(
        &global.domain,
        "issue_link_types",
        Some(&format!("/{link_type_id}")),
    );
    let payload: Value = json!({
        "name": link_type_name,
        "inward": link_type_inward,
        "outward": link_type_outward
    });
    match put_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!("Link type {link_type_id} updated")),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to update link type {link_type_id}: {e}"
        )),
    }
}
