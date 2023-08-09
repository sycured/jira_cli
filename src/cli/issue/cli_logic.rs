/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use rayon::prelude::*;

use jira_cli::{issue, Global};

pub fn add_label(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        issue::add_label(
            global,
            issue_key.as_str(),
            args.get_one::<String>("label").unwrap().as_str(),
        );
    });
}

pub fn add_version(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        issue::add_version(
            global,
            args.get_one::<String>("version_name").unwrap().as_str(),
            issue_key.as_str(),
        );
    });
}

pub fn add_vote(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys
        .par_iter()
        .for_each(|issue_key| issue::add_vote(global, issue_key.as_str()));
}

pub fn assign(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        issue::assign(
            global,
            args.get_one::<String>("issue_key").unwrap().as_str(),
            args.get_one::<String>("account_id").unwrap().as_str(),
            format!("Issue {issue_key} assigned").as_str(),
        );
    });
}

pub fn create(global: &Global, args: &ArgMatches) {
    issue::create(
        global,
        args.get_one::<String>("reporter_account_id")
            .unwrap()
            .as_str(),
        args.get_one::<String>("project_key").unwrap().as_str(),
        args.get_one::<String>("issue_type").unwrap().as_str(),
        args.get_one::<String>("issue_summary").unwrap().as_str(),
        args.get_one::<String>("issue_description")
            .unwrap()
            .as_str(),
        args.get_one::<String>("issue_priority")
            .unwrap_or(&String::new())
            .as_str(),
    );
}

pub fn create_link_type(global: &Global, args: &ArgMatches) {
    issue::create_link_type(
        global,
        args.get_one::<String>("name").unwrap().as_str(),
        args.get_one::<String>("outward").unwrap().as_str(),
        args.get_one::<String>("inward").unwrap().as_str(),
    );
}

pub fn delete(global: &Global, args: &ArgMatches) {
    issue::delete(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
        args.get_one::<String>("delete_subtasks").unwrap().as_str(),
    );
}

pub fn delete_link_type(global: &Global, args: &ArgMatches) {
    issue::delete_link_type(global, args.get_one::<String>("id").unwrap().as_str());
}

pub fn get_link_type(global: &Global, args: &ArgMatches) {
    issue::get_link_type(global, args.get_one::<String>("id").unwrap().as_str());
}

pub fn get_transitions(global: &Global, args: &ArgMatches) {
    issue::get_transitions(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn list_link_types(global: &Global) {
    issue::list_link_types(global);
}

pub fn list_priorities(global: &Global) {
    issue::list_priorities(global);
}

pub fn list_types(global: &Global, args: &ArgMatches) {
    issue::list_types(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn list_votes(global: &Global, args: &ArgMatches) {
    issue::list_votes(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn remove_label(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        issue::remove_label(
            global,
            issue_key.as_str(),
            args.get_one::<String>("label").unwrap().as_str(),
        );
    });
}

pub fn remove_version(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        issue::remove_version(
            global,
            args.get_one::<String>("version_name").unwrap().as_str(),
            issue_key.as_str(),
        );
    });
}

pub fn remove_vote(global: &Global, args: &ArgMatches) {
    issue::remove_vote(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn show_fixversions(global: &Global, args: &ArgMatches) {
    issue::show_fixversions(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn transition(global: &Global, args: &ArgMatches) {
    issue::transition(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
        args.get_one::<String>("transition_id").unwrap().as_str(),
    );
}

pub fn unassign(global: &Global, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(Iterator::collect)
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        issue::assign(
            global,
            args.get_one::<String>("issue_key").unwrap().as_str(),
            "null",
            format!("Issue {issue_key} unassigned").as_str(),
        );
    });
}

pub fn update_link_type(global: &Global, args: &ArgMatches) {
    issue::update_link_type(
        global,
        args.get_one::<String>("id").unwrap().as_str(),
        args.get_one::<String>("name").unwrap().as_str(),
        args.get_one::<String>("outward").unwrap().as_str(),
        args.get_one::<String>("inward").unwrap().as_str(),
    );
}
