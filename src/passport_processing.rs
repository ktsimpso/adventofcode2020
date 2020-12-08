use crate::lib::{file_to_string, parse_lines_borrowed, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete,
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
};
use simple_error::SimpleError;

pub const PASSPORT_PROCESSING: Command = Command::new(sub_command, "passport-processing", run);

struct PassportProcessingArgs {
    file: String,
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: Option::None,
            iyr: Option::None,
            eyr: Option::None,
            hgt: Option::None,
            hcl: Option::None,
            ecl: Option::None,
            pid: Option::None,
            cid: Option::None,
        }
    }
}

fn sub_command() -> App<'static, 'static> {
    SubCommand::with_name(PASSPORT_PROCESSING.name())
        .about(
            "Takes a passport file and validates each passport within. Returns the number of valid \
            passports in the input.",
        )
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help(
                    "Path to the input file. Input should be passports seperated by a blank line. \
                    Passport fields should be key:value and seperated by a space or a newline.",
                )
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Validates the default input")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let passport_processing_arguments = match arguments.subcommand_name() {
        Some("part1") => PassportProcessingArgs {
            file: "day4/input.txt".to_string(),
        },
        _ => PassportProcessingArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_passports(&passport_processing_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_passports(arguments: &PassportProcessingArgs) -> Result<usize, Error> {
    file_to_string(&arguments.file)
        .and_then(|file| parse_passports(&file.to_string()))
        .map(|passports| {
            passports
                .into_iter()
                .filter(|passport| validate_passport(&passport))
                .count()
        })
}

fn parse_passports(file: &str) -> Result<Vec<Passport>, Error> {
    many0(terminated(take_until("\n\n"), tag("\n\n")))(file)
        .and_then(|(_, passport_entries)| {
            parse_lines_borrowed(
                passport_entries,
                separated_list1(
                    alt((complete::char(' '), complete::char('\n'))),
                    separated_pair(complete::alphanumeric1, complete::char(':'), is_not(" \n")),
                ),
            )
            .map(|parse_results| {
                parse_results
                    .into_iter()
                    .map(|(_, result)| result)
                    .map(parse_passport)
                    .collect()
            })
        })
        .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse failure").into())
}

fn parse_passport(passport_candidate: Vec<(&str, &str)>) -> Passport {
    let mut passport = Passport::new();

    passport_candidate
        .into_iter()
        .for_each(|(key, value)| match key {
            "byr" => passport.byr = Some(value.to_string()),
            "iyr" => passport.iyr = Some(value.to_string()),
            "eyr" => passport.eyr = Some(value.to_string()),
            "hgt" => passport.hgt = Some(value.to_string()),
            "hcl" => passport.hcl = Some(value.to_string()),
            "ecl" => passport.ecl = Some(value.to_string()),
            "pid" => passport.pid = Some(value.to_string()),
            "cid" => passport.cid = Some(value.to_string()),
            _ => (),
        });

    passport
}

fn validate_passport(passport: &Passport) -> bool {
    passport.byr.is_some()
        && passport.iyr.is_some()
        && passport.eyr.is_some()
        && passport.hgt.is_some()
        && passport.hcl.is_some()
        && passport.ecl.is_some()
        && passport.pid.is_some()
}
