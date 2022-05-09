use std::collections::HashMap;

use clap::ArgMatches;
use rayon::prelude::*;

use crate::issue::functions;

pub fn add_label(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("issue_key")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|issue_key| {
            functions::add_label(global, issue_key, args.value_of("label").unwrap())
        });
}

pub fn add_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("issue_key")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|issue_key| {
            functions::add_version(global, args.value_of("version_name").unwrap(), issue_key)
        });
}

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
        global,
        args.value_of("reporter_account_id").unwrap(),
        args.value_of("project_key").unwrap(),
        args.value_of("issue_type").unwrap(),
        args.value_of("issue_summary").unwrap(),
        args.value_of("issue_description").unwrap(),
        args.value_of("issue_priority").unwrap_or(""),
    );
}

pub fn list_priorities(global: &HashMap<&str, &str>) {
    functions::list_priorities(global);
}

pub fn list_types(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_types(global, args.value_of("project_key").unwrap());
}

pub fn remove_label(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("issue_key")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|issue_key| {
            functions::remove_label(global, issue_key, args.value_of("label").unwrap())
        });
}

pub fn remove_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("issue_key")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|issue_key| {
            functions::remove_version(global, args.value_of("version_name").unwrap(), issue_key)
        });
}

pub fn show_fixversions(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::show_fixversions(global, args.value_of("issue_key").unwrap());
}
