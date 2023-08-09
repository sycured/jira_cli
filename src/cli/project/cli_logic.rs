/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::ArgMatches;
use rayon::prelude::*;

use jira_cli::{project, Global};

pub fn create(global: &Global, args: &ArgMatches) {
    project::create(
        global,
        args.get_one::<String>("project_name").unwrap().as_str(),
        args.get_one::<String>("project_key").unwrap().as_str(),
        args.get_one::<String>("jira_project_leadaccountid")
            .unwrap()
            .as_str(),
        args.get_one::<String>("project_type").unwrap().as_str(),
        args.get_one::<String>("project_template").unwrap().as_str(),
    );
}

pub fn delete_project(global: &Global, args: &ArgMatches) {
    project::delete(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn get_id(global: &Global, args: &ArgMatches) {
    project::get_id(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn list_features(global: &Global, args: &ArgMatches) {
    project::list_features(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn list_versions(global: &Global, args: &ArgMatches) {
    project::list_versions(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn new_version(global: &Global, args: &ArgMatches) {
    project::new_version(
        global,
        args.get_one::<String>("project_id").unwrap().as_str(),
        args.get_one::<String>("version_name").unwrap().as_str(),
    );
}

pub fn set_feature_state(global: &Global, args: &ArgMatches) {
    let feature_keys: Vec<&String> = args
        .get_many::<String>("feature_key")
        .map(Iterator::collect)
        .unwrap();
    feature_keys.par_iter().for_each(|feature_key| {
        project::set_feature_state(
            global,
            args.get_one::<String>("project_key").unwrap().as_str(),
            feature_key.as_str(),
            args.get_one::<String>("feature_state").unwrap().as_str(),
        );
    });
}
