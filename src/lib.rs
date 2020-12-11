#![feature(const_fn_fn_ptr_basics)]

use anyhow::Error;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    sequence::pair,
    IResult,
};
use simple_error::SimpleError;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Command<'a> {
    sub_command: fn() -> App<'static, 'static>,
    name: &'a str,
    run: fn(&ArgMatches) -> Result<(), Error>,
}

impl Command<'_> {
    pub const fn new<'a>(
        sub_command: fn() -> App<'static, 'static>,
        name: &'a str,
        run: fn(&ArgMatches) -> Result<(), Error>,
    ) -> Command<'a> {
        Command {
            sub_command: sub_command,
            name: name,
            run: run,
        }
    }

    pub fn sub_command(&self) -> App<'static, 'static> {
        (self.sub_command)()
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn run(&self, arguments: &ArgMatches) -> Result<(), Error> {
        (self.run)(arguments)
    }
}

pub struct SumChecker {
    base_numbers: HashMap<isize, usize>,
    unique_numbers: HashSet<isize>,
}

impl SumChecker {
    pub fn new() -> SumChecker {
        SumChecker {
            base_numbers: HashMap::new(),
            unique_numbers: HashSet::new(),
        }
    }

    pub fn with_vec(input: &Vec<isize>) -> SumChecker {
        let mut checker = SumChecker::new();

        input.into_iter().for_each(|number| {
            checker.add_number(*number);
        });
        checker
    }

    pub fn add_number(&mut self, number: isize) {
        self.unique_numbers.insert(number);
        self.base_numbers.insert(
            number,
            self.base_numbers
                .get(&number)
                .map(|count| count + 1)
                .unwrap_or(1),
        );
    }

    pub fn remove_number(&mut self, number: &isize) {
        let count = self.base_numbers.get(number).unwrap_or(&0usize).clone();

        match count {
            0 => (),
            1 => {
                self.unique_numbers.remove(number);
                self.base_numbers.remove(number);
            }
            value => {
                self.base_numbers.insert(*number, value - 1);
            }
        };
    }

    pub fn find_sum_of_n(&self, target: &isize, n: usize) -> Result<Vec<isize>, Error> {
        if n == 2 {
            self.find_sum(target)
        } else {
            (&self.unique_numbers)
                .into_iter()
                .find_map(|value| {
                    let new_target = target - value;
                    self.find_sum_of_n(&new_target, n - 1)
                        .ok()
                        .filter(|found_values| {
                            self.base_numbers.get(&value).unwrap_or(&0)
                                > &found_values
                                    .into_iter()
                                    .filter(|found_value| **found_value == *value)
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

    fn find_sum(&self, target: &isize) -> Result<Vec<isize>, Error> {
        (&self.unique_numbers)
            .into_iter()
            .find_map(|value| {
                self.base_numbers
                    .get_key_value(&(target - value))
                    .filter(|(key, count)| key != &value || count > &&1)
                    .map(|(key, _)| vec![*key, *value])
            })
            .ok_or(SimpleError::new(format!("No values found that sum to {}", target)).into())
    }
}

pub fn default_sub_command(
    command: &Command,
    about: &'static str,
    file_help: &'static str,
) -> App<'static, 'static> {
    SubCommand::with_name(command.name())
        .about(about)
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help(file_help)
                .takes_value(true)
                .required(true),
        )
}

pub fn file_to_lines(file_name: &String) -> Result<Vec<String>, Error> {
    File::open(file_name)
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
}

pub fn file_to_string(file_name: &String) -> Result<String, Error> {
    file_to_lines(file_name).map(|lines| {
        lines.into_iter().fold(String::new(), |mut acc, line| {
            acc.push_str(&line.to_string());
            acc.push('\n');
            acc
        })
    })
}

pub fn parse_lines<T, U, E, F>(lines: Vec<T>, mut parse_function: F) -> Result<Vec<U>, E>
where
    F: FnMut(&T) -> Result<U, E>,
{
    lines
        .into_iter()
        .try_fold(Vec::new(), |mut parsed_lines, line| {
            parse_function(&line).map(|parsed_line| {
                parsed_lines.push(parsed_line);
                parsed_lines
            })
        })
}

pub fn parse_lines_borrowed<T, U, E, F>(lines: Vec<T>, mut parse_function: F) -> Result<Vec<U>, E>
where
    F: FnMut(T) -> Result<U, E>,
{
    lines
        .into_iter()
        .try_fold(Vec::new(), |mut parsed_lines, line| {
            parse_function(line).map(|parsed_line| {
                parsed_lines.push(parsed_line);
                parsed_lines
            })
        })
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usisze_from_string)(input)
}

fn usisze_from_string(input: &str) -> Result<usize, Error> {
    usize::from_str_radix(input, 10).map_err(|err| err.into())
}

pub fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(pair(alt((tag("+"), tag("-"), tag(""))), digit1)),
        |value| isize::from_str_radix(value, 10),
    )(input)
}
