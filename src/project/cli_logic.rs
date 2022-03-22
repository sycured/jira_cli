use crate::project::functions;
use clap::ArgMatches;

pub fn create_project(matches: &ArgMatches, args: &ArgMatches) {
    functions::create_project(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("project_name").unwrap(),
        args.value_of("project_key").unwrap(),
        args.value_of("jira_project_leadaccountid").unwrap(),
        args.value_of("project_type").unwrap(),
        args.value_of("project_template").unwrap(),
    );
}

pub fn create_version(matches: &ArgMatches, args: &ArgMatches) {
    functions::create_version(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("project_id").unwrap(),
        args.value_of("version_name").unwrap(),
    );
}

pub fn delete_project(matches: &ArgMatches, args: &ArgMatches) {
    functions::delete_project(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("project_key").unwrap(),
    );
}

pub fn get_project_id(matches: &ArgMatches, args: &ArgMatches) {
    functions::get_project_id(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("project_key").unwrap(),
    );
}

pub fn set_project_feature_state(matches: &ArgMatches, args: &ArgMatches) {
    functions::set_project_feature_state(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("project_key").unwrap(),
        args.value_of("feature_key").unwrap(),
        args.value_of("feature_state").unwrap(),
    );
}
