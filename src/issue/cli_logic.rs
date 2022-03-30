use std::collections::HashMap;

use clap::ArgMatches;

use crate::issue::functions;

pub fn add_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::add_version(
        global["domain"],
        global["user"],
        global["token"],
        args.value_of("version_name").unwrap(),
        args.value_of("issue_key").unwrap(),
    );
}

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
        global["domain"],
        global["user"],
        global["token"],
        args.value_of("reporter_account_id").unwrap(),
        args.value_of("project_key").unwrap(),
        args.value_of("issue_type").unwrap(),
        args.value_of("issue_summary").unwrap(),
        args.value_of("issue_description").unwrap(),
        args.value_of("issue_priority").unwrap_or(""),
    );
}

pub fn list_priorities(global: &HashMap<&str, &str>) {
    functions::list_priorities(global["domain"], global["user"], global["token"]);
}

pub fn list_types(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_types(
        global["domain"],
        global["user"],
        global["token"],
        args.value_of("project_key").unwrap(),
    );
}

pub fn show_fixversions(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::show_fixversions(
        global["domain"],
        global["user"],
        global["token"],
        args.value_of("issue_key").unwrap(),
    );
}
