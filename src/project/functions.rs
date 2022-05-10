use std::collections::HashMap;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, ContentArrangement,
    Table,
};
use dialoguer::Confirm;
use itertools::Itertools;
use serde_json::Value;
use ureq::{json, Response};

use crate::{
    lib::{delete_request, get_request, post_request, put_request},
    urls::URLS,
};

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
    let success_message: String = format!("Project {} created", project_key);
    post_request(
        &url,
        payload,
        global["user"],
        global["token"],
        &success_message,
        false,
    );
}

pub fn delete_project(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}",
        global["domain"], URLS["project"], project_key
    );
    if Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to delete the project key: {}?",
            project_key
        ))
        .interact()
        .unwrap()
    {
        let success_message: String = format!("Project {} deleted", project_key);
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
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    println!("{}", json["id"].as_str().unwrap().parse::<i32>().unwrap());
}

pub fn list_features(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/features",
        global["domain"], URLS["project"], project_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    let mut table = Table::new();
    table
        .set_header(vec!["Key", "Description", "State", "Locked"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table
        .get_column_mut(0)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .get_column_mut(2)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .get_column_mut(3)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    json["features"].as_array().unwrap().iter().for_each(|x| {
        let state: &str = x["state"].as_str().unwrap();
        let key: &str = x["feature"].as_str().unwrap();
        let description: &str = x["localisedDescription"].as_str().unwrap();
        let locked: bool = x["toggleLocked"].as_bool().unwrap();
        let locked_color = if locked {
            comfy_table::Color::Red
        } else {
            comfy_table::Color::Green
        };
        table.add_row(vec![
            Cell::new(key),
            Cell::new(description),
            Cell::new(state),
            Cell::new(&locked.to_string()).fg(locked_color),
        ]);
    });
    println!("{table}");
}

pub fn list_versions(global: &HashMap<&str, &str>, project_key: &str) {
    let url: String = format!(
        "https://{}{}/{}/versions",
        global["domain"], URLS["project"], project_key
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Description", "Released", "Archived", "Id"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table
        .get_column_mut(0)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .get_column_mut(1)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .get_column_mut(2)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .get_column_mut(3)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
    table
        .get_column_mut(4)
        .unwrap()
        .set_cell_alignment(CellAlignment::Center);
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
            table.add_row(vec![
                Cell::new(name),
                Cell::new(description),
                Cell::new(&released.to_string()).fg(released_color),
                Cell::new(&archived.to_string()).fg(archived_color),
                Cell::new(id),
            ]);
        });
    println!("{table}");
}

pub fn new_version(global: &HashMap<&str, &str>, project_id: &str, version_name: &str) {
    let url: String = format!("https://{}{}", global["domain"], URLS["version"]);
    let payload: Value = json!({
      "name": version_name,
      "projectId": project_id.parse::<i32>().unwrap()
    });
    let success_message: String = format!("Version created: {}", version_name);
    post_request(
        &url,
        payload,
        global["user"],
        global["token"],
        &success_message,
        false,
    );
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
        payload,
        global["user"],
        global["token"],
        &success_message,
    );
}
