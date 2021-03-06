use std::collections::HashMap;

use clap::{ArgMatches, Command};

pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command<'static> {
    return Command::new("project")
        .about("Manage projects")
        .visible_aliases(&["p", "proj"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::get_id())
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::delete_project())
        .subcommand(cli_commands::list_features())
        .subcommand(cli_commands::list_versions())
        .subcommand(cli_commands::new_version())
        .subcommand(cli_commands::set_feature_state());
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    match args.subcommand() {
        Some(("create", args)) => cli_logic::create(&global, args),
        Some(("delete_project", args)) => cli_logic::delete_project(&global, args),
        Some(("get_id", args)) => cli_logic::get_id(&global, args),
        Some(("list_features", args)) => cli_logic::list_features(&global, args),
        Some(("list_versions", args)) => cli_logic::list_versions(&global, args),
        Some(("new_version", args)) => cli_logic::new_version(&global, args),
        Some(("set_feature_state", args)) => cli_logic::set_feature_state(&global, args),
        _ => unreachable!(),
    }
}
