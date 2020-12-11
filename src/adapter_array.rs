use crate::lib::{default_sub_command, file_to_lines, parse_lines, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, ArgMatches, SubCommand};
use simple_error::SimpleError;

pub const ADAPTER_ARRAY: Command = Command::new(sub_command, "adapter-array", run);

#[derive(Debug)]
struct AdapterArrayArgs {
    file: String,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &ADAPTER_ARRAY,
        "Takes a file with one number on each line and finfd joltage stats",
        "Path to the input file. Each line contains one integer.",
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Finds the value of 1 joltage jumps and 3 joltage jumps using all adapters and sums them \
                with the default input.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let adapter_array_arguments = match arguments.subcommand_name() {
        Some("part1") => AdapterArrayArgs {
            file: "day10/input.txt".to_string(),
        },
        _ => AdapterArrayArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
        },
    };

    process_adapters(&adapter_array_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_adapters(adapter_array_arguments: &AdapterArrayArgs) -> Result<usize, Error> {
    file_to_lines(&adapter_array_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_adapters))
        .map(|mut adapters| {
            adapters.push(0usize);
            let max = (*adapters
                .iter()
                .fold_first(|max, adapter| if max > adapter { max } else { adapter })
                .unwrap())
            .clone();
            adapters.push(max + 3);
            adapters.sort();
            adapters
        })
        .map(|adapters| find_and_sum_1_and_3_votage_gaps(&adapters))
}

fn find_and_sum_1_and_3_votage_gaps(adapters: &Vec<usize>) -> usize {
    let ones = adapters
        .windows(2)
        .filter(|window| window[1] - window[0] == 1usize)
        .count();
    let threes = adapters
        .windows(2)
        .filter(|window| window[1] - window[0] == 3usize)
        .count();

    ones * threes
}

fn parse_adapters(line: &String) -> Result<usize, Error> {
    parse_usize(line)
        .map_err(|_| SimpleError::new("Parse Error").into())
        .map(|(_, number)| number)
}
