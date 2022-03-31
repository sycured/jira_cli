use std::collections::HashMap;

use clap::{ArgMatches, Command};

pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command<'static> {
    return Command::new("user")
        .about("Manage users")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::get_account_id());
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    match args.subcommand() {
        Some(("get_account_id", args)) => cli_logic::get_account_id(&global, args),
        _ => unreachable!(),
    }
}
