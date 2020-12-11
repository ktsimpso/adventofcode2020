use crate::lib::{
    default_sub_command, file_to_lines, parse_isize, parse_lines, Command, SumChecker,
};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use simple_error::SimpleError;

pub const ENCODING_ERROR: Command = Command::new(sub_command, "encoding-error", run);

#[derive(Debug)]
struct EncodingErrorArgs {
    file: String,
    preamble_length: usize,
    exploit: bool,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_command(
        &ENCODING_ERROR,
        "Takes a file with one number on each line and a preamble find the number that does not fit the 
        encoding.",
        "Path to the input file. Each line contains one integer.",
    )
    .arg(
        Arg::with_name("preamble")
            .short("p")
            .help("Length of the preamble for the XMAS protocol.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("exploit")
        .short("e")
        .help("If passed, finds the exploit number based on the number found that did not fit encoding.")
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
            the program terminates with default input. Then finds the exploit value.",
            )
            .version("1.0.0"),
    )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let encoding_error_arguments = match arguments.subcommand_name() {
        Some("part1") => EncodingErrorArgs {
            file: "day9/input.txt".to_string(),
            preamble_length: 25,
            exploit: false,
        },
        Some("part2") => EncodingErrorArgs {
            file: "day9/input.txt".to_string(),
            preamble_length: 25,
            exploit: true,
        },
        _ => EncodingErrorArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            preamble_length: value_t_or_exit!(arguments.value_of("preamble"), usize),
            exploit: arguments.is_present("exploit"),
        },
    };

    process_numbers(&encoding_error_arguments)
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn process_numbers(encoding_error_arguments: &EncodingErrorArgs) -> Result<isize, Error> {
    file_to_lines(&encoding_error_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_numbers))
        .map(|numbers| {
            let result = find_missing_number(&numbers, &encoding_error_arguments.preamble_length);

            if encoding_error_arguments.exploit {
                let exploit_range =
                    find_continous_sequence_of_at_least_two_that_sum_to_target(&result, &numbers);
                let min = (&exploit_range)
                    .into_iter()
                    .fold(
                        isize::MAX,
                        |low, number| {
                            if low < *number {
                                low
                            } else {
                                *number
                            }
                        },
                    );

                let max = (&exploit_range)
                    .into_iter()
                    .fold(
                        isize::MIN,
                        |high, number| {
                            if high > *number {
                                high
                            } else {
                                *number
                            }
                        },
                    );
                min + max
            } else {
                result
            }
        })
}

fn find_missing_number(numbers: &Vec<isize>, preamble_length: &usize) -> isize {
    *numbers
        .windows(preamble_length + 1)
        .map(|window| window.split_last().unwrap())
        .map(|(test_number, preamble)| {
            SumChecker::with_vec(&preamble.to_vec())
                .find_sum_of_n(test_number, 2)
                .map_err(|_| test_number)
        })
        .find_map(|result| result.err())
        .unwrap_or(&0)
}

fn find_continous_sequence_of_at_least_two_that_sum_to_target(
    target: &isize,
    numbers: &Vec<isize>,
) -> Vec<isize> {
    let mut low = 0;
    let mut high = 1;

    loop {
        match sum_from_low_to_high(&low, &high, numbers) {
            sum if sum > *target => low += 1,
            sum if sum < *target => high += 1,
            _ => break,
        }
    }

    numbers[low..high].to_vec()
}

fn sum_from_low_to_high(low: &usize, high: &usize, numbers: &Vec<isize>) -> isize {
    numbers[*low..*high]
        .iter()
        .fold(0, |acc, number| acc + number)
}

fn parse_numbers(line: &String) -> Result<isize, Error> {
    parse_isize(line)
        .map_err(|_| SimpleError::new("Parse Error").into())
        .map(|(_, number)| number)
}
