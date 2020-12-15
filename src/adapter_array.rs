use crate::lib::{default_sub_command, file_to_lines, parse_lines, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use simple_error::SimpleError;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const ADAPTER_ARRAY: Command = Command::new(sub_command, "adapter-array", run);

#[derive(Debug)]
struct AdapterArrayArgs {
    file: String,
    stat: JoltageStat,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum JoltageStat {
    SumOfOneAndThreeJoltageGaps,
    CombinationOfValidAdapterChains,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &ADAPTER_ARRAY,
        "Takes a file with one number on each line and find joltage stats",
        "Path to the input file. Each line contains one integer.",
    )
    .arg(
        Arg::with_name("stat")
            .short("s")
            .help("Joltage stats requested. The stats available are as follows:\n\n\
            sum-of-one-and-three-joltage-gaps: Finds the value of 1 joltage jumps and 3 joltage \
            jumps using all adapters and sums them.\n\n\
            combination-of-valid-adapter-chains: Finds the number of valid adapter combinations \
            that could power the device.\n")
            .takes_value(true)
            .possible_values(&JoltageStat::VARIANTS)
            .required(true),
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Finds the value of 1 joltage jumps and 3 joltage jumps using all adapters and sums them \
                with the default input.",
            )
            .version("1.0.0"),
    )
    .subcommand(
        SubCommand::with_name("part2")
            .about(
                "Finds the number of valid adapter combinations that could power the device \
                with the default input.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let adapter_array_arguments = match arguments.subcommand_name() {
        Some("part1") => AdapterArrayArgs {
            file: "day10/input.txt".to_string(),
            stat: JoltageStat::SumOfOneAndThreeJoltageGaps,
        },
        Some("part2") => AdapterArrayArgs {
            file: "day10/input.txt".to_string(),
            stat: JoltageStat::CombinationOfValidAdapterChains,
        },
        _ => AdapterArrayArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            stat: value_t_or_exit!(arguments.value_of("stat"), JoltageStat),
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
        .map(|adapters| match adapter_array_arguments.stat {
            JoltageStat::SumOfOneAndThreeJoltageGaps => find_and_sum_1_and_3_votage_gaps(&adapters),
            JoltageStat::CombinationOfValidAdapterChains => {
                find_number_of_unique_valid_adapter_combinations(&adapters)
            }
        })
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

fn find_number_of_unique_valid_adapter_combinations(adapters: &Vec<usize>) -> usize {
    let mut number_of_ones = 0usize;
    let mut counting_ones = false;
    let mut combinations = 1usize;

    for diff in adapters.windows(2).map(|window| window[1] - window[0]) {
        if diff == 3usize && counting_ones {
            combinations *= number_of_ways_consecutives_ones_can_be_arranged(number_of_ones);
            number_of_ones = 0usize;
            counting_ones = false;
        }

        if diff == 1usize {
            counting_ones = true;
            number_of_ones += 1;
        }
    }

    combinations
}

// Only imperically tested up to n = 5 to find a recurance relation.
// Wolfram alpha doing the heavy lifting for the closed form because
// I can't be bothered to look up how to dervive it again.
fn number_of_ways_consecutives_ones_can_be_arranged(n: usize) -> usize {
    (n * n - n + 2) / 2
}

fn parse_adapters(line: &String) -> Result<usize, Error> {
    parse_usize(line)
        .map_err(|_| SimpleError::new("Parse Error").into())
        .map(|(_, number)| number)
}
