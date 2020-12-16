use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{branch::alt, character::complete, combinator::map, multi::many1};
use simple_error::SimpleError;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const SEATING_SYSTEM: Command = Command::new(sub_command, "seating-system", run);

#[derive(Debug)]
struct SeatingSystemArgs {
    file: String,
    tolerance: usize,
    adjacency_definition: AdjacencyDefinition,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum FloorTile {
    Floor,
    Seat { occupied: bool },
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum AdjacencyDefinition {
    DirectlyNextTo,
    LineOfSight,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SEATING_SYSTEM,
        "Takes a file with a seating chart and finds stats about people sitting",
        "Path to the input file. Each line contains one row of seats.",
    )
    .arg(
        Arg::with_name("tolerance")
            .short("t")
            .help("The amount of adjacent seats people are willing to sit beside before leaving")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("adjacency")
            .short("a")
            .help(
                "The definition of adjaceny. The possible definitions are as follows:\n\n\
            directly-next-to: Tiles directly next to the target tile are adjacent.\n\n\
            line-of-sight: The first Seat tile in a direct is adjacent.\n",
            )
            .takes_value(true)
            .possible_values(&AdjacencyDefinition::VARIANTS)
            .required(true),
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
    .subcommand(
        SubCommand::with_name("part2")
            .about(
                "Finds an equalibrium for steating arrangements with a tolerance of 5, and \
                adjacency of line-of-sight and then returns the number of occupied seats \
                with the default input.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let seating_system_arguments = match arguments.subcommand_name() {
        Some("part1") => SeatingSystemArgs {
            file: "day11/input.txt".to_string(),
            tolerance: 4,
            adjacency_definition: AdjacencyDefinition::DirectlyNextTo,
        },
        Some("part2") => SeatingSystemArgs {
            file: "day11/input.txt".to_string(),
            tolerance: 5,
            adjacency_definition: AdjacencyDefinition::LineOfSight,
        },
        _ => SeatingSystemArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            tolerance: value_t_or_exit!(arguments.value_of("tolerance"), usize),
            adjacency_definition: value_t_or_exit!(
                arguments.value_of("adjacency"),
                AdjacencyDefinition
            ),
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
            find_equalibrium(
                &seating_arrangement,
                &seating_system_arguments.tolerance,
                &seating_system_arguments.adjacency_definition,
            )
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

fn find_equalibrium(
    seating_arrangement: &Vec<Vec<FloorTile>>,
    tolerance: &usize,
    adjacency_definition: &AdjacencyDefinition,
) -> Vec<Vec<FloorTile>> {
    let mut previous_arrangement = seating_arrangement.to_vec();
    loop {
        let next_arrangement =
            iterate_seats(&previous_arrangement, tolerance, adjacency_definition);
        if next_arrangement == *previous_arrangement {
            break;
        }

        previous_arrangement = next_arrangement;
    }

    previous_arrangement
}

fn iterate_seats(
    seating_arrangement: &Vec<Vec<FloorTile>>,
    tolerance: &usize,
    adjacency_definition: &AdjacencyDefinition,
) -> Vec<Vec<FloorTile>> {
    seating_arrangement
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, tile)| match tile {
                    FloorTile::Seat { occupied: true } => {
                        let adjacent_tiles = match adjacency_definition {
                            AdjacencyDefinition::DirectlyNextTo => {
                                get_adjacent_tiles(&x, &y, seating_arrangement)
                            }
                            AdjacencyDefinition::LineOfSight => {
                                get_line_of_sight_seats(&x, &y, seating_arrangement)
                            }
                        };
                        match count_occupided_seats(adjacent_tiles) {
                            _x if _x >= *tolerance => FloorTile::Seat { occupied: false },
                            _ => FloorTile::Seat { occupied: true },
                        }
                    }
                    FloorTile::Seat { occupied: false } => {
                        let adjacent_tiles = match adjacency_definition {
                            AdjacencyDefinition::DirectlyNextTo => {
                                get_adjacent_tiles(&x, &y, seating_arrangement)
                            }
                            AdjacencyDefinition::LineOfSight => {
                                get_line_of_sight_seats(&x, &y, seating_arrangement)
                            }
                        };
                        match count_occupided_seats(adjacent_tiles) {
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

fn get_line_of_sight_seats(
    x: &usize,
    y: &usize,
    seating_arrangement: &Vec<Vec<FloorTile>>,
) -> Vec<FloorTile> {
    let mut result = Vec::new();
    let y_max = seating_arrangement.len() - 1;
    let x_max = seating_arrangement[0].len() - 1;

    // up left
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::Some(0),
        Option::Some(0),
        &std::ops::Sub::sub,
        &std::ops::Sub::sub,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // up
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::None,
        Option::Some(0),
        &std::ops::Mul::mul,
        &std::ops::Sub::sub,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // up right
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::Some(x_max),
        Option::Some(0),
        &std::ops::Add::add,
        &std::ops::Sub::sub,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // left
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::Some(0),
        Option::None,
        &std::ops::Sub::sub,
        &std::ops::Mul::mul,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // right
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::Some(x_max),
        Option::None,
        &std::ops::Add::add,
        &std::ops::Mul::mul,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // left down
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::Some(0),
        Option::Some(y_max),
        &std::ops::Sub::sub,
        &std::ops::Add::add,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // down
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::None,
        Option::Some(y_max),
        &std::ops::Mul::mul,
        &std::ops::Add::add,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    // down right
    traverse_until_seat(
        x,
        y,
        seating_arrangement,
        Option::Some(x_max),
        Option::Some(y_max),
        &std::ops::Add::add,
        &std::ops::Add::add,
    )
    .iter()
    .for_each(|tile| result.push(*tile));

    result
}

fn traverse_until_seat(
    x: &usize,
    y: &usize,
    seating_arrangement: &Vec<Vec<FloorTile>>,
    x_stop: Option<usize>,
    y_stop: Option<usize>,
    x_move: &dyn Fn(usize, usize) -> usize,
    y_move: &dyn Fn(usize, usize) -> usize,
) -> Option<FloorTile> {
    let mut current_x = *x;
    let mut current_y = *y;

    loop {
        if x_stop.map(|stop| current_x == stop).unwrap_or(false)
            || y_stop.map(|stop| current_y == stop).unwrap_or(false)
        {
            break;
        }

        current_x = x_move(current_x, 1);
        current_y = y_move(current_y, 1);

        match seating_arrangement[current_y][current_x] {
            FloorTile::Seat { occupied: _ } => {
                return Option::Some(seating_arrangement[current_y][current_x]);
            }
            FloorTile::Floor => (),
        }
    }

    Option::None
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
