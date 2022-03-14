use clap::{crate_authors, crate_name, crate_version, Arg, Command};

mod issue;
mod lib;
mod project;
mod user;

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .help("Domain")
                .env("JIRA_DOMAIN")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .help("User")
                .env("JIRA_USER")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .help("TOKEN - API KEY")
                .env("JIRA_TOKEN")
                .takes_value(true)
                .required(true),
        )
        .subcommand(issue::cli_add_version())
        .subcommand(project::cli_get_project_id())
        .subcommand(project::cli_create_version())
        .subcommand(project::cli_create_project())
        .subcommand(project::cli_delete_project())
        .subcommand(project::cli_set_project_feature_state())
        .subcommand(user::cli_get_account_id())
        .get_matches();

    match matches.subcommand() {
        Some(("get_project_id", args)) => {
            project::get_project_id(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("project_key").unwrap().to_string(),
            );
        }
        Some(("create_version", args)) => {
            project::create_version(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("project_id").unwrap().to_string(),
                args.value_of("version_name").unwrap().to_string(),
            );
        }
        Some(("add_version", args)) => {
            issue::add_version(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("version_name").unwrap().to_string(),
                args.value_of("issue_key").unwrap().to_string(),
            );
        }
        Some(("delete_project", args)) => {
            project::delete_project(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("project_key").unwrap().to_string(),
            );
        }
        Some(("create_project", args)) => {
            project::create_project(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("project_name").unwrap().to_string(),
                args.value_of("project_key").unwrap().to_string(),
                args.value_of("jira_project_leadaccountid")
                    .unwrap()
                    .to_string(),
                args.value_of("project_type").unwrap().to_string(),
                args.value_of("project_template").unwrap().to_string(),
            );
        }
        Some(("get_account_id", args)) => {
            user::get_account_id(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("email_address").unwrap().to_string(),
            );
        }
        Some(("set_project_feature_state", args)) => {
            project::set_project_feature_state(
                matches.value_of("domain").unwrap().to_string(),
                matches.value_of("user").unwrap().to_string(),
                matches.value_of("token").unwrap().to_string(),
                args.value_of("project_key").unwrap().to_string(),
                args.value_of("feature_key").unwrap().to_string(),
                args.value_of("feature_state").unwrap().to_string(),
            );
        }
        _ => {
            unreachable!();
        }
    }
}
