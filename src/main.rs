/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]

mod cli;

use std::io::stdout;

use clap::Command;
use clap_complete::{generate, Generator, Shell};
use human_panic::setup_panic;
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc;

use jira_cli::Global;

use crate::cli::{check_version, group, issue, labels, license, project, user};

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_owned(), &mut stdout());
}

fn main() {
    setup_panic!();
    let matches = cli::build_cli().get_matches();
    let global: Global = Global {
        domain: matches.get_one::<String>("domain").unwrap().to_string(),
        user: matches.get_one::<String>("user").unwrap().to_string(),
        token: matches.get_one::<String>("token").unwrap().to_string(),
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
