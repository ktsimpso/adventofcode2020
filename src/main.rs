mod lib;
mod password_philosophy;
mod report_repair;
mod toboggan_trajectory;
use anyhow::Error;
use clap::{App, AppSettings};
use simple_error::SimpleError;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    let matches = App::new("Advent of code 2020")
        .version(VERSION)
        .author("Kevin Simpson <ktsimpso@gmail.com>")
        .about("Run advent of code problems from this main program")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(report_repair::sub_command())
        .subcommand(password_philosophy::sub_command())
        .subcommand(toboggan_trajectory::sub_command())
        .get_matches();

    match matches.subcommand() {
        ("report-repair", Some(args)) => report_repair::run(args),
        ("password-philosophy", Some(args)) => password_philosophy::run(args),
        ("toboggan-trajectory", Some(args)) => toboggan_trajectory::run(args),
        _ => Err(SimpleError::new("No valid subcommand found").into()),
    }
}
