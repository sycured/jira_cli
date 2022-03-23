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
            issue::cli_logic::add_version(&matches, args);
        }
        Some(("create_issue", args)) => {
            issue::cli_logic::create_issue(&matches, args);
        }
        Some(("create_project", args)) => {
            project::cli_logic::create_project(&matches, args);
        }
        Some(("create_version", args)) => {
            project::cli_logic::create_version(&matches, args);
        }
        Some(("delete_project", args)) => {
            project::cli_logic::delete_project(&matches, args);
        }
        Some(("get_account_id", args)) => {
            user::cli_logic::get_account_id(&matches, args);
        }
        Some(("get_project_id", args)) => {
            project::cli_logic::get_project_id(&matches, args);
        }
        Some(("list_issue_priorities", _)) => {
            issue::cli_logic::list_issue_priorities(&matches);
        }
        Some(("list_issue_types", args)) => {
            issue::cli_logic::list_issue_types(&matches, args);
        }
        Some(("list_project_features", args)) => {
            project::cli_logic::list_project_features(&matches, args);
        }
        Some(("set_project_feature_state", args)) => {
            project::cli_logic::set_project_feature_state(&matches, args);
        }
        _ => {
            unreachable!();
        }
    }
}
