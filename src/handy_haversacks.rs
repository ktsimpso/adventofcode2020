use crate::lib::{default_sub_command, file_to_lines, parse_lines, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{map, recognize},
    multi::fold_many1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use simple_error::SimpleError;
use std::collections::{HashMap, HashSet, VecDeque};

pub const HANDY_HAVERSACKS: Command = Command::new(sub_command, "handy-haversacks", run);

#[derive(Debug)]
struct HandyHaversackArgs {
    file: String,
    sack_name: String,
}

#[derive(Debug)]
struct SackRule {
    sack_name: String,
    contains: HashMap<String, usize>,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &HANDY_HAVERSACKS,
        "Takes a file with rules about bags in bags in bags and finds interesting facts",
        "Path to the input file. Each line contains the rules for a bag.",
    )
    .arg(
        Arg::with_name("sack")
            .short("s")
            .help("name of the sack you are trying to find stats on.")
            .takes_value(true)
            .required(true),
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about("Finds the number of unique starting bags which contain at least 1 gold bag")
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let handy_haversack_arguments = match arguments.subcommand_name() {
        Some("part1") => HandyHaversackArgs {
            file: "day7/input.txt".to_string(),
            sack_name: "shiny gold".to_string(),
        },
        _ => HandyHaversackArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            sack_name: value_t_or_exit!(arguments.value_of("sack"), String),
        },
    };

    process_sacks(&handy_haversack_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_sacks(handy_haversack_arguments: &HandyHaversackArgs) -> Result<usize, Error> {
    file_to_lines(&handy_haversack_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_sack_rules))
        .map(|rules| find_bags_that_contain(&handy_haversack_arguments.sack_name, rules))
}

fn find_bags_that_contain(sack_name: &String, rules: Vec<SackRule>) -> usize {
    // When you just clone all the things to make the compiler happy, sad times are to be had
    let mut reverse_lookup: HashMap<String, HashSet<String>> = HashMap::new();
    rules.into_iter().for_each(|sack_rule| {
        sack_rule.contains.keys().into_iter().for_each(|child| {
            let mut lookup = if let Some(lookup) = reverse_lookup.get(child) {
                lookup.clone()
            } else {
                HashSet::new().clone()
            };
            lookup.insert(sack_rule.sack_name.clone());
            reverse_lookup.insert(child.clone(), lookup.clone());
        })
    });

    let mut parents = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(sack_name);

    while let Some(to_lookup) = queue.pop_back() {
        if !reverse_lookup.contains_key(to_lookup) {
            continue;
        }
        let new_parents = reverse_lookup.get(to_lookup).unwrap();
        new_parents.into_iter().for_each(|parent| {
            if !parents.contains(parent) {
                parents.insert(parent);
                queue.push_back(parent);
            };
        })
    }

    parents.len()
}

fn parse_sack_rules(line: &String) -> Result<SackRule, Error> {
    map(
        tuple((
            terminated(parse_sack_name, tag(" contain ")),
            alt((
                map(tag("no other bags."), |_| HashMap::new()),
                fold_many1(
                    separated_pair(
                        parse_usize,
                        tag(" "),
                        terminated(parse_sack_name, alt((tag(", "), tag(".")))),
                    ),
                    HashMap::new(),
                    |mut acc, (count, sack_name)| {
                        acc.insert(sack_name.to_string(), count);
                        acc
                    },
                ),
            )),
        )),
        |(sack_name, contains)| SackRule {
            sack_name: sack_name.to_string(),
            contains: contains,
        },
    )(line)
    .map_err(|_| SimpleError::new("Parse Error").into())
    .map(|(_, sack_rule)| sack_rule)
}

fn parse_sack_name(input: &str) -> IResult<&str, &str> {
    terminated(
        recognize(tuple((complete::alpha1, tag(" "), complete::alpha1))),
        tuple((tag(" "), alt((tag("bags"), tag("bag"))))),
    )(input)
}
