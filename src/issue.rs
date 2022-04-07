use std::collections::HashMap;

use clap::{ArgMatches, Command};

pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command<'static> {
    return Command::new("issue")
        .about("Manage issues")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::add_label())
        .subcommand(cli_commands::add_version())
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::list_priorities())
        .subcommand(cli_commands::list_types())
        .subcommand(cli_commands::remove_label())
        .subcommand(cli_commands::remove_version())
        .subcommand(cli_commands::show_fixversions());
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    match args.subcommand() {
        Some(("add_label", args)) => cli_logic::add_label(&global, args),
        Some(("add_version", args)) => cli_logic::add_version(&global, args),
        Some(("create", args)) => cli_logic::create(&global, args),
        Some(("list_priorities", _)) => cli_logic::list_priorities(&global),
        Some(("list_types", args)) => cli_logic::list_types(&global, args),
        Some(("remove_label", args)) => cli_logic::remove_label(&global, args),
        Some(("remove_version", args)) => cli_logic::remove_version(&global, args),
        Some(("show_fixversions", args)) => cli_logic::show_fixversions(&global, args),
        _ => unreachable!(),
    }
}
