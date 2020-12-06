extern crate clap;
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use simple_error::SimpleError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct ReportRepairArgs {
    file: String,
    target: isize,
    number: usize,
}

pub fn sub_command() -> App<'static, 'static> {
    SubCommand::with_name("report-repair")
        .about(
            "Looks through the input for n numbers that sum to target. \
Then multiplies the result and produces the output.",
        )
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Path to the input file. Input should be newline delimited integers.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .help("Target sum to find.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("Number of items that must be used in the sum")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about(
                    "Searches the default input for two values that sum to 2020. \
Then multiplies the result and produces the output.",
                )
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about(
                    "Searches the default input for three values that sum to 2020. \
Then multiplies the result and produces the output.",
                )
                .version("1.0.0"),
        )
}

pub fn run(arguments: &ArgMatches) -> Result<(), Error> {
    println!("=============Running report repair=============");

    let report_arguments = match arguments.subcommand_name() {
        Some("part1") => ReportRepairArgs {
            file: "day1/input.txt".to_string(),
            target: 2020,
            number: 2,
        },
        Some("part2") => ReportRepairArgs {
            file: "day1/input.txt".to_string(),
            target: 2020,
            number: 3,
        },
        _ => ReportRepairArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            target: value_t_or_exit!(arguments.value_of("target"), isize),
            number: value_t_or_exit!(arguments.value_of("number"), usize),
        },
    };

    File::open(&report_arguments.file)
        .map_err(|err| err.into())
        .and_then(|file| {
            BufReader::new(file)
                .lines()
                .try_fold(Vec::new(), |mut lines, line_result| {
                    line_result.map(|line| {
                        lines.push(line);
                        lines
                    })
                })
                .map_err(|err| err.into())
        })
        .and_then(|lines| {
            lines
                .into_iter()
                .try_fold(Vec::new(), |mut lines, line| {
                    line.parse::<isize>().map(|int_line| {
                        lines.push(int_line);
                        lines
                    })
                })
                .map_err(|err| err.into())
        })
        .and_then(|lines| {
            find_muliple_of_sum_of_n(&report_arguments.target, &lines, report_arguments.number)
        })
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn find_muliple_of_sum_of_n(target: &isize, input: &Vec<isize>, n: usize) -> Result<isize, Error> {
    find_sum_of_n(target, input, n)
        .map(|result| result.into_iter().fold(1, |acc, number| acc * number))
}

fn find_sum_of_n(target: &isize, input: &Vec<isize>, n: usize) -> Result<Vec<isize>, Error> {
    let numbers = build_numbers_map(input);

    if n == 2 {
        find_sum(target, input, numbers)
    } else {
        input
            .into_iter()
            .find_map(|value| {
                let new_target = target - value;
                find_sum_of_n(&new_target, input, n - 1)
                    .ok()
                    .filter(|found_values| {
                        *numbers.get(value).unwrap_or(&0)
                            > found_values
                                .into_iter()
                                .filter(|found_value| *found_value == value)
                                .count()
                    })
                    .map(|mut found_values| {
                        found_values.push(*value);
                        found_values
                    })
            })
            .ok_or(SimpleError::new(format!("No values found that sum to {}", target)).into())
    }
}

fn find_sum(
    target: &isize,
    input: &Vec<isize>,
    numbers: HashMap<&isize, usize>,
) -> Result<Vec<isize>, Error> {
    input
        .into_iter()
        .find_map(|value| {
            numbers
                .get_key_value(&(target - value))
                .filter(|(key, count)| key != &&value || count > &&1)
                .map(|(key, _count)| {
                    let mut result = Vec::new();
                    result.push(**key);
                    result.push(*value);
                    result
                })
        })
        .ok_or(SimpleError::new(format!("No values found that sum to {}", target)).into())
}

fn build_numbers_map(input: &Vec<isize>) -> HashMap<&isize, usize> {
    let mut numbers = HashMap::new();

    input.into_iter().for_each(|number| {
        numbers.insert(
            number,
            numbers.get(&number).map(|count| count + 1).unwrap_or(1),
        );
    });
    numbers
}
