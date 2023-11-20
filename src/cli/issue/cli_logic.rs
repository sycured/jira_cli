/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use jira_cli::{get_single_arg, issue, Global};
use rayon::prelude::*;
use std::borrow::ToOwned;

fn handle_issue_keys<F>(global: &Global, args: &ArgMatches, func: F)
where
    F: Fn(&Global, &str) + Sync,
{
    if let Some(issue_keys) = args.get_many::<String>("issue_key") {
        let issue_keys: Vec<String> = issue_keys.map(ToOwned::to_owned).collect();
        issue_keys
            .par_iter()
            .for_each(|issue_key| func(global, issue_key.as_str()));
    }
}

pub fn add_label(global: &Global, args: &ArgMatches) {
    let label = get_single_arg(args, "label");
    handle_issue_keys(global, args, |global, issue_key| {
        issue::add_label(global, issue_key, label);
    });
}

pub fn add_version(global: &Global, args: &ArgMatches) {
    let version_name = get_single_arg(args, "version_name");
    handle_issue_keys(global, args, |global, issue_key| {
        issue::add_version(global, version_name, issue_key);
    });
}

pub fn add_vote(global: &Global, args: &ArgMatches) {
    handle_issue_keys(global, args, issue::add_vote);
}

pub fn assign(global: &Global, args: &ArgMatches) {
    let account_id = get_single_arg(args, "account_id");
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
    let (
        reporter_account_id,
        project_key,
        issue_type,
        issue_summary,
        issue_description,
        issue_priority,
    ) = (
        get_single_arg(args, "reporter_account_id"),
        get_single_arg(args, "project_key"),
        get_single_arg(args, "issue_type"),
        get_single_arg(args, "issue_summary"),
        get_single_arg(args, "issue_description"),
        get_single_arg(args, "issue_priority"),
    );
    issue::create(
        global,
        reporter_account_id,
        project_key,
        issue_type,
        issue_summary,
        issue_description,
        issue_priority,
    );
}

pub fn create_link_type(global: &Global, args: &ArgMatches) {
    let (name, outward, inward) = (
        get_single_arg(args, "name"),
        get_single_arg(args, "outward"),
        get_single_arg(args, "inward"),
    );
    issue::create_link_type(global, name, outward, inward);
}

pub fn delete(global: &Global, args: &ArgMatches) {
    let issue_key = get_single_arg(args, "issue_key");
    let delete_subtasks = args.get_flag("delete_subtasks");
    issue::delete(global, issue_key, delete_subtasks);
}

pub fn delete_link_type(global: &Global, args: &ArgMatches) {
    let id = get_single_arg(args, "id");
    issue::delete_link_type(global, id);
}

pub fn get_link_type(global: &Global, args: &ArgMatches) {
    let id = get_single_arg(args, "id");
    issue::get_link_type(global, id);
}

pub fn get_transitions(global: &Global, args: &ArgMatches) {
    let issue_key = get_single_arg(args, "issue_key");
    issue::get_transitions(global, issue_key);
}

pub fn list_link_types(global: &Global) {
    issue::list_link_types(global);
}

pub fn list_priorities(global: &Global) {
    issue::list_priorities(global);
}

pub fn list_types(global: &Global, args: &ArgMatches) {
    let project_key = get_single_arg(args, "project_key");
    issue::list_types(global, project_key);
}

pub fn list_votes(global: &Global, args: &ArgMatches) {
    let issue_key = get_single_arg(args, "issue_key");
    issue::list_votes(global, issue_key);
}

pub fn remove_label(global: &Global, args: &ArgMatches) {
    let label = get_single_arg(args, "label");
    handle_issue_keys(global, args, |global, issue_key| {
        issue::remove_label(global, issue_key, label);
    });
}

pub fn remove_version(global: &Global, args: &ArgMatches) {
    let version_name = get_single_arg(args, "version_name");
    handle_issue_keys(global, args, |global, issue_key| {
        issue::remove_version(global, version_name, issue_key);
    });
}

pub fn remove_vote(global: &Global, args: &ArgMatches) {
    issue::remove_vote(global, get_single_arg(args, "issue_key"));
}

pub fn show_fixversions(global: &Global, args: &ArgMatches) {
    issue::show_fixversions(global, get_single_arg(args, "issue_key"));
}

pub fn transition(global: &Global, args: &ArgMatches) {
    issue::transition(
        global,
        get_single_arg(args, "issue_key"),
        get_single_arg(args, "transition_id"),
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
        get_single_arg(args, "id"),
        get_single_arg(args, "name"),
        get_single_arg(args, "outward"),
        get_single_arg(args, "inward"),
    );
}
