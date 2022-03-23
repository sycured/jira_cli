use crate::lib::{delete_request, get_request, post_request, put_request};
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, CellAlignment, ContentArrangement,
    Table,
};
use dialoguer::Confirm;
use serde_json::Value;
use ureq::{json, Response};

pub fn create_project(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_name: &str,
    jira_project_key: &str,
    jira_project_leadaccountid: &str,
    jira_project_type: &str,
    jira_project_template: &str,
) {
    let url: String = format!("https://{domain}/rest/api/3/project", domain = jira_domain);
    let payload: Value = json!({
        "name": jira_project_name,
        "key": jira_project_key,
        "leadAccountId": jira_project_leadaccountid,
        "projectTypeKey": jira_project_type,
        "projectTemplateKey": jira_project_template,
        "assigneeType": "UNASSIGNED"
    });
    let success_message: String = format!("Project {} created", jira_project_key);
    post_request(
        &url,
        payload,
        jira_user,
        jira_token,
        &success_message,
        false,
    );
}

pub fn create_version(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_id: &str,
    version_name: &str,
) {
    let url: String = format!("https://{domain}/rest/api/3/version", domain = jira_domain);
    let payload: Value = json!({
      "name": version_name,
      "projectId": jira_project_id.parse::<i32>().unwrap()
    });
    let success_message: String = format!("Version created: {}", version_name);
    post_request(
        &url,
        payload,
        jira_user,
        jira_token,
        &success_message,
        false,
    );
}

pub fn delete_project(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_key: &str,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/project/{key}",
        domain = jira_domain,
        key = jira_project_key
    );
    if Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to delete the project key: {}?",
            jira_project_key
        ))
        .interact()
        .unwrap()
    {
        let success_message: String = format!("Project {} deleted", jira_project_key);
        delete_request(&url, jira_user, jira_token, &success_message);
    } else {
        println!("Project {} not deleted.", jira_project_key);
    }
}

pub fn get_project_id(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_key: &str,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/project/{key}",
        domain = jira_domain,
        key = jira_project_key
    );
    let resp: Response = get_request(&url, jira_user, jira_token);
    let json: Value = resp.into_json().unwrap();
    println!("{}", json["id"].as_str().unwrap().parse::<i32>().unwrap());
}

pub fn list_project_features(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_key: &str,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/project/{key}/features",
        domain = jira_domain,
        key = jira_project_key
    );
    let resp: Response = get_request(&url, jira_user, jira_token);
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

pub fn set_project_feature_state(
    jira_domain: &str,
    jira_user: &str,
    jira_token: &str,
    jira_project_key: &str,
    jira_project_feature_key: &str,
    jira_project_feature_state: &str,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/project/{projectkey}/features/{featurekey}",
        domain = jira_domain,
        projectkey = jira_project_key,
        featurekey = jira_project_feature_key
    );
    let payload: Value = json!({ "state": jira_project_feature_state });
    let success_message: String = format!(
        "Feature {feature_key} set to {feature_state} on project {project_key}",
        feature_key = jira_project_feature_key,
        feature_state = jira_project_feature_state,
        project_key = jira_project_key
    );
    put_request(&url, payload, jira_user, jira_token, &success_message);
}
