/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{Arg, Command};

pub fn add_user() -> Command {
    Command::new("add_user")
        .about("Adds a user to a group")
        .visible_aliases(&["au", "add-user"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("account_id")
                .help("The account ID of the user")
                .value_delimiter(',')
                .required(true),
        )
        .arg(
            Arg::new("group_id")
                .help("The ID of the group")
                .env("JIRA_GROUP_ID")
                .required(true),
        )
}

pub fn create() -> Command {
    Command::new("create")
        .about("Create a group")
        .visible_alias("c")
        .arg_required_else_help(true)
        .arg(
            Arg::new("name")
                .help("The name of the group")
                .value_delimiter(',')
                .required(true),
        )
}

pub fn delete() -> Command {
    Command::new("delete")
        .about("Delete a group")
        .visible_aliases(&["d", "del", "rm"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("group_id")
                .help("The ID of the group")
                .env("JIRA_GROUP_ID")
                .required(true),
        )
}

pub fn find() -> Command {
    Command::new("find")
        .about("Returns a list of groups whose names contain a query string")
        .visible_aliases(&["f", "fd"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("query")
                .help("The string to find in group names")
                .required(true),
        )
}

pub fn list_groups() -> Command {
    Command::new("list_groups")
        .about("Returns a paginated list of groups")
        .visible_aliases(&["lg", "list-groups"])
        .arg(
            Arg::new("start_at")
                .help("The index of the first item to return in a page of results (page offset)")
                .default_value("0"),
        )
        .arg(
            Arg::new("max_results")
                .help("The maximum number of items to return per page")
                .default_value("50"),
        )
}

pub fn list_users() -> Command {
    Command::new("list_users")
        .about("List users in a group")
        .visible_aliases(&["lu", "list-users"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("group_id")
                .help("The ID of the group")
                .env("JIRA_GROUP_ID")
                .required(true),
        )
        .arg(
            Arg::new("include_inactive")
                .help("Include inactive users")
                .value_parser(["true", "false"])
                .default_value("false"),
        )
        .arg(
            Arg::new("start_at")
                .help("The index of the first item to return in a page of results (page offset)")
                .default_value("0"),
        )
        .arg(
            Arg::new("max_results")
                .help("The maximum number of items to return per page")
                .default_value("50"),
        )
}

pub fn remove_user() -> Command {
    Command::new("remove_user")
        .about("Remove a user from a group")
        .visible_aliases(&["ru", "remove-user"])
        .arg_required_else_help(true)
        .arg(
            Arg::new("account_id")
                .help("The ID of the account")
                .value_delimiter(',')
                .required(true),
        )
        .arg(
            Arg::new("group_id")
                .help("The ID of the group")
                .env("JIRA_GROUP_ID")
                .required(true),
        )
}
