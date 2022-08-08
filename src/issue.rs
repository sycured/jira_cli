use std::collections::HashMap;

use clap::{ArgMatches, Command};

pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command<'static> {
    return Command::new("issue")
        .about("Manage issues")
        .visible_alias("i")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::add_label())
        .subcommand(cli_commands::add_version())
        .subcommand(cli_commands::add_vote())
        .subcommand(cli_commands::assign())
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::create_link_type())
        .subcommand(cli_commands::delete())
        .subcommand(cli_commands::delete_link_type())
        .subcommand(cli_commands::get_link_type())
        .subcommand(cli_commands::get_transitions())
        .subcommand(cli_commands::list_link_types())
        .subcommand(cli_commands::list_priorities())
        .subcommand(cli_commands::list_votes())
        .subcommand(cli_commands::list_types())
        .subcommand(cli_commands::remove_label())
        .subcommand(cli_commands::remove_version())
        .subcommand(cli_commands::remove_vote())
        .subcommand(cli_commands::show_fixversions())
        .subcommand(cli_commands::transition())
        .subcommand(cli_commands::unassign())
        .subcommand(cli_commands::update_link_type());
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    match args.subcommand() {
        Some(("add_label", args)) => cli_logic::add_label(&global, args),
        Some(("add_version", args)) => cli_logic::add_version(&global, args),
        Some(("add_vote", args)) => cli_logic::add_vote(&global, args),
        Some(("assign", args)) => cli_logic::assign(&global, args),
        Some(("create", args)) => cli_logic::create(&global, args),
        Some(("create_link_type", args)) => cli_logic::create_link_type(&global, args),
        Some(("delete", args)) => cli_logic::delete(&global, args),
        Some(("delete_link_type", args)) => cli_logic::delete_link_type(&global, args),
        Some(("get_link_type", args)) => cli_logic::get_link_type(&global, args),
        Some(("get_transitions", args)) => cli_logic::get_transitions(&global, args),
        Some(("list_link_types", _)) => cli_logic::list_link_types(&global),
        Some(("list_priorities", _)) => cli_logic::list_priorities(&global),
        Some(("list_votes", args)) => cli_logic::list_votes(&global, args),
        Some(("list_types", args)) => cli_logic::list_types(&global, args),
        Some(("remove_label", args)) => cli_logic::remove_label(&global, args),
        Some(("remove_version", args)) => cli_logic::remove_version(&global, args),
        Some(("remove_vote", args)) => cli_logic::remove_vote(&global, args),
        Some(("show_fixversions", args)) => cli_logic::show_fixversions(&global, args),
        Some(("transition", args)) => cli_logic::transition(&global, args),
        Some(("unassign", args)) => cli_logic::unassign(&global, args),
        Some(("update_link_type", args)) => cli_logic::update_link_type(&global, args),
        _ => unreachable!(),
    }
}
