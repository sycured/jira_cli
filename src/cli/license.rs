/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{crate_description, crate_name, Command};
use lazy_static::lazy_static;

lazy_static! {
    static ref LICENSE_TEXT: String = format!(
        "{}\n\
        {}\n\
        \n\
        Copyright (c) 2022, sycured\
        \nAll rights reserved\
        \n\
        \n\
        SPDX-License-Identifier: GPL-2.0-only\
        \n\
        \n\
        License file:\n\
        - https://github.com/sycured/jira_cli/blob/main/LICENSE\n\
        - https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt",
        crate_name!(),
        crate_description!()
    );
}

fn license() {
    println!("{}", &*LICENSE_TEXT);
}

pub fn cli_commands() -> Command {
    Command::new("license")
        .visible_alias("L")
        .about("Show copyright and license")
        .ignore_errors(true)
}

pub fn logic_commands() {
    license();
}
