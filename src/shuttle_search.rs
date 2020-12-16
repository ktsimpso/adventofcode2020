use crate::lib::{default_sub_command, file_to_string, parse_isize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{terminated, tuple},
};
use simple_error::SimpleError;

pub const SHUTTLE_SEARCH: Command = Command::new(sub_command, "shuttle-search", run);

#[derive(Debug)]
struct ShuttleSearchArgs {
    file: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BusRoute {
    Bus(isize),
    // I'm *sure* this will come up later
    X,
}

#[derive(Debug, Clone)]
struct BusSchedule {
    depart_time: isize,
    routes: Vec<BusRoute>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &SHUTTLE_SEARCH,
        "Takes a file with a target time and bus schedule then finds the next bus and multiplies that by \
        the wait time.",
        "Path to the input file. First line contains the target time. Next line contains the comma \
        delimited bus schedule.",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Finds the next bus information with the default input.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let shuttle_search_arguments = match arguments.subcommand_name() {
        Some("part1") => ShuttleSearchArgs {
            file: "day13/input.txt".to_string(),
        },
        _ => ShuttleSearchArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_schedule(&shuttle_search_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_schedule(shuttle_search_arguments: &ShuttleSearchArgs) -> Result<isize, Error> {
    file_to_string(&shuttle_search_arguments.file)
        .and_then(|file| parse_schedule(&file))
        .map(|schedule| {
            let (bus_number, depart_time) = find_next_bus(&schedule);
            (depart_time - schedule.depart_time) * bus_number
        })
}

fn find_next_bus(schedule: &BusSchedule) -> (isize, isize) {
    schedule
        .routes
        .clone()
        .into_iter()
        .filter_map(|bus_route| match bus_route {
            BusRoute::Bus(x) => Some(x),
            BusRoute::X => None,
        })
        .map(|bus_number| {
            let depart_time =
                schedule.depart_time - ((schedule.depart_time % bus_number) - bus_number);
            (bus_number, depart_time)
        })
        .fold_first(|low, new| if new.1 < low.1 { new } else { low })
        .unwrap()
}

fn parse_schedule(file: &String) -> Result<BusSchedule, Error> {
    map(
        tuple((
            terminated(parse_isize, tag("\n")),
            terminated(
                separated_list1(
                    tag(","),
                    alt((
                        map(parse_isize, |bus_number| BusRoute::Bus(bus_number)),
                        map(tag("x"), |_| BusRoute::X),
                    )),
                ),
                tag("\n"),
            ),
        )),
        |(depart_time, routes)| BusSchedule {
            depart_time: depart_time,
            routes: routes,
        },
    )(file)
    .map_err(|_| SimpleError::new("Parse failure").into())
    .map(|(_, bus_schedule)| bus_schedule)
}
