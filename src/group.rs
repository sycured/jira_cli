/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use clap::{ArgMatches, Command};

pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command<'static> {
    return Command::new("group")
        .about("Manage groups of users")
        .visible_alias("gr")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::add_user())
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::delete())
        .subcommand(cli_commands::find())
        .subcommand(cli_commands::list_groups())
        .subcommand(cli_commands::list_users())
        .subcommand(cli_commands::remove_user());
}

pub fn logic_commands(global: HashMap<&str, &str>, args: &ArgMatches) {
    match args.subcommand() {
        Some(("add_user", args)) => cli_logic::add_user(&global, args),
        Some(("create", args)) => cli_logic::create(&global, args),
        Some(("delete", args)) => cli_logic::delete(&global, args),
        Some(("find", args)) => cli_logic::find(&global, args),
        Some(("list_groups", args)) => cli_logic::list_groups(&global, args),
        Some(("list_users", args)) => cli_logic::list_users(&global, args),
        Some(("remove_user", args)) => cli_logic::remove_user(&global, args),
        _ => unreachable!(),
    }
}
