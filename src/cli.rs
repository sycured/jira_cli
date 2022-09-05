/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{
    Arg, Command, crate_authors, crate_description, crate_name, crate_version, value_parser,
};
use clap_complete::Shell;

use crate::{check_version, group, issue, labels, license, project, user};

fn generate() -> Command<'static> {
    return Command::new("generate")
        .about("Generate autocompletion script for your shell")
        .visible_aliases(&["g", "gen"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("shell")
                .value_parser(value_parser!(Shell))
                .required(true),
        );
}

pub fn build_cli() -> Command<'static> {
    return Command::new(crate_name!())
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
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .help("User")
                .env("JIRA_USER")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .help("TOKEN - API KEY")
                .env("JIRA_TOKEN")
                .takes_value(true)
                .required(true),
        )
        .subcommand(generate())
        .subcommand(check_version::cli_commands())
        .subcommand(group::cli_commands())
        .subcommand(issue::cli_commands())
        .subcommand(labels::cli_commands())
        .subcommand(license::cli_commands())
        .subcommand(project::cli_commands())
        .subcommand(user::cli_commands());
}
