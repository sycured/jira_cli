/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::{collections::HashMap, io::stdout};

use clap::Command;
use clap_complete::{generate, Generator, Shell};
use human_panic::setup_panic;

mod check_version;
mod cli;
mod group;
mod issue;
mod labels;
mod license;
mod project;
mod urls;
mod user;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_owned(), &mut stdout());
}

fn main() {
    setup_panic!();
    let matches = cli::build_cli().get_matches();
    let global: HashMap<&str, &str> = HashMap::from([
        (
            "domain",
            matches.get_one::<String>("domain").unwrap().as_str(),
        ),
        (
            "token",
            matches.get_one::<String>("token").unwrap().as_str(),
        ),
        ("user", matches.get_one::<String>("user").unwrap().as_str()),
    ]);

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
