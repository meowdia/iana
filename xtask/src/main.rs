// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

mod cli;
mod codegen;
mod fetch;
mod output;
mod snapshot;

use cli::Command;
use codegen::Mode;
use std::{env, process::ExitCode};

type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    match Command::parse(env::args().skip(1))? {
        Command::Discover => {
            for group in fetch::discover_online()? {
                println!("{group}");
            }
            Ok(())
        }
        Command::Fetch(selection) => fetch::run(selection),
        Command::Update(selection) => {
            fetch::run(selection)?;
            codegen::generate(Mode::Write)
        }
        Command::Generate => codegen::generate(Mode::Write),
        Command::Check => codegen::generate(Mode::Check),
    }
}
