/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use rayon::prelude::*;

use jira_cli::{get_single_arg, project, Global};

fn get_project_keys(args: &ArgMatches) -> Vec<&String> {
    args.get_many::<String>("project_key")
        .map(Iterator::collect)
        .unwrap()
}

fn get_feature_keys(args: &ArgMatches) -> Vec<&String> {
    args.get_many::<String>("feature_key")
        .map(Iterator::collect)
        .unwrap()
}

pub fn create(global: &Global, args: &ArgMatches) {
    let (project_name, project_key, jira_project_leadaccountid, project_type, project_template) = (
        get_single_arg(args, "project_name"),
        get_single_arg(args, "project_key"),
        get_single_arg(args, "jira_project_leadaccountid"),
        get_single_arg(args, "project_type"),
        get_single_arg(args, "project_template"),
    );
    project::create(
        global,
        project_name,
        project_key,
        jira_project_leadaccountid,
        project_type,
        project_template,
    );
}

pub fn delete_project(global: &Global, args: &ArgMatches) {
    let project_keys = get_project_keys(args);
    project_keys
        .par_iter()
        .for_each(|prj| project::delete(global, prj, !args.get_flag("disable_undo")));
}

pub fn get_id(global: &Global, args: &ArgMatches) {
    let project_key = get_single_arg(args, "project_key");
    project::get_id(global, project_key);
}

pub fn list_features(global: &Global, args: &ArgMatches) {
    let project_key = get_single_arg(args, "project_key");
    project::list_features(global, project_key);
}

pub fn list_projects(global: &Global, args: &ArgMatches) {
    let (start_at, max_results) = (
        get_single_arg(args, "start_at"),
        get_single_arg(args, "max_results"),
    );
    project::list_projects(global, start_at, max_results).unwrap();
}

pub fn list_versions(global: &Global, args: &ArgMatches) {
    let project_key = get_single_arg(args, "project_key");
    project::list_versions(global, project_key);
}

pub fn new_version(global: &Global, args: &ArgMatches) {
    let (project_id, version_name) = (
        get_single_arg(args, "project_id"),
        get_single_arg(args, "version_name"),
    );
    project::new_version(global, project_id, version_name);
}

pub fn set_feature_state(global: &Global, args: &ArgMatches) {
    let feature_keys = get_feature_keys(args);
    let project_key = get_single_arg(args, "project_key");
    let feature_state = get_single_arg(args, "feature_state");
    feature_keys.par_iter().for_each(|feature_key| {
        project::set_feature_state(global, project_key, feature_key, feature_state);
    });
}
