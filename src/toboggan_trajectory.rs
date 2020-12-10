use crate::lib::{default_sub_command, file_to_lines, parse_lines, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, values_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    character::complete,
    combinator::map,
    multi::many1,
    sequence::{preceded, tuple},
};
use simple_error::SimpleError;
use std::str::FromStr;

pub const TOBOGGAN_TRAJECTORY: Command = Command::new(sub_command, "toboggan-trajectory", run);

struct TobogganTrajectoryArgs {
    file: String,
    slopes: Vec<Slope>,
}

struct Slope {
    right: usize,
    down: usize,
}

impl FromStr for Slope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tuple((parse_usize, preceded(complete::char(','), parse_usize)))(s)
            .map(|(_, (right, down))| Slope {
                right: right,
                down: down,
            })
            .map_err(|_| SimpleError::new("Parse failure").into())
    }
}

#[derive(Debug)]
enum Terrain {
    Clear,
    Tree,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(&TOBOGGAN_TRAJECTORY, "Takes a toboggan hill and a slope an returns the product of the number of trees \
    that the toboggan hit on each slope", "Path to the input file. Input should be a toboggan hill with . denoting\
    an empty space and # denoting a tree.")
        .arg(Arg::with_name("slope")
            .short("s")
            .help(
                    "Slope of the toboggan specified by number of right units then number of down units \
                separated by a comma. Example: 3,1",
                )
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .min_values(1),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Validates the default input with a single slope of 3,1")
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about("Validates the default input with slopes of 1,1 3,1 5,1 7,1 1,2")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let tobaggan_tarjectory_arguments = match arguments.subcommand_name() {
        Some("part1") => TobogganTrajectoryArgs {
            file: "day3/input.txt".to_string(),
            slopes: vec![Slope { right: 3, down: 1 }],
        },
        Some("part2") => TobogganTrajectoryArgs {
            file: "day3/input.txt".to_string(),
            slopes: vec![
                Slope { right: 1, down: 1 },
                Slope { right: 3, down: 1 },
                Slope { right: 5, down: 1 },
                Slope { right: 7, down: 1 },
                Slope { right: 1, down: 2 },
            ],
        },
        _ => TobogganTrajectoryArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            slopes: values_t_or_exit!(arguments.values_of("slope"), Slope),
        },
    };

    file_to_lines(&tobaggan_tarjectory_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_toboggan_line))
        .map(|hill| {
            tobaggan_tarjectory_arguments
                .slopes
                .into_iter()
                .map(|slope| run_through_slope(&hill, &slope))
                .fold(1usize, |acc, trees| acc * trees)
        })
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn run_through_slope(hill: &Vec<Vec<Terrain>>, slope: &Slope) -> usize {
    let x_max = hill[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;

    loop {
        x = (x + slope.right) % x_max;
        y = y + slope.down;

        if y >= hill.len() {
            break;
        }

        tree_count += match hill[y][x] {
            Terrain::Clear => 0,
            Terrain::Tree => 1,
        };
    }

    tree_count
}

fn parse_toboggan_line(line: &String) -> Result<Vec<Terrain>, Error> {
    many1(alt((
        map(complete::char('.'), |_| Terrain::Clear),
        map(complete::char('#'), |_| Terrain::Tree),
    )))(line.as_str())
    .map(|(_, terrain)| terrain)
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse failure").into())
}
