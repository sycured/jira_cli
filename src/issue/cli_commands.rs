use clap::{Arg, Command};

pub fn add_version() -> Command<'static> {
    return Command::new("add_version")
        .about("Add version to Jira issue")
        .arg_required_else_help(true)
        .arg(Arg::new("issue_key").help("Issue key").required(true))
        .arg(
            Arg::new("version_name")
                .help("Version name")
                .env("JIRA_VERSION_NAME")
                .required(true),
        );
}

pub fn create_issue() -> Command<'static> {
    return Command::new("create_issue")
        .about("Create Jira issue")
        .arg_required_else_help(true)
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .env("JIRA_PROJECT_KEY")
                .required(true),
        )
        .arg(Arg::new("issue_type").help("Issue type").required(true))
        .arg(Arg::new("issue_summary").help("Summary").required(true))
        .arg(
            Arg::new("issue_description")
                .help("Description")
                .required(true),
        )
        .arg(
            Arg::new("reporter_account_id")
                .help("Reporter account id")
                .required(true),
        )
        .arg(Arg::new("issue_priority").help("Issue priority"));
}

pub fn list_issue_priorities() -> Command<'static> {
    return Command::new("list_issue_priorities").about("Get Jira issue priorities");
}

pub fn list_issue_types() -> Command<'static> {
    return Command::new("get_issue_types")
        .about("Get issue types from Jira project")
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}
