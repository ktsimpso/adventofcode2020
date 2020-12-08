use crate::lib::{file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete,
    combinator::{map, map_parser, map_res},
    multi::fold_many1,
    sequence::tuple,
};
use simple_error::SimpleError;

pub const BINARY_BOARDING: Command = Command::new(sub_command, "binary-boarding", run);

#[derive(Debug)]
struct BinaryBoardingArgs {
    file: String,
}

#[derive(Debug)]
struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn sub_command() -> App<'static, 'static> {
    SubCommand::with_name(BINARY_BOARDING.name())
        .about("Takes a file with boarding passes and finds the highest seat id")
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Path to the input file. Input should be newline separated boarding passes.")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Validates the default input but does not validate field values")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let binary_boarding_arguments = match arguments.subcommand_name() {
        Some("part1") => BinaryBoardingArgs {
            file: "day5/input.txt".to_string(),
        },
        _ => BinaryBoardingArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_boarding_passes(&binary_boarding_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_boarding_passes(binary_boarding_arguments: &BinaryBoardingArgs) -> Result<usize, Error> {
    file_to_lines(&binary_boarding_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_boarding_pass_line))
        .map(|boarding_passes| {
            boarding_passes.into_iter().fold(0, |max, value| {
                if max > value.seat_id() {
                    max
                } else {
                    value.seat_id()
                }
            })
        })
}

fn parse_boarding_pass_line(line: &String) -> Result<BoardingPass, Error> {
    tuple((
        map_res(
            map_parser(
                take(7usize),
                fold_many1(
                    alt((
                        map(complete::char('F'), |_| "0"),
                        map(complete::char('B'), |_| "1"),
                    )),
                    String::new(),
                    |mut acc: String, digit| {
                        acc.push_str(digit);
                        acc
                    },
                ),
            ),
            |result| usize::from_str_radix(&result, 2),
        ),
        map_res(
            map_parser(
                take(3usize),
                fold_many1(
                    alt((
                        map(complete::char('L'), |_| "0"),
                        map(complete::char('R'), |_| "1"),
                    )),
                    String::new(),
                    |mut acc: String, digit| {
                        acc.push_str(digit);
                        acc
                    },
                ),
            ),
            |result| usize::from_str_radix(&result, 2),
        ),
    ))(line.as_str())
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error").into())
    .map(|(_, (row, column))| BoardingPass {
        row: row,
        column: column,
    })
}
