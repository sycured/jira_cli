use serde_json::Value;
use std::collections::HashMap;
use ureq::Response;

use crate::lib::get_request;

pub fn get_account_id(global: &HashMap<&str, &str>, email_address: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/groupuserpicker?query={}",
        global["domain"], email_address
    );
    let resp: Response = get_request(&url, global["user"], global["token"]);
    let json: Value = resp.into_json().unwrap();
    println!(
        "{}",
        json["users"]["users"][0]["accountId"].as_str().unwrap()
    );
}
