/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use jira_cli::{issue, Global};
use rayon::prelude::*;
use std::borrow::ToOwned;

const ACCOUNT_ID: &str = "account_id";
const ISSUE_KEY: &str = "issue_key";
const LABEL: &str = "label";
const REPORTER_ACCOUNT_ID: &str = "reporter_account_id";
const PROJECT_KEY: &str = "project_key";
const ISSUE_TYPE: &str = "issue_type";
const ISSUE_SUMMARY: &str = "issue_summary";
const ISSUE_DESCRIPTION: &str = "issue_description";
const ISSUE_PRIORITY: &str = "issue_priority";
const VERSION_NAME: &str = "version_name";
const NAME: &str = "name";
const OUTWARD: &str = "outward";
const INWARD: &str = "inward";
const TRANSITION_ID: &str = "transition_id";
const DELETE_SUBTASKS: &str = "delete_subtasks";

fn handle_issue_keys<F>(global: &Global, args: &ArgMatches, func: F)
where
    F: Fn(&Global, &String) + Sync,
{
    if let Some(issue_keys) = args.get_many::<String>(ISSUE_KEY) {
        let issue_keys: Vec<String> = issue_keys.map(ToOwned::to_owned).collect();
        issue_keys
            .par_iter()
            .for_each(|issue_key| func(global, issue_key));
    }
}

pub fn add_label(global: &Global, args: &ArgMatches) {
    let label = args.get_one::<String>(LABEL).unwrap();
    handle_issue_keys(global, args, |global, issue_key| {
        issue::add_label(global, issue_key, label);
    });
}

pub fn add_version(global: &Global, args: &ArgMatches) {
    let version_name = args.get_one::<String>(VERSION_NAME).unwrap();
    handle_issue_keys(global, args, |global, issue_key| {
        issue::add_version(global, version_name, issue_key);
    });
}

pub fn add_vote(global: &Global, args: &ArgMatches) {
    handle_issue_keys(global, args, |global, issue_key| {
        issue::add_vote(global, issue_key);
    });
}

pub fn assign(global: &Global, args: &ArgMatches) {
    let account_id = args.get_one::<String>(ACCOUNT_ID).unwrap().as_str();
    handle_issue_keys(global, args, |global, issue_key| {
        issue::assign(
            global,
            issue_key,
            account_id,
            &format!("Issue {issue_key} assigned"),
        );
    });
}

pub fn create(global: &Global, args: &ArgMatches) {
    issue::create(
        global,
        args.get_one::<String>(REPORTER_ACCOUNT_ID)
            .unwrap()
            .as_str(),
        args.get_one::<String>(PROJECT_KEY).unwrap().as_str(),
        args.get_one::<String>(ISSUE_TYPE).unwrap().as_str(),
        args.get_one::<String>(ISSUE_SUMMARY).unwrap().as_str(),
        args.get_one::<String>(ISSUE_DESCRIPTION).unwrap().as_str(),
        args.get_one::<String>(ISSUE_PRIORITY)
            .unwrap_or(&String::new())
            .as_str(),
    );
}

pub fn create_link_type(global: &Global, args: &ArgMatches) {
    issue::create_link_type(
        global,
        args.get_one::<String>(NAME).unwrap().as_str(),
        args.get_one::<String>(OUTWARD).unwrap().as_str(),
        args.get_one::<String>(INWARD).unwrap().as_str(),
    );
}

pub fn delete(global: &Global, args: &ArgMatches) {
    issue::delete(
        global,
        args.get_one::<String>(ISSUE_KEY).unwrap().as_str(),
        args.get_flag(DELETE_SUBTASKS),
    );
}

pub fn delete_link_type(global: &Global, args: &ArgMatches) {
    issue::delete_link_type(global, args.get_one::<String>("id").unwrap().as_str());
}

pub fn get_link_type(global: &Global, args: &ArgMatches) {
    issue::get_link_type(global, args.get_one::<String>("id").unwrap().as_str());
}

pub fn get_transitions(global: &Global, args: &ArgMatches) {
    issue::get_transitions(global, args.get_one::<String>(ISSUE_KEY).unwrap().as_str());
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
        args.get_one::<String>(PROJECT_KEY).unwrap().as_str(),
    );
}

pub fn list_votes(global: &Global, args: &ArgMatches) {
    issue::list_votes(global, args.get_one::<String>(ISSUE_KEY).unwrap().as_str());
}

pub fn remove_label(global: &Global, args: &ArgMatches) {
    let label = args.get_one::<String>(LABEL).unwrap().as_str();
    handle_issue_keys(global, args, |global, issue_key| {
        issue::remove_label(global, issue_key, label);
    });
}

pub fn remove_version(global: &Global, args: &ArgMatches) {
    let version_name = args.get_one::<String>(VERSION_NAME).unwrap().as_str();
    handle_issue_keys(global, args, |global, issue_key| {
        issue::remove_version(global, version_name, issue_key);
    });
}

pub fn remove_vote(global: &Global, args: &ArgMatches) {
    issue::remove_vote(global, args.get_one::<String>(ISSUE_KEY).unwrap().as_str());
}

pub fn show_fixversions(global: &Global, args: &ArgMatches) {
    issue::show_fixversions(global, args.get_one::<String>(ISSUE_KEY).unwrap().as_str());
}

pub fn transition(global: &Global, args: &ArgMatches) {
    issue::transition(
        global,
        args.get_one::<String>(ISSUE_KEY).unwrap().as_str(),
        args.get_one::<String>(TRANSITION_ID).unwrap().as_str(),
    );
}

pub fn unassign(global: &Global, args: &ArgMatches) {
    handle_issue_keys(global, args, |global, issue_key| {
        issue::assign(
            global,
            issue_key,
            "null",
            &format!("Issue {issue_key} unassigned"),
        );
    });
}

pub fn update_link_type(global: &Global, args: &ArgMatches) {
    issue::update_link_type(
        global,
        args.get_one::<String>("id").unwrap().as_str(),
        args.get_one::<String>(NAME).unwrap().as_str(),
        args.get_one::<String>(OUTWARD).unwrap().as_str(),
        args.get_one::<String>(INWARD).unwrap().as_str(),
    );
}
