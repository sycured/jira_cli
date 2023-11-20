/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use rayon::prelude::*;

use jira_cli::{get_single_arg, group, Global};

fn get_account_ids(args: &ArgMatches) -> Vec<&String> {
    args.get_many::<String>("account_id")
        .map(Iterator::collect)
        .unwrap()
}

fn get_names(args: &ArgMatches) -> Vec<&String> {
    args.get_many::<String>("name")
        .map(Iterator::collect)
        .unwrap()
}

pub fn add_user(global: &Global, args: &ArgMatches) {
    let account_ids = get_account_ids(args);
    let group_id = get_single_arg(args, "group_id");
    account_ids.par_iter().for_each(|account_id| {
        group::add_user(global, account_id, group_id);
    });
}

pub fn create(global: &Global, args: &ArgMatches) {
    let names = get_names(args);
    names
        .par_iter()
        .for_each(|name| group::create(global, name));
}

pub fn delete(global: &Global, args: &ArgMatches) {
    let group_id = get_single_arg(args, "group_id");
    group::delete(global, group_id);
}

pub fn find(global: &Global, args: &ArgMatches) {
    let query = get_single_arg(args, "query");
    group::find(global, query);
}

pub fn list_groups(global: &Global, args: &ArgMatches) {
    let start_at = get_single_arg(args, "start_at");
    let max_results = get_single_arg(args, "max_results");
    group::list_groups(global, start_at, max_results);
}

pub fn list_users(global: &Global, args: &ArgMatches) {
    let group_id = get_single_arg(args, "group_id");
    let include_inactive = get_single_arg(args, "include_inactive").parse().unwrap();
    let start_at = get_single_arg(args, "start_at");
    let max_results = get_single_arg(args, "max_results");
    group::list_users(global, group_id, include_inactive, start_at, max_results);
}

pub fn remove_user(global: &Global, args: &ArgMatches) {
    let account_ids = get_account_ids(args);
    let group_id = get_single_arg(args, "group_id");
    account_ids.par_iter().for_each(|account_id| {
        group::remove_user(global, account_id, group_id);
    });
}
