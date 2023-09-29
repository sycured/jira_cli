/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![forbid(unsafe_code)]

use crate::urls::URLS;

pub mod group;
pub mod issue;
pub mod labels;
pub mod project;
pub mod sprint;
pub mod urls;
pub mod user;

#[must_use]
pub fn generate_url(domain: &str, key: &str, query: Option<&str>) -> String {
    let mut url = format!("https://{}{}", domain, URLS[key]);
    if let Some(query) = query {
        url = format!("{url}{query}");
    }
    url
}
