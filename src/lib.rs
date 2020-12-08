#![feature(const_fn_fn_ptr_basics)]

use anyhow::Error;
use clap::{App, ArgMatches};
use nom::{character::complete::digit1, combinator::map_res, IResult};
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
