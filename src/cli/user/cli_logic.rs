/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;

use jira_cli::{user, Global};

pub fn create(global: &Global, args: &ArgMatches) {
    user::create(
        global,
        args.get_one::<String>("email").unwrap(),
        args.get_one::<String>("display_name").unwrap(),
    );
}

pub fn delete(global: &Global, args: &ArgMatches) {
    user::delete(
        global,
        args.get_one::<String>("account_id").unwrap().as_str(),
    );
}

pub fn get_account_id(global: &Global, args: &ArgMatches) {
    user::get_account_id(
        global,
        args.get_one::<String>("email_address").unwrap().as_str(),
    );
}

pub fn get_user_groups(global: &Global, args: &ArgMatches) {
    user::get_user_groups(
        global,
        args.get_one::<String>("account_id").unwrap().as_str(),
    );
}
