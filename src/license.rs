/*
 * Copyright (c) 2022, sycured
 * All rights reserved
 *
 * SPDX-License-Identifier: GPL-2.0-only
 */

use clap::{crate_description, crate_name, Command};

fn license() {
    println!(
        "{}",
        format!(
            "{}\n\
            {}\n\
            \n\
        Copyright (c) 2022, sycured\n\
        All rights reserved\n\
        \n\
        SPDX-License-Identifier: GPL-2.0-only\n\
        \n\
        License file:\n\
            - https://github.com/sycured/jira_cli/blob/main/LICENSE\n\
            - https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt",
            crate_name!(),
            crate_description!()
        )
    );
}

pub fn cli_commands() -> Command<'static> {
    return Command::new("license")
        .visible_alias("L")
        .about("Show copyright and license");
}

pub fn logic_commands() {
    license();
}
