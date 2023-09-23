/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref URLS: HashMap<&'static str, &'static str> = [
        ("group", "/rest/api/3/group"),
        ("group_user_picker", "/rest/api/3/groupuserpicker"),
        ("groups", "/rest/api/3/groups"),
        ("issue", "/rest/api/3/issue"),
        ("issue_link_types", "/rest/api/3/issueLinkType"),
        ("label", "/rest/api/3/label"),
        ("priority", "/rest/api/3/priority"),
        ("project", "/rest/api/3/project"),
        ("sprint", "/est/agile/1.0/sprint"),
        ("user", "/rest/api/3/user"),
        ("version", "/rest/api/3/version"),
    ]
    .iter()
    .copied()
    .collect();
}
