/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]

mod cli;

use std::io::stdout;

use clap::{ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};
use human_panic::setup_panic;
use pretty_env_logger::init as pretty_env_logger_init;

use jira_cli::Global;

use crate::cli::{check_version, group, issue, labels, license, project, user};

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_owned(), &mut stdout());
}

fn get_one_match_string(matches: &ArgMatches, name: &str) -> String {
    matches.get_one::<String>(name).unwrap().to_string()
}

fn main() {
    pretty_env_logger_init();
    setup_panic!();
    let matches = cli::build_cli().get_matches();
    let global: Global = Global {
        domain: get_one_match_string(&matches, "domain"),
        user: get_one_match_string(&matches, "user"),
        token: get_one_match_string(&matches, "token"),
    };

    match matches.subcommand() {
        Some(("check_version", _)) => {
            check_version::logic_commands();
        }
        Some(("generate", args)) => {
            let shell = args.get_one::<Shell>("shell").unwrap();
            let mut cmd = cli::build_cli();
            print_completions(*shell, &mut cmd);
        }
        Some(("group", args)) => group::logic_commands(&global, args),
        Some(("issue", args)) => issue::logic_commands(&global, args),
        Some(("labels", args)) => labels::logic_commands(&global, args),
        Some(("license", _)) => license::logic_commands(),
        Some(("project", args)) => project::logic_commands(&global, args),
        Some(("user", args)) => user::logic_commands(&global, args),
        _ => unreachable!(),
    }
}
