/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::{collections::HashMap, process::exit};

use rayon::prelude::*;
use serde_json::{json, Value};

use jira_cli::{confirm, delete_request, get_request, post_request};

use crate::urls::URLS;

pub fn create(global: &HashMap<&str, &str>, email: &str, display_name: &str) {
    let url = format!("https://{}{}", global["domain"], URLS["user"]);
    let payload: Value = json!({
        "emailAddress": email,
        "displayName": display_name
    });
    match post_request(&url, &payload, global["user"], global["token"]) {
        Ok(_) => println!("User {email} created"),
        Err(e) => {
            eprintln!("Impossible to create the user {email}: {e}");
            exit(1);
        }
    }
}

#[allow(clippy::unit_arg)]
pub fn delete(global: &HashMap<&str, &str>, account_id: &str) {
    let url = format!(
        "https://{}{}?accountId={account_id}",
        global["domain"], URLS["user"]
    );

    if confirm(format!(
        "Are you sure you want to delete the account id: {}?",
        account_id
    )) {
        match delete_request(&url, global["user"], global["token"]) {
            Ok(_) => println!("User {account_id} deleted"),
            Err(e) => {
                eprintln!("Impossible to delete the user {account_id}: {e}");
            }
        }
    } else {
        println!("User {account_id} not deleted.");
    }
}

pub fn get_account_id(global: &HashMap<&str, &str>, email_address: &str) {
    let url: String = format!(
        "https://{}{}?query={email_address}",
        global["domain"], URLS["group_user_picker"]
    );
    let resp = get_request(&url, global["user"], global["token"]).unwrap();
    let json: Value = resp.json().unwrap();
    if json["users"]["users"].as_array().unwrap().is_empty() {
        println!("User {email_address} not found.");
        exit(1);
    } else {
        println!(
            "{}",
            json["users"]["users"][0]["accountId"].as_str().unwrap()
        );
    }
}

pub fn get_user_groups(global: &HashMap<&str, &str>, account_id: &str) {
    let url: String = format!(
        "https://{}{}/groups?accountId={account_id}",
        global["domain"], URLS["user"]
    );
    match get_request(&url, global["user"], global["token"]) {
        Ok(r) => {
            let json: Value = r.json().unwrap();
            if json["name"] == json!(null) {
                println!("No groups found for account id {account_id}");
            } else {
                json["name"].as_array().unwrap().par_iter().for_each(|x| {
                    println!("{x}");
                });
            }
        }
        Err(e) => {
            eprintln!("Impossible to get groups for user {account_id}: {e}");
            exit(1);
        }
    }
}
