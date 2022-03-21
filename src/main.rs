use clap::Command;
use clap_complete::{generate, Generator, Shell};
use std::io::stdout;

mod cli;
mod issue;
mod lib;
mod project;
mod user;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_owned(), &mut stdout());
}

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("generate", args)) => {
            let shell = args.value_of_t::<Shell>("shell").unwrap();
            let mut cmd = cli::build_cli();
            print_completions(shell, &mut cmd);
        }
        Some(("add_version", args)) => {
            issue::add_version(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("version_name").unwrap(),
                args.value_of("issue_key").unwrap(),
            );
        }
        Some(("create_issue", args)) => {
            issue::create_issue(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("reporter_account_id").unwrap(),
                args.value_of("project_key").unwrap(),
                args.value_of("issue_type").unwrap(),
                args.value_of("issue_summary").unwrap(),
                args.value_of("issue_description").unwrap(),
                args.value_of("issue_priority").unwrap_or(""),
            );
        }
        Some(("create_project", args)) => {
            project::create_project(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("project_name").unwrap(),
                args.value_of("project_key").unwrap(),
                args.value_of("jira_project_leadaccountid").unwrap(),
                args.value_of("project_type").unwrap(),
                args.value_of("project_template").unwrap(),
            );
        }
        Some(("create_version", args)) => {
            project::create_version(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("project_id").unwrap(),
                args.value_of("version_name").unwrap(),
            );
        }
        Some(("delete_project", args)) => {
            project::delete_project(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("project_key").unwrap(),
            );
        }
        Some(("get_account_id", args)) => {
            user::get_account_id(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("email_address").unwrap(),
            );
        }
        Some(("get_issue_priorities", _)) => {
            issue::get_issue_priorities(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
            );
        }
        Some(("get_issue_types", args)) => {
            issue::get_issue_types(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("project_key").unwrap(),
            );
        }
        Some(("get_project_id", args)) => {
            project::get_project_id(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("project_key").unwrap(),
            );
        }
        Some(("set_project_feature_state", args)) => {
            project::set_project_feature_state(
                matches.value_of("domain").unwrap(),
                matches.value_of("user").unwrap(),
                matches.value_of("token").unwrap(),
                args.value_of("project_key").unwrap(),
                args.value_of("feature_key").unwrap(),
                args.value_of("feature_state").unwrap(),
            );
        }
        _ => {
            unreachable!();
        }
    }
}
