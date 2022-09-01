/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use clap::ArgMatches;

use super::functions;

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
        global,
        args.get_one::<String>("email").unwrap(),
        args.get_one::<String>("display_name").unwrap(),
    );
}

pub fn delete(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete(
        global,
        args.get_one::<String>("account_id").unwrap().as_str(),
    );
}

pub fn get_account_id(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_account_id(
        global,
        args.get_one::<String>("email_address").unwrap().as_str(),
    );
}

pub fn get_user_groups(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_user_groups(
        global,
        args.get_one::<String>("account_id").unwrap().as_str(),
    );
}
