/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    confirm, delete_request, generate_url, get_request, handle_error_and_exit, post_request,
    print_output, Global,
};

pub fn create(global: &Global, email: &str, display_name: &str) {
    let url = generate_url(&global.domain, "user", None);
    let payload: Value = json!({
        "emailAddress": email,
        "displayName": display_name
    });
    match post_request(&url, &payload, global.b64auth()) {
        Ok(_) => print_output(&format!("User {email} created")),
        Err(e) => handle_error_and_exit(&format!("Unable to create the user {email}: {e}")),
    }
}

#[allow(clippy::unit_arg)]
pub fn delete(global: &Global, account_id: &str) {
    let url = generate_url(
        &global.domain,
        "user",
        Some(&format!("?accountId={account_id}")),
    );

    if confirm(format!(
        "Are you sure you want to delete the account id: {account_id}?"
    )) {
        match delete_request(&url, global.b64auth()) {
            Ok(_) => print_output(&format!("User {account_id} deleted")),
            Err(e) => {
                handle_error_and_exit(&format!("Unable to delete the user {account_id}: {e}"));
            }
        }
    } else {
        print_output(&format!("User {account_id} not deleted."));
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn get_account_id(global: &Global, email_address: &str) {
    let url: String = generate_url(
        &global.domain,
        "group_user_picker",
        Some(&format!("?query={email_address}")),
    );
    let resp = get_request(&url, global.b64auth()).expect("Failed to get request");
    let json: Value = resp.json().expect("Failed to parse json");
    if json["users"]["users"]
        .as_array()
        .expect("Failed to get array")
        .is_empty()
    {
        handle_error_and_exit(format!("User {email_address} not found.").as_str());
    } else {
        print_output(
            json["users"]["users"][0]["accountId"]
                .as_str()
                .expect("Failed to get accountId")
                .to_string()
                .as_str(),
        );
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn get_user_groups(global: &Global, account_id: &str) {
    let url: String = generate_url(
        &global.domain,
        "user",
        Some(&format!("/groups?accountId={account_id}")),
    );
    match get_request(&url, global.b64auth()) {
        Ok(r) => {
            let json: Value = r.json().expect("Failed to parse json");
            if json["name"] == json!(null) {
                print_output(&format!("No groups found for account id {account_id}"));
            } else {
                json["name"].as_array().unwrap().par_iter().for_each(|x| {
                    print_output(&format!("{x}"));
                });
            }
        }
        Err(e) => handle_error_and_exit(&format!(
            "Impossible to get groups for user {account_id}: {e}"
        )),
    }
}
