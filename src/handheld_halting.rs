use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
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
    modify: bool,
}

#[derive(Debug, EnumString, EnumVariantNames, Clone)]
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
    .arg(
        Arg::with_name("modify")
            .short("m")
            .help("When passed, attempts to modify the input program to remove infinite loop"),
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Finds the value of the accumulator when a loop is detected, or when \
            the program terminates with default input.",
            )
            .version("1.0.0"),
    )
    .subcommand(
        SubCommand::with_name("part2")
            .about(
                "Finds the value of the accumulator when a loop is detected, or when \
            the program terminates with default input, but attempts to correct the program.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let handheld_halting_arguments = match arguments.subcommand_name() {
        Some("part1") => HandHeldHaltingArgs {
            file: "day8/input.txt".to_string(),
            modify: false,
        },
        Some("part2") => HandHeldHaltingArgs {
            file: "day8/input.txt".to_string(),
            modify: true,
        },
        _ => HandHeldHaltingArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            modify: arguments.is_present("modify"),
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
        .map(|program| {
            let result = find_acc_when_infinite(&program);

            if !handheld_halting_arguments.modify {
                match result {
                    Ok(value) => value,
                    Err(value) => value,
                }
            } else {
                match result {
                    Ok(value) => return value,
                    Err(_) => (),
                };

                for (index, instruction) in program.clone().into_iter().enumerate() {
                    let mut new_program = program.clone();
                    match instruction {
                        ProgramLine::Acc(_) => continue,
                        ProgramLine::Jmp(value) => new_program[index] = ProgramLine::Nop(value),
                        ProgramLine::Nop(value) => new_program[index] = ProgramLine::Jmp(value),
                    }
                    match find_acc_when_infinite(&new_program) {
                        Ok(value) => return value,
                        Err(_) => (),
                    }
                }

                0
            }
        })
}

fn find_acc_when_infinite(program: &Vec<ProgramLine>) -> Result<isize, isize> {
    let mut acc_value = 0;
    let mut visited = HashSet::new();
    let mut program_counter = 0isize;
    let size = isize::try_from(program.len()).unwrap();

    while program_counter < size {
        if visited.contains(&program_counter) {
            return Err(acc_value);
        }
        visited.insert(program_counter);

        let index = match usize::try_from(program_counter) {
            Ok(index) => index,
            Err(_) => return Err(acc_value),
        };

        match program[index] {
            ProgramLine::Acc(value) => acc_value += value,
            ProgramLine::Jmp(value) => {
                program_counter += value;
                continue;
            }
            ProgramLine::Nop(_) => (),
        }

        program_counter += 1;
    }

    Ok(acc_value)
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
