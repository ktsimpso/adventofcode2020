use crate::lib::{default_sub_commnad, file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete,
    combinator::{map, map_parser, map_res},
    multi::fold_many1,
    sequence::tuple,
};
use simple_error::SimpleError;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const BINARY_BOARDING: Command = Command::new(sub_command, "binary-boarding", run);

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum BoardingIdStategy {
    HighestInList,
    MissingFromList,
}

#[derive(Debug)]
struct BinaryBoardingArgs {
    file: String,
    strategy: BoardingIdStategy,
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
    default_sub_commnad(&BINARY_BOARDING, "Takes a file with boarding passes and finds the highest seat id", "Path to the input file. Input should be newline separated boarding passes.")
        .arg(
            Arg::with_name("strategy")
                .short("s")
                .help(
                    "What strategy to use when finding the boarding id.\n\n\
                highest-in-list: Finds the highest boarding id in the list\n\
                missing-from-list: searching for the missing boarding id or 0 if many missing ids\n",
                )
                .takes_value(true)
                .possible_values(&BoardingIdStategy::VARIANTS)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Finds the highest boarding id from the default input")
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about("Finds the missing boarding id from the default input")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let binary_boarding_arguments = match arguments.subcommand_name() {
        Some("part1") => BinaryBoardingArgs {
            file: "day5/input.txt".to_string(),
            strategy: BoardingIdStategy::HighestInList,
        },
        Some("part2") => BinaryBoardingArgs {
            file: "day5/input.txt".to_string(),
            strategy: BoardingIdStategy::MissingFromList,
        },
        _ => BinaryBoardingArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            strategy: value_t_or_exit!(arguments.value_of("strategy"), BoardingIdStategy),
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
        .map(|boarding_passes| match binary_boarding_arguments.strategy {
            BoardingIdStategy::HighestInList => find_highest_boarding_id(boarding_passes),
            BoardingIdStategy::MissingFromList => find_missing_boarding_id(boarding_passes),
        })
}

fn find_highest_boarding_id(boarding_passes: Vec<BoardingPass>) -> usize {
    boarding_passes.into_iter().fold(0, |max, value| {
        if max > value.seat_id() {
            max
        } else {
            value.seat_id()
        }
    })
}

fn find_missing_boarding_id(boarding_passes: Vec<BoardingPass>) -> usize {
    let mut boarding_ids: Vec<usize> = boarding_passes
        .into_iter()
        .map(|boarding_pass| boarding_pass.seat_id())
        .collect();

    boarding_ids.sort();
    boarding_ids
        .windows(2)
        .map(|window| (window[0], window[1]))
        .find(|(low, high)| (low + 2) == *high)
        .map(|(low, _)| low + 1)
        .unwrap_or(0)
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
