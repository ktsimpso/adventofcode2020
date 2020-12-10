#![feature(iterator_fold_self)]

use crate::lib::{default_sub_command, file_to_string, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_until},
    combinator::{map, map_parser},
    multi::{fold_many1, separated_list0, separated_list1},
};
use simple_error::SimpleError;
use std::collections::HashSet;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const CUSTOM_CUSTOMS: Command = Command::new(sub_command, "custom-customs", run);

#[derive(Debug)]
struct CustomCustomsArgs {
    file: String,
    strategy: CustomsCountStrategy,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum CustomsCountStrategy {
    CountUniquePerGroup,
    CountIntersectionPerGroup,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(&CUSTOM_CUSTOMS, "Takes a file with customs questions groups and cacluates the sum of unique per group questions", "Path to the input file. Groups are separated by a blank line, people within a group are \
    separated by a newline.")
        .arg(
            Arg::with_name("strategy")
                .short("s")
                .help("Counting strategy for each group. The strategies are as follows:\n\n\
                count-unique-per-group: Counts the number of unqiue anwers within a group\n\n\
                count-intersection-per-group: Count the number of questions that all members \
                of a group answered.\n")
                .takes_value(true)
                .possible_values(&CustomsCountStrategy::VARIANTS)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about("Finds the sum of unique group answers with the default input")
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about("Finds the sum of answers all group members completed with the default input")
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let custom_customs_arguments = match arguments.subcommand_name() {
        Some("part1") => CustomCustomsArgs {
            file: "day6/input.txt".to_string(),
            strategy: CustomsCountStrategy::CountUniquePerGroup,
        },
        Some("part2") => CustomCustomsArgs {
            file: "day6/input.txt".to_string(),
            strategy: CustomsCountStrategy::CountIntersectionPerGroup,
        },
        _ => CustomCustomsArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            strategy: value_t_or_exit!(arguments.value_of("strategy"), CustomsCountStrategy),
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
        .map(|customs_forms| match custom_customs_arguments.strategy {
            CustomsCountStrategy::CountUniquePerGroup => {
                count_unique_answers_per_group(customs_forms)
            }
            CustomsCountStrategy::CountIntersectionPerGroup => {
                count_answers_all_group_members_answered(customs_forms)
            }
        })
}

fn count_unique_answers_per_group(customs_forms: Vec<Vec<HashSet<char>>>) -> usize {
    customs_forms
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold(HashSet::new(), |mut acc: HashSet<char>, person| {
                    acc.extend(&person);
                    acc
                })
                .len()
        })
        .fold(0, |acc, answers| acc + answers)
}

fn count_answers_all_group_members_answered(customs_forms: Vec<Vec<HashSet<char>>>) -> usize {
    customs_forms
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .fold_first(|acc: HashSet<char>, person| {
                    acc.into_iter()
                        .filter(|answer| person.contains(answer))
                        .collect()
                })
                .map(|questions| questions.len())
                .unwrap_or(0)
        })
        .fold(0, |acc, answers| acc + answers)
}

fn parse_customs_forms(file: &String) -> Result<Vec<Vec<HashSet<char>>>, Error> {
    separated_list0(
        tag("\n\n"),
        map_parser(
            alt((take_until("\n\n"), take_till1(|_| false))),
            separated_list1(
                tag("\n"),
                map_parser(
                    alt((take_until("\n"), take_till1(|_| false))),
                    map(
                        fold_many1(take(1usize), String::new(), |mut acc: String, item| {
                            acc.push_str(item);
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
            ),
        ),
    )(file.as_str())
    .map_err(|_: nom::Err<nom::error::Error<&str>>| SimpleError::new("Parse Error").into())
    .map(|(_, custom_forms)| custom_forms)
}
