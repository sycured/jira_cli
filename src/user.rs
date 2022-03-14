use crate::lib::get_request;
use clap::{Arg, Command};
use serde_json::Value;
use ureq::Response;

pub fn get_account_id(
    jira_domain: String,
    jira_user: String,
    jira_token: String,
    email_address: String,
) {
    let url: String = format!(
        "https://{domain}/rest/api/3/groupuserpicker?query={query}",
        domain = jira_domain,
        query = email_address
    );
    let resp: Response = get_request(url, jira_user, jira_token);
    let json: Value = resp.into_json().unwrap();
    println!("{}", json["users"]["users"][0]["accountId"]);
}

pub fn cli_get_account_id() -> Command<'static> {
    Command::new("get_account_id")
        .about("Get account id")
        .arg_required_else_help(true)
        .arg(
            Arg::new("email_address")
                .help("Email address")
                .takes_value(true)
                .required(true),
        )
}
