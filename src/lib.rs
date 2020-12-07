use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
