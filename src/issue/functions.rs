/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use attohttpc::Response;
use comfy_table::{Cell, CellAlignment};
use jira_cli::create_and_print_table;
use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    lib::{delete_request, get_request, post_request, put_request},
    urls::URLS,
};

pub fn add_label(global: &HashMap<&str, &str>, issue_key: &str, label: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue"], issue_key
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
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn add_version(global: &HashMap<&str, &str>, version_name: &str, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue"], issue_key
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
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn add_vote(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/votes",
        global["domain"], URLS["issue"], issue_key
    );
    let payload: Value = json!({});
    let success_message: String = format!("Vote added to issue {}", issue_key);
    put_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn assign(
    global: &HashMap<&str, &str>,
    issue_key: &str,
    account_id: &str,
    success_message: &str,
) {
    let url: String = format!(
        "https://{}{}/{}/assignee",
        global["domain"], URLS["issue"], issue_key
    );
    let payload: Value = json!({ "accountId": account_id });
    put_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        success_message,
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

    let resp = post_request(&url, &payload, global["user"], global["token"], true).unwrap_right();
    let json: Value = resp.json().unwrap();
    println!("Issue created: {}", json["key"]);
}

pub fn create_link_type(global: &HashMap<&str, &str>, name: &str, inward: &str, outward: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["issue_link_types"]);
    let payload: Value = json!({
        "name": name,
        "inward": inward,
        "outward": outward
    });
    let resp = post_request(&url, &payload, global["user"], global["token"], true).unwrap_right();
    let json: Value = resp.json().unwrap();
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

pub fn delete(global: &HashMap<&str, &str>, issue_key: &str, delete_subtasks: &str) {
    let url: String = format!(
        "https://{}{}/{}?deleteSubtasks={}",
        global["domain"], URLS["issue"], issue_key, delete_subtasks
    );
    let mut success_message: String = format!("Issue {} deleted", issue_key);
    if delete_subtasks == "true" {
        success_message += " with its subtasks";
    }
    delete_request(&url, global["user"], global["token"], &success_message);
}

pub fn delete_link_type(global: &HashMap<&str, &str>, link_type_id: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue_link_types"], link_type_id
    );
    let success_message: String = format!("Link type {} deleted", link_type_id);
    delete_request(&url, global["user"], global["token"], &success_message);
}

//noinspection DuplicatedCode
pub fn get_link_type(global: &HashMap<&str, &str>, link_type_id: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue_link_types"], link_type_id
    );
    let resp = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
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
    )
}

pub fn get_transitions(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/transitions",
        global["domain"], URLS["issue"], issue_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
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
    )
}

//noinspection DuplicatedCode
pub fn list_link_types(global: &HashMap<&str, &str>) {
    let url: String = format!("https://{}{}", global["domain"], URLS["issue_link_types"]);
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
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
    )
}

pub fn list_priorities(global: &HashMap<&str, &str>) {
    let url: String = format!("https://{}{}", global["domain"], URLS["priority"]);
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    json.as_array().unwrap().par_iter().for_each(|x| {
        println!("{}", x["name"]);
    });
}

pub fn list_types(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/createmeta?projectKeys={}",
        global["domain"], URLS["issue"], project_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    json["projects"][0]["issuetypes"]
        .as_array()
        .unwrap()
        .par_iter()
        .for_each(|x| {
            println!("{}", x["name"]);
        });
}

//noinspection DuplicatedCode
pub fn list_votes(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/votes",
        global["domain"], URLS["issue"], issue_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
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
        )
    } else {
        println!("Issue {} has 0 vote", issue_key);
    }
}

pub fn remove_label(global: &HashMap<&str, &str>, issue_key: &str, label: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue"], issue_key
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
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn remove_version(global: &HashMap<&str, &str>, version_name: &str, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue"], issue_key
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
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}

pub fn remove_vote(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/votes",
        global["domain"], URLS["issue"], issue_key
    );
    let success_message: String = format!("Vote removed from issue {}", issue_key);
    delete_request(&url, global["user"], global["token"], &success_message);
}

pub fn show_fixversions(global: &HashMap<&str, &str>, issue_key: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["issue"], issue_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    json["fields"]["fixVersions"]
        .as_array()
        .unwrap()
        .par_iter()
        .for_each(|x| {
            println!("{}", x["name"]);
        });
}

pub fn transition(global: &HashMap<&str, &str>, issue_key: &str, transition_id: &str) {
    let url: String = format!(
        "https://{}{}/{}/transitions",
        global["domain"], URLS["issue"], issue_key
    );
    let payload: Value = json!({
        "transition": {
            "id": transition_id
        }
    });
    let resp = post_request(&url, &payload, global["user"], global["token"], true).unwrap_right();
    match resp.status().as_str() {
        "204" => println!("Success"),
        "400" => println!("Request failed or is invalid"),
        "401" => eprintln!("Authentication credentials are incorrect or missing"),
        "404" => println!("The issue is not found or the user does not have permission to view it"),
        _ => panic!("Status code unknown"),
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
        "https://{}{}/{}",
        global["domain"], URLS["issue_link_types"], link_type_id
    );
    let payload: Value = json!({
        "name": link_type_name,
        "inward": link_type_inward,
        "outward": link_type_outward
    });
    let success_message: String = format!("Link type {} updated", link_type_id);
    put_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}
