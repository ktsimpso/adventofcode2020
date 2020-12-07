use crate::lib::{file_to_lines, parse_lines};
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::bytes::complete::{tag, take, take_while1};
use nom::character::complete;
use nom::combinator::{map_parser, map_res};
use nom::sequence::{preceded, tuple};
use simple_error::SimpleError;

struct PasswordPhilosophyArgs {
    file: String,
}

#[derive(Debug)]
struct PasswordLine {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

pub fn sub_command() -> App<'static, 'static> {
    SubCommand::with_name("password-philosophy")
        .about(
            "Takes a list of password key/password pairs and returns the number of valid passwords.",
        )
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Path to the input file. Input should be newline delimited and each line \
                should have the form: {min_number_of_characters}-{max_number_of_characters} {charactert}: {password}")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about(
                    "Validates the default input.",
                )
                .version("1.0.0"),
        )
}

pub fn run(arguments: &ArgMatches) -> Result<(), Error> {
    println!("=============Running password philosophy=============");

    let password_philosophy_arguments = match arguments.subcommand_name() {
        Some("part1") => PasswordPhilosophyArgs {
            file: "day2/input.txt".to_string(),
        },
        _ => PasswordPhilosophyArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    file_to_lines(&password_philosophy_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_password_line))
        .map(|password_lines| {
            password_lines
                .into_iter()
                .filter(|password_line| {
                    let instances = password_line
                        .password
                        .chars()
                        .filter(|character| character == &password_line.character)
                        .count();
                    instances >= password_line.min && instances <= password_line.max
                })
                .count()
        })
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn parse_password_line(line: &String) -> Result<PasswordLine, Error> {
    tuple((
        parse_usize,
        preceded(complete::char('-'), parse_usize),
        preceded(
            complete::char(' '),
            map_parser(take(1usize), complete::anychar),
        ),
        preceded(tag(": "), take_while1(|_| true)),
    ))(line)
    .map(|(_, (min, max, character, password))| PasswordLine {
        min: min,
        max: max,
        character: character,
        password: password.to_string(),
    })
    .map_err(|_| SimpleError::new("Parse failure").into())
}

fn parse_usize(input: &str) -> nom::IResult<&str, usize> {
    map_res(complete::digit1, usisze_from_string)(input)
}

fn usisze_from_string(input: &str) -> Result<usize, Error> {
    usize::from_str_radix(input, 10).map_err(|err| err.into())
}
