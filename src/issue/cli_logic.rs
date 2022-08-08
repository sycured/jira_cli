use std::collections::HashMap;

use clap::ArgMatches;
use rayon::prelude::*;

use super::functions;

pub fn add_label(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        functions::add_label(
            global,
            issue_key.as_str(),
            args.get_one::<String>("label").unwrap().as_str(),
        )
    });
}

pub fn add_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        functions::add_version(
            global,
            args.get_one::<String>("version_name").unwrap().as_str(),
            issue_key.as_str(),
        )
    });
}

pub fn add_vote(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys
        .par_iter()
        .for_each(|issue_key| functions::add_vote(global, issue_key.as_str()));
}

pub fn assign(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        functions::assign(
            global,
            args.get_one::<String>("issue_key").unwrap().as_str(),
            args.get_one::<String>("account_id").unwrap().as_str(),
            format!("Issue {} assigned", issue_key).as_str(),
        )
    });
}

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create(
        global,
        args.get_one::<String>("reporter_account_id")
            .unwrap()
            .as_str(),
        args.get_one::<String>("project_key").unwrap().as_str(),
        args.get_one::<String>("issue_type").unwrap().as_str(),
        args.get_one::<String>("issue_summary").unwrap().as_str(),
        args.get_one::<String>("issue_description")
            .unwrap()
            .as_str(),
        args.get_one::<String>("issue_priority")
            .unwrap_or(&"".to_string())
            .as_str(),
    );
}

pub fn create_link_type(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::create_link_type(
        global,
        args.get_one::<String>("name").unwrap().as_str(),
        args.get_one::<String>("outward").unwrap().as_str(),
        args.get_one::<String>("inward").unwrap().as_str(),
    );
}

pub fn delete(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
        args.get_one::<String>("delete_subtasks").unwrap().as_str(),
    );
}

pub fn delete_link_type(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete_link_type(global, args.get_one::<String>("id").unwrap().as_str());
}

pub fn get_link_type(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_link_type(global, args.get_one::<String>("id").unwrap().as_str());
}

pub fn get_transitions(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::get_transitions(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn list_link_types(global: &HashMap<&str, &str>) {
    functions::list_link_types(global);
}

pub fn list_priorities(global: &HashMap<&str, &str>) {
    functions::list_priorities(global);
}

pub fn list_types(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_types(
        global,
        args.get_one::<String>("project_key").unwrap().as_str(),
    );
}

pub fn list_votes(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_votes(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn remove_label(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        functions::remove_label(
            global,
            issue_key.as_str(),
            args.get_one::<String>("label").unwrap().as_str(),
        )
    });
}

pub fn remove_version(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        functions::remove_version(
            global,
            args.get_one::<String>("version_name").unwrap().as_str(),
            issue_key.as_str(),
        )
    });
}

pub fn remove_vote(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::remove_vote(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn show_fixversions(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::show_fixversions(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
    );
}

pub fn transition(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::transition(
        global,
        args.get_one::<String>("issue_key").unwrap().as_str(),
        args.get_one::<String>("transition_id").unwrap().as_str(),
    );
}

pub fn unassign(global: &HashMap<&str, &str>, args: &ArgMatches) {
    let issue_keys: Vec<&String> = args
        .get_many::<String>("issue_key")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap();
    issue_keys.par_iter().for_each(|issue_key| {
        functions::assign(
            global,
            args.get_one::<String>("issue_key").unwrap().as_str(),
            "null",
            format!("Issue {} unassigned", issue_key).as_str(),
        )
    });
}

pub fn update_link_type(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::update_link_type(
        global,
        args.get_one::<String>("id").unwrap().as_str(),
        args.get_one::<String>("name").unwrap().as_str(),
        args.get_one::<String>("outward").unwrap().as_str(),
        args.get_one::<String>("inward").unwrap().as_str(),
    );
}
