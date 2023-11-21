/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;

use jira_cli::{get_single_arg, user, Global};

pub fn create(global: &Global, args: &ArgMatches) {
    let (email, display_name) = (
        get_single_arg(args, "email"),
        get_single_arg(args, "display_name"),
    );
    user::create(global, email, display_name);
}

pub fn delete(global: &Global, args: &ArgMatches) {
    let account_id = get_single_arg(args, "account_id");
    user::delete(global, account_id);
}

pub fn get_account_id(global: &Global, args: &ArgMatches) {
    let email_address = get_single_arg(args, "email_address");
    user::get_account_id(global, email_address);
}

pub fn get_user_groups(global: &Global, args: &ArgMatches) {
    let account_id = get_single_arg(args, "account_id");
    user::get_user_groups(global, account_id);
}
