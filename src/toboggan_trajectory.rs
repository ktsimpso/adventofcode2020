use crate::lib::{file_to_lines, parse_lines};
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{branch::alt, character::complete, combinator::map, multi::many1};
use simple_error::SimpleError;

struct TobogganTrajectoryArgs {
    file: String,
    right: usize,
    down: usize,
}

#[derive(Debug)]
enum Terrain {
    Clear,
    Tree,
}

pub fn sub_command() -> App<'static, 'static> {
    SubCommand::with_name("toboggan-trajectory")
        .about(
            "Takes a toboggan hill and a slope an returns to number of trees that the toboggan hit",
        )
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help(
                    "Path to the input file. Input should be a toboggan hill with . denoting\
                 an empty space and # denoting a tree.",
                )
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("right")
                .short("r")
                .help("number of spaces to go right on each iteration")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("down")
                .short("d")
                .help("number of spaces to go down on each iteration")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Validates the default input with right 3, down 1")
                .version("1.0.0"),
        )
}

pub fn run(arguments: &ArgMatches) -> Result<(), Error> {
    println!("=============Running toboggan trajectory=============");

    let tobaggan_tarjectory_arguments = match arguments.subcommand_name() {
        Some("part1") => TobogganTrajectoryArgs {
            file: "day3/input.txt".to_string(),
            right: 3,
            down: 1,
        },
        _ => TobogganTrajectoryArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            right: value_t_or_exit!(arguments.value_of("right"), usize),
            down: value_t_or_exit!(arguments.value_of("down"), usize),
        },
    };

    file_to_lines(&tobaggan_tarjectory_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_toboggan_line))
        .map(|hill| {
            run_through_slope(
                &hill,
                &tobaggan_tarjectory_arguments.right,
                &tobaggan_tarjectory_arguments.down,
            )
        })
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn run_through_slope(hill: &Vec<Vec<Terrain>>, right: &usize, down: &usize) -> usize {
    let x_max = hill[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;

    loop {
        x = (x + right) % x_max;
        y = y + down;

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
