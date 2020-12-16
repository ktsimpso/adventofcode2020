use crate::lib::{default_sub_command, file_to_lines, parse_isize, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};
use nom::{character::complete, combinator::map_res, sequence::tuple};
use simple_error::SimpleError;
use std::collections::HashMap;
use std::convert::TryFrom;

pub const RAIN_RISK: Command = Command::new(sub_command, "rain-risk", run);

#[derive(Debug)]
struct RainRiskArgs {
    file: String,
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
enum Direction {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Left(usize),
    Right(usize),
    Forward(isize),
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &RAIN_RISK,
        "Takes a file with a seating chart and finds stats about people sitting",
        "Path to the input file. Each line contains one row of seats.",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Finds an equalibrium for steating arrangements with a tolerance of 4, and \
                adjacency of directly-next-to and then returns the number of occupied seats \
                with the default input.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let rain_risk_arguments = match arguments.subcommand_name() {
        Some("part1") => RainRiskArgs {
            file: "day12/input.txt".to_string(),
        },
        _ => RainRiskArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_directions(&rain_risk_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_directions(rain_risk_arguments: &RainRiskArgs) -> Result<isize, Error> {
    file_to_lines(&rain_risk_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_directions))
        .map(|directions| {
            let (x, y) = travel_directions(&directions);
            x.abs() + y.abs()
        })
}

fn travel_directions(directions: &Vec<Direction>) -> (isize, isize) {
    let mut point = (0, 0);
    let mut current_direction = Direction::East(0);
    let index_direction: HashMap<usize, Direction> = [
        Direction::North(0),
        Direction::East(0),
        Direction::South(0),
        Direction::West(0),
    ]
    .iter()
    .enumerate()
    .map(|(index, direction)| (index, *direction))
    .collect();
    let direction_index: HashMap<Direction, usize> = index_direction
        .clone()
        .into_iter()
        .map(|(index, direction)| (direction, index))
        .collect();

    for direction in directions {
        match direction {
            Direction::North(x) => point.0 += x,
            Direction::East(x) => point.1 += x,
            Direction::South(x) => point.0 -= x,
            Direction::West(x) => point.1 -= x,
            Direction::Right(x) => {
                current_direction = *index_direction
                    .get(&((direction_index.get(&current_direction).unwrap() + (x / 90)) % 4))
                    .unwrap()
            }
            Direction::Left(x) => {
                current_direction = *index_direction
                    .get(&((direction_index.get(&current_direction).unwrap() + 4 - (x / 90)) % 4))
                    .unwrap()
            }
            Direction::Forward(x) => match current_direction {
                Direction::North(_) => point.0 += x,
                Direction::East(_) => point.1 += x,
                Direction::South(_) => point.0 -= x,
                Direction::West(_) => point.1 -= x,
                _ => (),
            },
        }
    }

    point
}

fn parse_directions(line: &String) -> Result<Direction, Error> {
    map_res(
        tuple((complete::alpha1, parse_isize)),
        |(direction, value)| match direction {
            "N" => Ok(Direction::North(value)),
            "E" => Ok(Direction::East(value)),
            "S" => Ok(Direction::South(value)),
            "W" => Ok(Direction::West(value)),
            "R" => usize::try_from(value)
                .map(|value| Direction::Right(value))
                .map_err(|_| SimpleError::new("Could not convert to isize")),
            "L" => usize::try_from(value)
                .map(|value| Direction::Left(value))
                .map_err(|_| SimpleError::new("Could not convert to isize")),
            "F" => Ok(Direction::Forward(value)),
            x => Err(SimpleError::new(format!("Unkonwn direction {:#?}", x))),
        },
    )(line)
    .map(|(_, direction)| direction)
    .map_err(|_| SimpleError::new("Parse failure").into())
}
