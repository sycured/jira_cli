/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{ArgMatches, Command};

use crate::Global;

pub mod cli_commands;
pub mod cli_logic;
pub mod functions;

pub fn cli_commands() -> Command {
    Command::new("user")
        .about("Manage users")
        .visible_aliases(["u", "usr"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::delete())
        .subcommand(cli_commands::get_account_id())
        .subcommand(cli_commands::get_user_groups())
}

pub fn logic_commands(global: &Global, args: &ArgMatches) {
    match args.subcommand() {
        Some(("create", args)) => cli_logic::create(global, args),
        Some(("delete", args)) => cli_logic::delete(global, args),
        Some(("get_account_id", args)) => cli_logic::get_account_id(global, args),
        Some(("get_user_groups", args)) => cli_logic::get_user_groups(global, args),
        _ => unreachable!(),
    }
}
