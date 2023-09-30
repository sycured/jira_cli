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

use crate::{
    confirm, create_and_print_table, delete_request, generate_url, get_request,
    handle_error_and_exit, post_request, print_output, put_request, Global,
};

#[allow(clippy::too_many_arguments)]
pub fn create(
    global: &Global,
    project_name: &str,
    project_key: &str,
    project_leadaccountid: &str,
    project_type: &str,
    project_template: &str,
) {
    let url: String = generate_url(&global.domain, "project", None);
    let payload: Value = json!({
        "name": project_name,
        "key": project_key,
        "leadAccountId": project_leadaccountid,
        "projectTypeKey": project_type,
        "projectTemplateKey": project_template,
        "assigneeType": "UNASSIGNED"
    });
    match post_request(&url, &payload, &global.b64auth()) {
        Ok(_) => print_output(&format!("Project {project_key} created")),
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to create the project {project_key}: {e}"
        )),
    }
}

#[allow(clippy::unit_arg)]
pub fn delete(global: &Global, project_key: &str, enable_undo: bool) {
    let url: String = generate_url(
        &global.domain,
        "project",
        Some(&format!("/{project_key}?enableUndo={enable_undo}")),
    );
    if confirm(format!(
        "Are you sure you want to delete the project key: {project_key}?"
    )) {
        match delete_request(&url, &global.b64auth()) {
            Ok(_) => print_output(&format!("Project {project_key} deleted")),
            Err(e) => handle_error_and_exit(&format!(
                "Impossible to delete the project {project_key}: {e}"
            )),
        }
    } else {
        println!("Project {project_key} not deleted.");
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn get_id(global: &Global, project_key: &str) {
    let url: String = generate_url(&global.domain, "project", Some(&format!("/{project_key}")));
    match get_request(&url, &global.b64auth()) {
        Err(e) => {
            handle_error_and_exit(&format!("Impossible to get project {project_key} id: {e}"));
        }
        Ok(r) => {
            let json: Value = r.json().expect("Failed to parse json");
            print_output(&format!(
                "{}",
                json["id"]
                    .as_str()
                    .expect("Failed to get id")
                    .parse::<i32>()
                    .expect("Failed to parse id as i32")
            ));
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn list_features(global: &Global, project_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "project",
        Some(&format!("/{project_key}/features")),
    );
    match get_request(&url, &global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to list features for project {project_key}: {e}"
        )),
        Ok(r) => {
            let json: Value = r.json().expect("Faield to parse json");
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
                        Cell::new(locked.to_string()).fg(locked_color),
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
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn list_versions(global: &Global, project_key: &str) {
    let url: String = generate_url(
        &global.domain,
        "project",
        Some(&format!("/{project_key}/versions")),
    );
    match get_request(&url, &global.b64auth()) {
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to list versions on project {project_key}: {e}"
        )),
        Ok(r) => {
            let json: Value = r.json().expect("Failed to parse json");
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
                        Cell::new(released.to_string()).fg(released_color),
                        Cell::new(archived.to_string()).fg(archived_color),
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
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn new_version(global: &Global, project_id: &str, version_name: &str) {
    let url: String = generate_url(&global.domain, "version", None);
    let payload: Value = json!({
      "name": version_name,
      "projectId": project_id.parse::<i32>().unwrap()
    });
    match post_request(&url, &payload, &global.b64auth()) {
        Ok(_) => print_output(&format!("Version created: {version_name}")),
        Err(e) => handle_error_and_exit(&format!("Failed to create version {version_name}: {e}")),
    }
}

pub fn set_feature_state(
    global: &Global,
    project_key: &str,
    project_feature_key: &str,
    project_feature_state: &str,
) {
    let url: String = generate_url(
        &global.domain,
        "project",
        Some(&format!("/{project_key}/features/{project_feature_key}")),
    );
    let payload: Value = json!({ "state": project_feature_state });
    match put_request(&url, &payload, &global.b64auth()) {
        Ok(_) => print_output(&format!(
            "Feature {project_feature_key} set to {project_feature_state} on project {project_key}"
        )),
        Err(e) => handle_error_and_exit(
            &format!(
                "Impossible to set feature {project_feature_key} to {project_feature_state} on project {project_feature_key}: {e}"
            ))
    }
}
