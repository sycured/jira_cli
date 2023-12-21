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

pub fn cli_commands() -> Command {
    Command::new("project")
        .about("Manage projects")
        .visible_aliases(["p", "proj"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(cli_commands::get_id())
        .subcommand(cli_commands::add_component())
        .subcommand(cli_commands::create())
        .subcommand(cli_commands::delete_component())
        .subcommand(cli_commands::delete_project())
        .subcommand(cli_commands::list_components())
        .subcommand(cli_commands::list_features())
        .subcommand(cli_commands::list_projects())
        .subcommand(cli_commands::list_versions())
        .subcommand(cli_commands::new_version())
        .subcommand(cli_commands::set_feature_state())
}

pub fn logic_commands(global: &Global, args: &ArgMatches) {
    match args.subcommand() {
        Some(("add_component", args)) => cli_logic::add_component(global, args),
        Some(("create", args)) => cli_logic::create(global, args),
        Some(("delete_component", args)) => cli_logic::delete_component(global, args),
        Some(("delete_project", args)) => cli_logic::delete_project(global, args),
        Some(("get_id", args)) => cli_logic::get_id(global, args),
        Some(("list_components", args)) => cli_logic::list_components(global, args),
        Some(("list_features", args)) => cli_logic::list_features(global, args),
        Some(("list_projects", args)) => cli_logic::list_projects(global, args),
        Some(("list_versions", args)) => cli_logic::list_versions(global, args),
        Some(("new_version", args)) => cli_logic::new_version(global, args),
        Some(("set_feature_state", args)) => cli_logic::set_feature_state(global, args),
        _ => unreachable!(),
    }
}
