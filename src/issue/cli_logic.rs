use crate::issue::functions;
use clap::ArgMatches;

pub fn add_version(matches: &ArgMatches, args: &ArgMatches) {
    functions::add_version(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("version_name").unwrap(),
        args.value_of("issue_key").unwrap(),
    );
}

pub fn create_issue(matches: &ArgMatches, args: &ArgMatches) {
    functions::create_issue(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("reporter_account_id").unwrap(),
        args.value_of("project_key").unwrap(),
        args.value_of("issue_type").unwrap(),
        args.value_of("issue_summary").unwrap(),
        args.value_of("issue_description").unwrap(),
        args.value_of("issue_priority").unwrap_or(""),
    );
}

pub fn list_issue_priorities(matches: &ArgMatches) {
    functions::list_issue_priorities(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
    );
}

pub fn list_issue_types(matches: &ArgMatches, args: &ArgMatches) {
    functions::list_issue_types(
        matches.value_of("domain").unwrap(),
        matches.value_of("user").unwrap(),
        matches.value_of("token").unwrap(),
        args.value_of("project_key").unwrap(),
    );
}
