use crate::lib::{file_to_string, parse_lines_borrowed, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_until},
    character::complete,
    combinator::{all_consuming, map_parser, map_res},
    multi::{many0, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
};
use simple_error::SimpleError;
use std::str::FromStr;
use strum_macros::{EnumString, EnumVariantNames};

pub const PASSPORT_PROCESSING: Command = Command::new(sub_command, "passport-processing", run);

#[derive(Debug)]
struct PassportProcessingArgs {
    file: String,
    verify_fields: bool,
}

#[derive(Debug, EnumVariantNames, EnumString)]
enum HeightUnit {
    #[strum(serialize = "cm")]
    Centimeters,

    #[strum(serialize = "in")]
    Inches,
}

#[derive(Debug)]
struct Height {
    height: usize,
    unit: HeightUnit,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<EyeColor>,
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
        .arg(
            Arg::with_name("verify-fields")
            .short("v")
            .help(
                "When passed, verifies the field value of the passport instead of just presence."
            )
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Validates the default input but does not validate field values")
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about("Validates the default input and validates field values")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let passport_processing_arguments = match arguments.subcommand_name() {
        Some("part1") => PassportProcessingArgs {
            file: "day4/input.txt".to_string(),
            verify_fields: false,
        },
        Some("part2") => PassportProcessingArgs {
            file: "day4/input.txt".to_string(),
            verify_fields: true,
        },
        _ => PassportProcessingArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            verify_fields: arguments.is_present("verify-fields"),
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
        .and_then(|file| parse_passports(&file.to_string(), arguments.verify_fields))
        .map(|passports| {
            passports
                .into_iter()
                .filter(|passport| validate_passport(&passport))
                .count()
        })
}

fn parse_passports(file: &str, verify_fields: bool) -> Result<Vec<Passport>, Error> {
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
                    .map(|passport_candidate| parse_passport(passport_candidate, verify_fields))
                    .collect()
            })
        })
        .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse failure").into())
}

fn parse_passport(passport_candidate: Vec<(&str, &str)>, verify_fields: bool) -> Passport {
    let mut passport = Passport::new();

    passport_candidate
        .into_iter()
        .for_each(|(key, value)| match key {
            "byr" => {
                passport.byr = if verify_fields {
                    parse_byr(value)
                } else {
                    Some(0)
                }
            }
            "iyr" => {
                passport.iyr = if verify_fields {
                    parse_iyr(value)
                } else {
                    Some(0)
                }
            }
            "eyr" => {
                passport.eyr = if verify_fields {
                    parse_eyr(value)
                } else {
                    Some(0)
                }
            }
            "hgt" => {
                passport.hgt = if verify_fields {
                    parse_hgt(value)
                } else {
                    Some(Height {
                        height: 0,
                        unit: HeightUnit::Centimeters,
                    })
                }
            }
            "hcl" => {
                passport.hcl = if verify_fields {
                    parse_hcl(value)
                } else {
                    Some(value.to_string())
                }
            }
            "ecl" => {
                passport.ecl = if verify_fields {
                    parse_ecl(value)
                } else {
                    Some(EyeColor::Amb)
                }
            }
            "pid" => {
                passport.pid = if verify_fields {
                    parse_pid(value)
                } else {
                    Some(value.to_string())
                }
            }
            "cid" => passport.cid = Some(value.to_string()),
            _ => (),
        });

    passport
}

fn parse_byr(input: &str) -> Option<usize> {
    all_consuming(parse_usize)(input)
        .map(|(_, value)| value)
        .ok()
        .filter(|value| value >= &1920 && value <= &2002)
}

fn parse_iyr(input: &str) -> Option<usize> {
    all_consuming(parse_usize)(input)
        .map(|(_, value)| value)
        .ok()
        .filter(|value| value >= &2010 && value <= &2020)
}

fn parse_eyr(input: &str) -> Option<usize> {
    all_consuming(parse_usize)(input)
        .map(|(_, value)| value)
        .ok()
        .filter(|value| value >= &2020 && value <= &2030)
}

fn parse_hgt(input: &str) -> Option<Height> {
    all_consuming(tuple((
        parse_usize,
        map_res(complete::alpha1, HeightUnit::from_str),
    )))(input)
    .map_err(|_| SimpleError::new("Parse Error"))
    .map(|(_, (height, unit))| Height {
        height: height,
        unit: unit,
    })
    .ok()
    .filter(|height| match height.unit {
        HeightUnit::Centimeters => height.height >= 150 && height.height <= 193,
        HeightUnit::Inches => height.height >= 59 && height.height <= 76,
    })
}

fn parse_hcl(input: &str) -> Option<String> {
    all_consuming(preceded(
        complete::char('#'),
        map_parser(take(6usize), complete::hex_digit1),
    ))(input)
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error"))
    .map(|(_, value)| value.to_string())
    .ok()
}

fn parse_ecl(input: &str) -> Option<EyeColor> {
    all_consuming(map_res(complete::alpha1, EyeColor::from_str))(input)
        .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error"))
        .map(|(_, eye_color)| eye_color)
        .ok()
}

fn parse_pid(input: &str) -> Option<String> {
    all_consuming(map_parser(take(9usize), complete::digit1))(input)
        .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error"))
        .map(|(_, pid)| pid.to_string())
        .ok()
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
