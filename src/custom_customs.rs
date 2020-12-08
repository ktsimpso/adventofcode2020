use crate::lib::{file_to_string, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, AppSettings, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_until},
    combinator::{map, map_parser},
    multi::{fold_many1, separated_list0},
};
use simple_error::SimpleError;
use std::collections::HashSet;

pub const CUSTOM_CUSTOMS: Command = Command::new(sub_command, "custom-customs", run);

#[derive(Debug)]
struct CustomCustomsArgs {
    file: String,
}

fn sub_command() -> App<'static, 'static> {
    SubCommand::with_name(CUSTOM_CUSTOMS.name())
        .about("Takes a file with customs questions groups and cacluates the sum of unique per group questions")
        .version("1.0.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Path to the input file. Groups are separated by a blank line, people within a group are \
                separated by a newline.")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Finds the sum of unique group answers with the default input")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let custom_customs_arguments = match arguments.subcommand_name() {
        Some("part1") => CustomCustomsArgs {
            file: "day6/input.txt".to_string(),
        },
        _ => CustomCustomsArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_customs_forms(&custom_customs_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_customs_forms(custom_customs_arguments: &CustomCustomsArgs) -> Result<usize, Error> {
    file_to_string(&custom_customs_arguments.file)
        .and_then(|file| parse_customs_forms(&file))
        .map(|customs_forms| {
            customs_forms
                .into_iter()
                .map(|form| form.len())
                .fold(0, |acc, answers| acc + answers)
        })
}

fn parse_customs_forms(file: &String) -> Result<Vec<HashSet<char>>, Error> {
    separated_list0(
        tag("\n\n"),
        map_parser(
            alt((take_until("\n\n"), take_till1(|_| false))),
            map(
                fold_many1(take(1usize), String::new(), |mut acc: String, item| {
                    if item != "\n" {
                        acc.push_str(item);
                    };
                    acc
                }),
                |answers| {
                    let mut unique_answers = HashSet::new();
                    answers.chars().for_each(|character| {
                        unique_answers.insert(character);
                    });
                    unique_answers
                },
            ),
        ),
    )(file.as_str())
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error").into())
    .map(|(_, custom_forms)| custom_forms)
}
