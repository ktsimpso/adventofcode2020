extern crate clap;
mod report_repair;
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
        .get_matches();

    matches
        .subcommand_matches("report-repair")
        .ok_or(SimpleError::new("report-repair was not the sub command").into())
        .and_then(report_repair::run)
}
