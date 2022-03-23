use crate::{issue, project, user};
use clap::{crate_authors, crate_name, crate_version, Arg, Command};
use clap_complete::Shell;

fn generate() -> Command<'static> {
    return Command::new("generate")
        .about("Generate autocompletion script for your shell")
        .arg_required_else_help(true)
        .arg(
            Arg::new("shell")
                .possible_values(Shell::possible_values())
                .required(true),
        );
}

pub fn build_cli() -> Command<'static> {
    return Command::new(crate_name!())
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
        .subcommand(generate())
        .subcommand(issue::cli_commands::add_version())
        .subcommand(issue::cli_commands::create_issue())
        .subcommand(issue::cli_commands::list_issue_priorities())
        .subcommand(issue::cli_commands::list_issue_types())
        .subcommand(project::cli_commands::get_project_id())
        .subcommand(project::cli_commands::create_version())
        .subcommand(project::cli_commands::create_project())
        .subcommand(project::cli_commands::delete_project())
        .subcommand(project::cli_commands::list_project_features())
        .subcommand(project::cli_commands::set_project_feature_state())
        .subcommand(user::cli_commands::get_account_id());
}
