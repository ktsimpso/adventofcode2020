use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};
use nom::{branch::alt, character::complete, combinator::map, multi::many1};
use simple_error::SimpleError;

pub const SEATING_SYSTEM: Command = Command::new(sub_command, "seating-system", run);

#[derive(Debug)]
struct SeatingSystemArgs {
    file: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum FloorTile {
    Floor,
    Seat { occupied: bool },
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SEATING_SYSTEM,
        "Takes a file with a setaing chart and finds stats about people sitting",
        "Path to the input file. Each line contains one row of seats.",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Finds an equalibrium for steating arrangements and returns the number of \
                occupied seats.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let seating_system_arguments = match arguments.subcommand_name() {
        Some("part1") => SeatingSystemArgs {
            file: "day11/input.txt".to_string(),
        },
        _ => SeatingSystemArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_seat_layout(&seating_system_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_seat_layout(seating_system_arguments: &SeatingSystemArgs) -> Result<usize, Error> {
    file_to_lines(&seating_system_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_row_of_seats))
        .map(|seating_arrangement| {
            find_equalibrium(&seating_arrangement)
                .into_iter()
                .fold(0usize, |acc, row| {
                    acc + row
                        .into_iter()
                        .filter(|tile| match tile {
                            FloorTile::Seat { occupied: true } => true,
                            _ => false,
                        })
                        .count()
                })
        })
}

fn find_equalibrium(seating_arrangement: &Vec<Vec<FloorTile>>) -> Vec<Vec<FloorTile>> {
    let mut previous_arrangement = seating_arrangement.to_vec();
    loop {
        let next_arrangement = iterate_seats(&previous_arrangement);
        if next_arrangement == *previous_arrangement {
            break;
        }

        previous_arrangement = next_arrangement;
    }

    previous_arrangement
}

fn iterate_seats(seating_arrangement: &Vec<Vec<FloorTile>>) -> Vec<Vec<FloorTile>> {
    seating_arrangement
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, tile)| match tile {
                    FloorTile::Seat { occupied: true } => {
                        match count_occupided_seats(get_adjacent_tiles(&x, &y, seating_arrangement))
                        {
                            _x if _x >= 4 => FloorTile::Seat { occupied: false },
                            _ => FloorTile::Seat { occupied: true },
                        }
                    }
                    FloorTile::Seat { occupied: false } => {
                        match count_occupided_seats(get_adjacent_tiles(&x, &y, seating_arrangement))
                        {
                            0 => FloorTile::Seat { occupied: true },
                            _ => FloorTile::Seat { occupied: false },
                        }
                    }
                    FloorTile::Floor => FloorTile::Floor,
                })
                .collect()
        })
        .collect()
}

fn count_occupided_seats(seats: Vec<FloorTile>) -> usize {
    seats
        .into_iter()
        .filter(|tile| match tile {
            FloorTile::Seat { occupied: true } => true,
            _ => false,
        })
        .count()
}

fn get_adjacent_tiles(
    x: &usize,
    y: &usize,
    seating_arrangement: &Vec<Vec<FloorTile>>,
) -> Vec<FloorTile> {
    let mut result = Vec::new();
    if y > &0 {
        result.extend(find_adjacent_tiles_in_row(
            x,
            &seating_arrangement[y - 1],
            true,
        ));
    }

    result.extend(find_adjacent_tiles_in_row(
        x,
        &seating_arrangement[*y],
        false,
    ));

    if *y < seating_arrangement.len() - 1 {
        result.extend(find_adjacent_tiles_in_row(
            x,
            &seating_arrangement[y + 1],
            true,
        ));
    }

    result
}

fn find_adjacent_tiles_in_row(x: &usize, row: &Vec<FloorTile>, include_x: bool) -> Vec<FloorTile> {
    let mut result = Vec::new();
    if x > &0 {
        result.push(row[x - 1]);
    }

    if include_x {
        result.push(row[*x]);
    }

    if *x < row.len() - 1 {
        result.push(row[x + 1]);
    }

    result
}

fn parse_row_of_seats(line: &String) -> Result<Vec<FloorTile>, Error> {
    many1(alt((
        map(complete::char('.'), |_| FloorTile::Floor),
        map(complete::char('#'), |_| FloorTile::Seat { occupied: true }),
        map(complete::char('L'), |_| FloorTile::Seat { occupied: false }),
    )))(line.as_str())
    .map(|(_, seating_arrangement)| seating_arrangement)
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse failure").into())
}
