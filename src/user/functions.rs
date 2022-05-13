use std::collections::HashMap;

use attohttpc::Response;
use dialoguer::Confirm;
use serde_json::{json, Value};

use crate::{
    lib::{delete_request, get_request, post_request},
    urls::URLS,
};

pub fn create(global: &HashMap<&str, &str>, email: &str, display_name: &str) {
    let url = format!("https://{}{}", global["domain"], URLS["user"]);
    let payload: Value = json!({
        "emailAddress": email,
        "displayName": display_name
    });
    let success_message: String = format!("User {} created", email);
    post_request(
        &url,
        &payload,
        global["user"],
        global["token"],
        &success_message,
        false,
    );
}

pub fn delete(global: &HashMap<&str, &str>, account_id: &str) {
    let url = format!(
        "https://{}{}?accountId={}",
        global["domain"], URLS["user"], account_id
    );
    if Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to delete the account id: {}?",
            account_id
        ))
        .interact()
        .unwrap()
    {
        let success_message: String = format!("User {} deleted", account_id);
        delete_request(&url, global["user"], global["token"], &success_message);
    } else {
        println!("User {} not deleted.", account_id);
    }
}

pub fn get_account_id(global: &HashMap<&str, &str>, email_address: &str) {
    let url: String = format!(
        "https://{}{}?query={}",
        global["domain"], URLS["group_user_picker"], email_address
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    println!(
        "{}",
        json["users"]["users"][0]["accountId"].as_str().unwrap()
    );
}

pub fn get_user_groups(global: &HashMap<&str, &str>, account_id: &str) {
    let url: String = format!(
        "https://{}{}/groups?accountId={}",
        global["domain"], URLS["user"], account_id
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.json().unwrap();
    if json["name"] != json!(null) {
        json["name"].as_array().unwrap().iter().for_each(|x| {
            println!("{}", x);
        });
    } else {
        println!("No groups found for account id {}", account_id);
    }
}
