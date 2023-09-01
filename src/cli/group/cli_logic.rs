/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use rayon::prelude::*;

use jira_cli::{group, Global};

pub fn add_user(global: &Global, args: &ArgMatches) {
    let account_ids: Vec<&String> = args
        .get_many::<String>("account_id")
        .map(Iterator::collect)
        .unwrap();
    account_ids.par_iter().for_each(|account_id| {
        group::add_user(
            global,
            account_id.as_str(),
            args.get_one::<String>("group_id").unwrap().as_str(),
        );
    });
}

pub fn create(global: &Global, args: &ArgMatches) {
    let account_ids: Vec<&String> = args
        .get_many::<String>("name")
        .map(Iterator::collect)
        .unwrap();
    account_ids
        .par_iter()
        .for_each(|name| group::create(global, name.as_str()));
}

pub fn delete(global: &Global, args: &ArgMatches) {
    group::delete(global, args.get_one::<String>("group_id").unwrap().as_str());
}

pub fn find(global: &Global, args: &ArgMatches) {
    group::find(global, args.get_one::<String>("query").unwrap().as_str());
}

pub fn list_groups(global: &Global, args: &ArgMatches) {
    group::list_groups(
        global,
        args.get_one::<String>("start_at").unwrap().as_str(),
        args.get_one::<String>("max_results").unwrap().as_str(),
    );
}

pub fn list_users(global: &Global, args: &ArgMatches) {
    group::list_users(
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

pub fn remove_user(global: &Global, args: &ArgMatches) {
    let account_ids: Vec<&String> = args
        .get_many::<String>("account_id")
        .map(Iterator::collect)
        .unwrap();
    account_ids.par_iter().for_each(|account_id| {
        group::remove_user(
            global,
            account_id.as_str(),
            args.get_one::<String>("group_id").unwrap().as_str(),
        );
    });
}