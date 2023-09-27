/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use phf::phf_map;

pub(crate) static URLS: phf::Map<&'static str, &'static str> = phf_map! {
    "group" => "/rest/api/3/group",
    "group_user_picker" => "/rest/api/3/groupuserpicker",
    "groups" => "/rest/api/3/groups",
    "issue" => "/rest/api/3/issue",
    "issue_link_types" => "/rest/api/3/issueLinkType",
    "label" => "/rest/api/3/label",
    "priority" => "/rest/api/3/priority",
    "project" => "/rest/api/3/project",
    "sprint" => "/est/agile/1.0/sprint",
    "user" => "/rest/api/3/user",
    "version" => "/rest/api/3/version",
};
