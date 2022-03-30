use clap::{ArgMatches, Command};
use std::collections::HashMap;
pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command<'static> {
    return Command::new("issue")
        .about("Manage issues")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::add_version())
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::list_priorities())
        .subcommand(cli_commands::list_types());
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    match args.subcommand() {
        Some(("add_version", args)) => {
            cli_logic::add_version(&global, args);
        }
        Some(("create", args)) => {
            cli_logic::create(&global, args);
        }
        Some(("list_priorities", _)) => {
            cli_logic::list_priorities(&global);
        }
        Some(("list_types", args)) => {
            cli_logic::list_types(&global, args);
        }
        _ => {
            unreachable!();
        }
    }
}
