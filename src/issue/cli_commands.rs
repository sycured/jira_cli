use clap::{Arg, Command};

pub fn add_label() -> Command<'static> {
    return Command::new("add_label")
        .about("Add a label to an issue")
        .arg_required_else_help(true)
        .arg(
            Arg::new("issue_key")
                .help("The key of the issue")
                .required(true),
        )
        .arg(
            Arg::new("label")
                .help("The label to add")
                .env("JIRA_LABEL")
                .required(true),
        );
}

pub fn add_version() -> Command<'static> {
    return Command::new("add_version")
        .about("Add a version to an issue")
        .arg_required_else_help(true)
        .arg(Arg::new("issue_key").help("Issue key").required(true))
        .arg(
            Arg::new("version_name")
                .help("Version name")
                .env("JIRA_VERSION_NAME")
                .required(true),
        );
}

pub fn create() -> Command<'static> {
    return Command::new("create")
        .about("Create an issue")
        .arg_required_else_help(true)
        .arg(
            Arg::new("issue_priority")
                .long("priority")
                .short('p')
                .takes_value(true)
                .help("Issue priority"),
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
        .arg(
            Arg::new("project_key")
                .help("Project key")
                .env("JIRA_PROJECT_KEY")
                .required(true),
        );
}

pub fn list_priorities() -> Command<'static> {
    return Command::new("list_priorities").about("List issue priorities");
}

pub fn list_types() -> Command<'static> {
    return Command::new("list_types")
        .about("List issue types for this project")
        .arg_required_else_help(true)
        .arg(Arg::new("project_key").help("Project key").required(true));
}

pub fn remove_label() -> Command<'static> {
    return Command::new("remove_label")
        .about("Remove a label from an issue")
        .arg_required_else_help(true)
        .arg(
            Arg::new("issue_key")
                .help("The key of the issue")
                .required(true),
        )
        .arg(
            Arg::new("label")
                .help("The label to add")
                .env("JIRA_LABEL")
                .required(true),
        );
}

pub fn remove_version() -> Command<'static> {
    return Command::new("remove_version")
        .about("Remove a version from an issue")
        .arg_required_else_help(true)
        .arg(Arg::new("issue_key").help("Issue key").required(true))
        .arg(
            Arg::new("version_name")
                .help("Version name")
                .env("JIRA_VERSION_NAME")
                .required(true),
        );
}

pub fn show_fixversions() -> Command<'static> {
    return Command::new("show_fixversions")
        .about("Show fix versions for this issue")
        .arg_required_else_help(true)
        .arg(Arg::new("issue_key").help("Issue key").required(true));
}
