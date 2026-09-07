/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod cli;
mod handler;
mod commands;
mod bridge;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    handler::handle_command(&cli.command);
}
