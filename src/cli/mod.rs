/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

pub mod check_version;
pub mod group;
pub mod issue;
pub mod labels;
pub mod license;
pub mod project;
pub mod user;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, Arg, Command,
};
use clap_complete::Shell;

fn generate() -> Command {
    Command::new("generate")
        .about("Generate autocompletion script for your shell")
        .visible_aliases(["g", "gen"])
        .ignore_errors(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("shell")
                .value_parser(value_parser!(Shell))
                .required(true),
        )
}

#[allow(clippy::module_name_repetitions)]
pub fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .help("Domain")
                .env("JIRA_DOMAIN")
                .required(true),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .help("User")
                .env("JIRA_USER")
                .required(true),
        )
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .help("TOKEN - API KEY")
                .env("JIRA_TOKEN")
                .required(true),
        )
        .subcommand(generate())
        .subcommand(check_version::cli_commands())
        .subcommand(group::cli_commands())
        .subcommand(issue::cli_commands())
        .subcommand(labels::cli_commands())
        .subcommand(license::cli_commands())
        .subcommand(project::cli_commands())
        .subcommand(user::cli_commands())
}
