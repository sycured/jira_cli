use clap::{Arg, Command};

pub fn create() -> Command<'static> {
    return Command::new("create")
        .about("Create project")
        .visible_alias("c")
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_name")
                .help("Project name")
                .required(true),
        )
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .required(true),
        )
        .arg(
            Arg::new("jira_project_leadaccountid")
                .help("Project lead (account id)")
                .required(true)
        )
        .arg(
            Arg::new("project_type")
                .help("Project type")
                .default_value("software")
                .value_parser(["business", "software", "service_desk"]),
        )
        .arg(
            Arg::new("project_template")
                .help("Project template")
                .default_value("com.pyxis.greenhopper.jira:gh-simplified-agility-kanban")
                .value_parser(["com.atlassian.jira-core-project-templates:jira-core-simplified-content-management", "com.atlassian.jira-core-project-templates:jira-core-simplified-document-approval", "com.atlassian.jira-core-project-templates:jira-core-simplified-lead-tracking", "com.atlassian.jira-core-project-templates:jira-core-simplified-process-control", "com.atlassian.jira-core-project-templates:jira-core-simplified-procurement", "com.atlassian.jira-core-project-templates:jira-core-simplified-project-management", "com.atlassian.jira-core-project-templates:jira-core-simplified-recruitment", "com.atlassian.jira-core-project-templates:jira-core-simplified-task-tracking", "com.atlassian.servicedesk:simplified-it-service-management", "com.atlassian.servicedesk:simplified-general-service-desk", "com.atlassian.servicedesk:simplified-internal-service-desk", "com.atlassian.servicedesk:simplified-external-service-desk", "com.atlassian.servicedesk:simplified-hr-service-desk", "com.atlassian.servicedesk:simplified-facilities-service-desk", "com.atlassian.servicedesk:simplified-legal-service-desk", "com.pyxis.greenhopper.jira:gh-simplified-agility-kanban", "com.pyxis.greenhopper.jira:gh-simplified-agility-scrum", "com.pyxis.greenhopper.jira:gh-simplified-basic", "com.pyxis.greenhopper.jira:gh-simplified-kanban-classic", "com.pyxis.greenhopper.jira:gh-simplified-scrum-classic"]),
        );
}

pub fn delete_project() -> Command<'static> {
    return Command::new("delete_project")
        .visible_aliases(&["destroy", "destroy_project"])
        .about("Delete project")
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}

pub fn get_id() -> Command<'static> {
    return Command::new("get_id")
        .about("Get project id")
        .visible_aliases(&["gi", "get-id"])
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}

pub fn list_features() -> Command<'static> {
    return Command::new("list_features")
        .about("List project features")
        .visible_aliases(&["lf", "list-features"])
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}

pub fn list_versions() -> Command<'static> {
    return Command::new("list_versions")
        .about("List project versions")
        .visible_aliases(&["lv", "list-versions"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .env("JIRA_PROJECT_ID")
                .required(true),
        );
}

pub fn new_version() -> Command<'static> {
    return Command::new("new_version")
        .about("Create version")
        .visible_aliases(&["nv", "new-version"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("version_name")
                .help("Version name")
                .env("JIRA_VERSION_NAME")
                .required(true),
        )
        .arg(
            Arg::new("project_id")
                .help("Project id (use get_project_id subcommand to get it")
                .env("JIRA_PROJECT_ID")
                .required(true),
        );
}

pub fn set_feature_state() -> Command<'static> {
    return Command::new("set_feature_state")
        .about("Set project feature state")
        .visible_aliases(&["sfs", "set-feature-state"])
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true))
        .arg(
            Arg::new("feature_key")
                .help("Feature key")
                .use_value_delimiter(true)
                .required(true),
        )
        .arg(
            Arg::new("feature_state")
                .help("Feature state")
                .value_parser(["ENABLED", "DISABLED", "COMING_SOON"])
                .required(true),
        );
}
