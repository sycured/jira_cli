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

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
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

pub fn delete_project(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete_project(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn get_id(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_id(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn list_features(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_features(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn list_versions(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_versions(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn new_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::new_version(
        global,
        args.get_one::<String>("project_id").unwrap().as_str(),
        args.get_one::<String>("version_name").unwrap().as_str(),
    );
}

pub fn set_feature_state(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let feature_keys: Vec<&String> = args
        .get_many::<String>("feature_key")
        .map(std::iter::Iterator::collect)
        .unwrap();
    feature_keys.par_iter().for_each(|feature_key| {
        functions::set_feature_state(
            global,
            args.get_one::<String>("project_key").unwrap().as_str(),
            feature_key.as_str(),
            args.get_one::<String>("feature_state").unwrap().as_str(),
        );
    });
}
