#![feature(const_fn_fn_ptr_basics)]
#![feature(iterator_fold_self)]

mod binary_boarding;
mod custom_customs;
mod lib;
mod passport_processing;
mod password_philosophy;
mod report_repair;
mod toboggan_trajectory;

use anyhow::Error;
use clap::{App, AppSettings};
use lib::Command;
use simple_error::SimpleError;
use std::collections::HashMap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const COMMANDS: &'static [Command] = &[
    toboggan_trajectory::TOBOGGAN_TRAJECTORY,
    password_philosophy::PASSWORD_PHILOSOPHY,
    report_repair::REPORT_REPAIR,
    passport_processing::PASSPORT_PROCESSING,
    binary_boarding::BINARY_BOARDING,
    custom_customs::CUSTOM_CUSTOMS,
];

fn main() -> Result<(), Error> {
    let app = App::new("Advent of code 2020")
        .version(VERSION)
        .author("Kevin Simpson <ktsimpso@gmail.com>")
        .about("Run advent of code problems from this main program")
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let matches = COMMANDS
        .iter()
        .fold(app, |app, command| app.subcommand(command.sub_command()))
        .get_matches();

    let sub_commands: HashMap<&str, &Command> = COMMANDS
        .iter()
        .map(|command| (command.name(), command))
        .collect();

    if let (command_name, Some(args)) = matches.subcommand() {
        sub_commands
            .get(command_name)
            .ok_or_else::<Error, _>(|| SimpleError::new("No valid subcommand found").into())
            .and_then(|command| {
                println!("=============Running {:}=============", command.name());
                command.run(args)
            })
    } else {
        Err(SimpleError::new("No arguments found").into())
    }
}
