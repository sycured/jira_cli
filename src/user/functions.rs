use serde_json::Value;
use ureq::Response;

use crate::lib::get_request;

pub fn get_account_id(domain: &str, user: &str, token: &str, email_address: &str) {
    let url: String = format!(
        "https://{}/rest/api/3/groupuserpicker?query={}",
        domain, email_address
    );
    let resp: Response = get_request(&url, user, token);
    let json: Value = resp.into_json().unwrap();
    println!("{}", json["users"]["users"][0]["accountId"]);
}
