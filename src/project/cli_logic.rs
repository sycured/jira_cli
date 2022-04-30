use std::collections::HashMap;

use clap::ArgMatches;

use rayon::prelude::*;

use crate::project::functions;

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
        global,
        args.value_of("project_name").unwrap(),
        args.value_of("project_key").unwrap(),
        args.value_of("jira_project_leadaccountid").unwrap(),
        args.value_of("project_type").unwrap(),
        args.value_of("project_template").unwrap(),
    );
}

pub fn delete_project(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete_project(global, args.value_of("project_key").unwrap());
}

pub fn get_id(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_id(global, args.value_of("project_key").unwrap());
}

pub fn list_features(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_features(global, args.value_of("project_key").unwrap());
}

pub fn list_versions(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_versions(global, args.value_of("project_key").unwrap());
}

pub fn new_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::new_version(
        global,
        args.value_of("project_id").unwrap(),
        args.value_of("version_name").unwrap(),
    );
}

pub fn set_feature_state(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("feature_key")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|feature_key| {
            functions::set_feature_state(
                global,
                args.value_of("project_key").unwrap(),
                feature_key,
                args.value_of("feature_state").unwrap(),
            )
        });
}
