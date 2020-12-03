extern crate clap;
use anyhow::Error;
use clap::ArgMatches;
use simple_error::SimpleError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(arguments: &ArgMatches) -> Result<(), Error> {
    println!("=============Running report repair=============");

    File::open("day1/input.txt")
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
                    line.parse::<usize>().map(|int_line| {
                        lines.push(int_line);
                        lines
                    })
                })
                .map_err(|err| err.into())
        })
        .and_then(|lines| find_sum(&2020, &lines))
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn find_sum(target: &usize, input: &Vec<usize>) -> Result<usize, Error> {
    let mut numbers = HashMap::new();

    input.into_iter().for_each(|number| {
        numbers.insert(
            number,
            numbers.get(&number).map(|count| count + 1).unwrap_or(1),
        );
    });

    input
        .into_iter()
        .find_map(|value| {
            numbers
                .get_key_value(&(target - value))
                .filter(|(key, count)| key != &&value || count > &&1)
                .map(|(key, count)| *key * value)
        })
        .ok_or(SimpleError::new(format!("No values found that sum to {}", target)).into())
}
