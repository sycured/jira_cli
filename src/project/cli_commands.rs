use clap::{Arg, Command};

pub fn create_project() -> Command<'static> {
    return Command::new("create_project")
        .about("Create project")
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
                .possible_values(["business", "software", "service_desk"]),
        )
        .arg(
            Arg::new("project_template")
                .help("Project template")
                .default_value("com.pyxis.greenhopper.jira:gh-simplified-agility-kanban")
                .possible_values(["com.atlassian.jira-core-project-templates:jira-core-simplified-content-management", "com.atlassian.jira-core-project-templates:jira-core-simplified-document-approval", "com.atlassian.jira-core-project-templates:jira-core-simplified-lead-tracking", "com.atlassian.jira-core-project-templates:jira-core-simplified-process-control", "com.atlassian.jira-core-project-templates:jira-core-simplified-procurement", "com.atlassian.jira-core-project-templates:jira-core-simplified-project-management", "com.atlassian.jira-core-project-templates:jira-core-simplified-recruitment", "com.atlassian.jira-core-project-templates:jira-core-simplified-task-tracking", "com.atlassian.servicedesk:simplified-it-service-management", "com.atlassian.servicedesk:simplified-general-service-desk", "com.atlassian.servicedesk:simplified-internal-service-desk", "com.atlassian.servicedesk:simplified-external-service-desk", "com.atlassian.servicedesk:simplified-hr-service-desk", "com.atlassian.servicedesk:simplified-facilities-service-desk", "com.atlassian.servicedesk:simplified-legal-service-desk", "com.pyxis.greenhopper.jira:gh-simplified-agility-kanban", "com.pyxis.greenhopper.jira:gh-simplified-agility-scrum", "com.pyxis.greenhopper.jira:gh-simplified-basic", "com.pyxis.greenhopper.jira:gh-simplified-kanban-classic", "com.pyxis.greenhopper.jira:gh-simplified-scrum-classic"]),
        );
}

pub fn create_version() -> Command<'static> {
    return Command::new("create_version")
        .about("Create version")
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

pub fn delete_project() -> Command<'static> {
    return Command::new("delete_project")
        .aliases(&["destroy", "destroy_project"])
        .about("Delete project")
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}

pub fn get_project_id() -> Command<'static> {
    return Command::new("get_project_id")
        .about("Get project id")
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}

pub fn set_project_feature_state() -> Command<'static> {
    return Command::new("set_project_feature_state")
        .about("Set project feature state")
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true))
        .arg(Arg::new("feature_key").help("Feature key").required(true))
        .arg(
            Arg::new("feature_state")
                .help("Feature state")
                .possible_values(["ENABLED", "DISABLED", "COMING_SOON"])
                .required(true),
        );
}
