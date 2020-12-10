use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};
use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{map, map_res, rest},
    sequence::separated_pair,
};
use simple_error::SimpleError;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;
use strum_macros::{EnumString, EnumVariantNames};

pub const HANDHELD_HALTING: Command = Command::new(sub_command, "handheld-halting", run);

#[derive(Debug)]
struct HandHeldHaltingArgs {
    file: String,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum ProgramLine {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &HANDHELD_HALTING,
        "Takes a file with a simple program (that infinite loops) and finds information about it.",
        "Path to the input file. Each line contains one instruction",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about("Finds the value of the accumulator when a loop is detected with default input.")
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let handheld_halting_arguments = match arguments.subcommand_name() {
        Some("part1") => HandHeldHaltingArgs {
            file: "day8/input.txt".to_string(),
        },
        _ => HandHeldHaltingArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_program(&handheld_halting_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_program(handheld_halting_arguments: &HandHeldHaltingArgs) -> Result<isize, Error> {
    file_to_lines(&handheld_halting_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_program_line))
        .map(|program| find_acc_when_infinite(program))
}

fn find_acc_when_infinite(program: Vec<ProgramLine>) -> isize {
    let mut acc_value = 0;
    let mut visited = HashSet::new();
    let mut program_counter = 0isize;

    loop {
        if visited.contains(&program_counter) {
            break;
        }
        visited.insert(program_counter);

        match program[usize::try_from(program_counter).unwrap()] {
            ProgramLine::Acc(value) => acc_value += value,
            ProgramLine::Jmp(value) => {
                program_counter += value;
                continue;
            }
            ProgramLine::Nop(_) => (),
        }

        program_counter += 1;
    }

    acc_value
}

fn parse_program_line(line: &String) -> Result<ProgramLine, Error> {
    map(
        separated_pair(
            map_res(complete::alpha1, ProgramLine::from_str),
            tag(" "),
            map_res(rest, |value| isize::from_str_radix(value, 10)),
        ),
        |(instruction, value)| match instruction {
            ProgramLine::Acc(_) => ProgramLine::Acc(value),
            ProgramLine::Jmp(_) => ProgramLine::Jmp(value),
            ProgramLine::Nop(_) => ProgramLine::Nop(value),
        },
    )(line.as_str())
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error").into())
    .map(|(_, instruction)| instruction)
}
