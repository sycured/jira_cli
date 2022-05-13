use std::collections::HashMap;

use clap::ArgMatches;
use rayon::prelude::*;

use super::functions;

pub fn add_user(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("account_id")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|account_id| {
            functions::add_user(global, account_id, args.value_of("group_id").unwrap())
        });
}

pub fn create(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("name")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|name| functions::create(global, name));
}

pub fn delete(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::delete(global, args.value_of("group_id").unwrap());
}

pub fn find(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::find(global, args.value_of("query").unwrap());
}

pub fn list_groups(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_groups(
        global,
        args.value_of("start_at").unwrap(),
        args.value_of("max_results").unwrap(),
    );
}

pub fn list_users(global: &HashMap<&str, &str>, args: &ArgMatches) {
    functions::list_users(
        global,
        args.value_of("group_id").unwrap(),
        args.value_of("include_inactive").unwrap().parse().unwrap(),
        args.value_of("start_at").unwrap(),
        args.value_of("max_results").unwrap(),
    );
}

pub fn remove_user(global: &HashMap<&str, &str>, args: &ArgMatches) {
    args.values_of("account_id")
        .unwrap()
        .collect::<Vec<&str>>()
        .par_iter()
        .for_each(|account_id| {
            functions::remove_user(global, account_id, args.value_of("group_id").unwrap())
        });
}
