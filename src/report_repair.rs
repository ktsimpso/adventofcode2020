use crate::lib::{default_sub_command, file_to_lines, parse_lines, Command, SumChecker};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};

pub const REPORT_REPAIR: Command = Command::new(sub_command, "report-repair", run);

struct ReportRepairArgs {
    file: String,
    target: isize,
    number: usize,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &REPORT_REPAIR,
        "Looks through the input for n numbers that sum to target. \
    Then multiplies the result and produces the output.",
        "Path to the input file. Input should be newline delimited integers.",
    )
    .arg(
        Arg::with_name("target")
            .short("t")
            .help("Target sum to find.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("number")
            .short("n")
            .help("Number of items that must be used in the sum")
            .takes_value(true)
            .required(true),
    )
    .subcommand(
        SubCommand::with_name("part1")
            .about(
                "Searches the default input for two values that sum to 2020. \
Then multiplies the result and produces the output.",
            )
            .version("1.0.0"),
    )
    .subcommand(
        SubCommand::with_name("part2")
            .about(
                "Searches the default input for three values that sum to 2020. \
Then multiplies the result and produces the output.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let report_arguments = match arguments.subcommand_name() {
        Some("part1") => ReportRepairArgs {
            file: "day1/input.txt".to_string(),
            target: 2020,
            number: 2,
        },
        Some("part2") => ReportRepairArgs {
            file: "day1/input.txt".to_string(),
            target: 2020,
            number: 3,
        },
        _ => ReportRepairArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            target: value_t_or_exit!(arguments.value_of("target"), isize),
            number: value_t_or_exit!(arguments.value_of("number"), usize),
        },
    };

    file_to_lines(&report_arguments.file)
        .and_then(|lines| {
            parse_lines(lines, |line| line.parse::<isize>()).map_err(|err| err.into())
        })
        .and_then(|lines| {
            find_muliple_of_sum_of_n(&report_arguments.target, &lines, report_arguments.number)
        })
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn find_muliple_of_sum_of_n(target: &isize, input: &Vec<isize>, n: usize) -> Result<isize, Error> {
    SumChecker::with_vec(input)
        .find_sum_of_n(target, n)
        .map(|result| result.into_iter().fold(1, |acc, number| acc * number))
}
