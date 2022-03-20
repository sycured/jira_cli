use crate::lib::{delete_request, get_request, post_request, put_request};
use clap::{Arg, Command};
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
    post_request(&url, payload, jira_user, jira_token, &success_message);
}

pub fn cli_create_project() -> Command<'static> {
    Command::new("create_project")
        .about("Create project")
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_name")
                .help("Project name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("jira_project_leadaccountid")
                .help("Project lead (account id)")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("project_type")
                .help("Project type")
                .default_value("software")
                .possible_values(["business", "software", "service_desk"]),
        )
        .arg(
            Arg::new("project_template")
                .help("Project template")
                .default_value("com.pyxis.greenhopper.jira:gh-simplified-agility-kanban")
                .possible_values(["com.atlassian.jira-core-project-templates:jira-core-simplified-content-management", "com.atlassian.jira-core-project-templates:jira-core-simplified-document-approval", "com.atlassian.jira-core-project-templates:jira-core-simplified-lead-tracking", "com.atlassian.jira-core-project-templates:jira-core-simplified-process-control", "com.atlassian.jira-core-project-templates:jira-core-simplified-procurement", "com.atlassian.jira-core-project-templates:jira-core-simplified-project-management", "com.atlassian.jira-core-project-templates:jira-core-simplified-recruitment", "com.atlassian.jira-core-project-templates:jira-core-simplified-task-tracking", "com.atlassian.servicedesk:simplified-it-service-management", "com.atlassian.servicedesk:simplified-general-service-desk", "com.atlassian.servicedesk:simplified-internal-service-desk", "com.atlassian.servicedesk:simplified-external-service-desk", "com.atlassian.servicedesk:simplified-hr-service-desk", "com.atlassian.servicedesk:simplified-facilities-service-desk", "com.atlassian.servicedesk:simplified-legal-service-desk", "com.pyxis.greenhopper.jira:gh-simplified-agility-kanban", "com.pyxis.greenhopper.jira:gh-simplified-agility-scrum", "com.pyxis.greenhopper.jira:gh-simplified-basic", "com.pyxis.greenhopper.jira:gh-simplified-kanban-classic", "com.pyxis.greenhopper.jira:gh-simplified-scrum-classic"]),
        )
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
    post_request(&url, payload, jira_user, jira_token, &success_message);
}

pub fn cli_create_version() -> Command<'static> {
    Command::new("create_version")
        .about("Create version")
        .arg_required_else_help(true)
        .arg(
            Arg::new("version_name")
                .help("Version name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("project_id")
                .help("Project id (use get_project_id subcommand to get it")
                .env("JIRA_PROJECT_ID")
                .takes_value(true)
                .required(true),
        )
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

pub fn cli_delete_project() -> Command<'static> {
    Command::new("delete_project")
        .about("Delete project")
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .takes_value(true),
        )
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

pub fn cli_get_project_id() -> Command<'static> {
    Command::new("get_project_id")
        .about("Get project id")
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .takes_value(true),
        )
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

pub fn cli_set_project_feature_state() -> Command<'static> {
    Command::new("set_project_feature_state")
        .about("Set project feature state")
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("feature_key")
                .help("Feature key")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("feature_state")
                .help("Feature state")
                .takes_value(true)
                .possible_values(["ENABLED", "DISABLED", "COMING_SOON"])
                .required(true),
        )
}
