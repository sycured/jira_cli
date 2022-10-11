/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use comfy_table::{Cell, CellAlignment};
use itertools::Itertools;
use rayon::prelude::*;
use serde_json::{json, Value};

use jira_cli::{
    confirm, create_and_print_table, delete_request, get_request, post_request, put_request,
};

use crate::urls::URLS;

#[allow(clippy::too_many_arguments)]
pub fn create(
    global: &HashMap<&str, &str>,
    project_name: &str,
    project_key: &str,
    project_leadaccountid: &str,
    project_type: &str,
    project_template: &str,
) {
    let url: String = format!("https://{}{}", global["domain"], URLS["project"]);
    let payload: Value = json!({
        "name": project_name,
        "key": project_key,
        "leadAccountId": project_leadaccountid,
        "projectTypeKey": project_type,
        "projectTemplateKey": project_template,
        "assigneeType": "UNASSIGNED"
    });
    if post_request(&url, &payload, global["user"], global["token"], false).unwrap_left() {
        println!("Project {} created", project_key);
    }
}

#[allow(clippy::unit_arg)]
pub fn delete_project(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["project"], project_key
    );
    let success_message: String = format!("Project {} deleted", project_key);
    if confirm(format!(
        "Are you sure you want to delete the project key: {}?",
        project_key
    )) {
        delete_request(&url, global["user"], global["token"], &success_message);
    } else {
        println!("Project {} not deleted.", project_key);
    }
}

pub fn get_id(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["project"], project_key
    );
    let json: Value = get_request(&url, global["user"], global["token"])
        .json()
        .unwrap();
    println!("{}", json["id"].as_str().unwrap().parse::<i32>().unwrap());
}

pub fn list_features(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/features",
        global["domain"], URLS["project"], project_key
    );
    let json: Value = get_request(&url, global["user"], global["token"])
        .json()
        .unwrap();
    let rows: Vec<Vec<Cell>> = json["features"]
        .as_array()
        .unwrap()
        .par_iter()
        .map(|x| {
            let locked: bool = x["toggleLocked"].as_bool().unwrap();
            let locked_color = if locked {
                comfy_table::Color::Red
            } else {
                comfy_table::Color::Green
            };
            vec![
                Cell::new(x["feature"].as_str().unwrap()),
                Cell::new(x["localisedDescription"].as_str().unwrap()),
                Cell::new(x["state"].as_str().unwrap()),
                Cell::new(&locked.to_string()).fg(locked_color),
            ]
        })
        .collect();
    create_and_print_table(
        vec!["Key", "Description", "State", "Locked"],
        &HashMap::from([
            (0, CellAlignment::Center),
            (2, CellAlignment::Center),
            (3, CellAlignment::Center),
        ]),
        rows,
    );
}

pub fn list_versions(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/versions",
        global["domain"], URLS["project"], project_key
    );
    let json: Value = get_request(&url, global["user"], global["token"])
        .json()
        .unwrap();
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    json.as_array()
        .unwrap()
        .iter()
        .sorted_by_key(|item| item["name"].as_str().unwrap())
        .for_each(|x| {
            let id: &str = x["id"].as_str().unwrap();
            let name: &str = x["name"].as_str().unwrap();
            let description: &str = x["description"].as_str().unwrap_or("");
            let released: bool = x["released"].as_bool().unwrap();
            let archived: bool = x["archived"].as_bool().unwrap();
            let released_color = if released {
                comfy_table::Color::Green
            } else {
                comfy_table::Color::Red
            };
            let archived_color = if archived {
                comfy_table::Color::Red
            } else {
                comfy_table::Color::Green
            };
            rows.push(vec![
                Cell::new(name),
                Cell::new(description),
                Cell::new(&released.to_string()).fg(released_color),
                Cell::new(&archived.to_string()).fg(archived_color),
                Cell::new(id),
            ]);
        });
    create_and_print_table(
        vec!["Name", "Description", "Released", "Archived", "Id"],
        &HashMap::from([
            (0, CellAlignment::Center),
            (1, CellAlignment::Center),
            (2, CellAlignment::Center),
            (3, CellAlignment::Center),
            (4, CellAlignment::Center),
        ]),
        rows,
    );
}

pub fn new_version(global: &HashMap<&str, &str>, project_id: &str, version_name: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["version"]);
    let payload: Value = json!({
      "name": version_name,
      "projectId": project_id.parse::<i32>().unwrap()
    });
    if post_request(&url, &payload, global["user"], global["token"], false).unwrap_left() {
        println!("Version created: {}", version_name);
    }
}

pub fn set_feature_state(
    global: &HashMap<&str, &str>,
    project_key: &str,
    project_feature_key: &str,
    project_feature_state: &str,
) {
    let url: String = format!(
        "https://{}{}/{}/features/{}",
        global["domain"], URLS["project"], project_key, project_feature_key
    );
    let payload: Value = json!({ "state": project_feature_state });
    let success_message: String = format!(
        "Feature {} set to {} on project {}",
        project_feature_key, project_feature_state, project_key
    );
    put_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        &success_message,
    );
}
