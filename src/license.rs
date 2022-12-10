/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{Command, crate_description, crate_name};

fn license() {
    println!(
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

pub fn cli_commands() -> Command {
    Command::new("license")
        .visible_alias("L")
        .about("Show copyright and license")
}

pub fn logic_commands() {
    license();
}
