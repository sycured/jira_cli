/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{Arg, ArgMatches, Command};

use jira_cli::labels::list;

use crate::Global;

pub fn cli_commands() -> Command {
    Command::new("labels")
        .about("List available labels for the Global label field")
        .visible_alias("l")
        .arg(
            Arg::new("max_results")
                .value_name("max_results")
                .help("The maximum number of items to return")
                .default_value("1000"),
        )
        .arg(
            Arg::new("start_at")
                .value_name("start_at")
                .help("The index of the first item to return")
                .default_value("0"),
        )
}

pub fn logic_commands(global: &Global, args: &ArgMatches) {
    list(
        global,
        args.get_one::<String>("start_at")
            .unwrap_or(&String::new())
            .as_str(),
        args.get_one::<String>("max_results")
            .unwrap_or(&String::new())
            .as_str(),
    );
}
