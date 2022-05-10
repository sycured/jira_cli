use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref URLS: HashMap<&'static str, &'static str> = [
        ("group_user_picker", "/rest/api/3/groupuserpicker"),
        ("issue", "/rest/api/3/issue"),
        ("label", "/rest/api/3/label"),
        ("priority", "/rest/api/3/priority"),
        ("project", "/rest/api/3/project"),
        ("user", "/rest/api/3/user"),
        ("version", "/rest/api/3/version"),
    ]
    .iter()
    .cloned()
    .collect();
}
