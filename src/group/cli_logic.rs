/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use clap::ArgMatches;
use rayon::prelude::*;

use super::functions;

pub fn add_user(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let account_ids: Vec<&String> = args
        .get_many::<String>("account_id")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    account_ids.par_iter().for_each(|account_id| {
        functions::add_user(
            global,
            account_id.as_str(),
            args.get_one::<String>("group_id").unwrap().as_str(),
        )
    });
}

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let account_ids: Vec<&String> = args
        .get_many::<String>("name")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    account_ids
        .par_iter()
        .for_each(|name| functions::create(global, name.as_str()));
}

pub fn delete(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete(global, args.get_one::<String>("group_id").unwrap().as_str());
}

pub fn find(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::find(global, args.get_one::<String>("query").unwrap().as_str());
}

pub fn list_groups(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_groups(
        global,
        args.get_one::<String>("start_at").unwrap().as_str(),
        args.get_one::<String>("max_results").unwrap().as_str(),
    );
}

pub fn list_users(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_users(
        global,
        args.get_one::<String>("group_id").unwrap().as_str(),
        args.get_one::<String>("include_inactive")
            .unwrap()
            .parse()
            .unwrap(),
        args.get_one::<String>("start_at").unwrap().as_str(),
        args.get_one::<String>("max_results").unwrap().as_str(),
    );
}

pub fn remove_user(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let account_ids: Vec<&String> = args
        .get_many::<String>("account_id")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    account_ids.par_iter().for_each(|account_id| {
        functions::remove_user(
            global,
            account_id.as_str(),
            args.get_one::<String>("group_id").unwrap().as_str(),
        )
    });
}
