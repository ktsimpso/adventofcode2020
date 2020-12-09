use crate::lib::{default_sub_commnad, file_to_lines, parse_lines, parse_usize, Command};
use anyhow::Error;
use clap::{value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use nom::{
    bytes::complete::{tag, take, take_while1},
    character::complete,
    combinator::map_parser,
    sequence::{preceded, tuple},
};
use simple_error::SimpleError;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

pub const PASSWORD_PHILOSOPHY: Command = Command::new(sub_command, "password-philosophy", run);

struct PasswordPhilosophyArgs {
    file: String,
    password_policy: PasswordPolicy,
}

#[derive(Debug)]
struct PasswordLine {
    first: usize,
    second: usize,
    character: char,
    password: String,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum PasswordPolicy {
    RequiredCount,
    RequiredPositions,
}

fn sub_command() -> App<'static, 'static> {
    default_sub_commnad(&PASSWORD_PHILOSOPHY, "Takes a list of password key/password pairs and returns the number of valid passwords.",
    "Path to the input file. Input should be newline delimited and each line \
    should have the form: {unsigned int}-{unsigned int} {character}: {password}")
        .arg(
            Arg::with_name("policy")
                .short("p")
                .help("Password policy to use to validate the password. Valid policies are as follows:\n\n\
                required-count: The first {unsigned int} is the minimum number of {character} required in \
                the password. While the second {unsigned int} is the maximum.\n\n\
                required-positions: Each {unsigned int} is a 1 based index where exactly one of those indexes \
                contains the {character}.")
                .takes_value(true)
                .possible_values(&PasswordPolicy::VARIANTS)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("part1")
                .about(
                    "Validates the default input with the required-count policy",
                )
                .version("1.0.0"),
        )
        .subcommand(
            SubCommand::with_name("part2")
                .about(
                    "Validates the default input with the required-positions policy",
                )
                .version("1.0.0"),
        )
}

fn run(arguments: &ArgMatches) -> Result<(), Error> {
    let password_philosophy_arguments = match arguments.subcommand_name() {
        Some("part1") => PasswordPhilosophyArgs {
            file: "day2/input.txt".to_string(),
            password_policy: PasswordPolicy::RequiredCount,
        },
        Some("part2") => PasswordPhilosophyArgs {
            file: "day2/input.txt".to_string(),
            password_policy: PasswordPolicy::RequiredPositions,
        },
        _ => PasswordPhilosophyArgs {
            file: value_t_or_exit!(arguments.value_of("file"), String),
            password_policy: value_t_or_exit!(arguments.value_of("policy"), PasswordPolicy),
        },
    };

    let password_validator = match password_philosophy_arguments.password_policy {
        PasswordPolicy::RequiredCount => is_min_max_char_password_valid,
        PasswordPolicy::RequiredPositions => is_position_char_password_valid,
    };

    file_to_lines(&password_philosophy_arguments.file)
        .and_then(|lines| parse_lines(lines, parse_password_line))
        .map(|password_lines| {
            password_lines
                .into_iter()
                .filter(password_validator)
                .count()
        })
        .map(|result| {
            println!("{:#?}", result);
        })
        .map(|_| ())
}

fn is_min_max_char_password_valid(password_line: &PasswordLine) -> bool {
    let instances = password_line
        .password
        .chars()
        .filter(|character| character == &password_line.character)
        .count();
    instances >= password_line.first && instances <= password_line.second
}

fn is_position_char_password_valid(password_line: &PasswordLine) -> bool {
    password_line
        .password
        .chars()
        .enumerate()
        .filter(|(index, character)| {
            let one_index = index + 1;
            (one_index == password_line.first || one_index == password_line.second)
                && character == &password_line.character
        })
        .count()
        == 1
}

fn parse_password_line(line: &String) -> Result<PasswordLine, Error> {
    tuple((
        parse_usize,
        preceded(complete::char('-'), parse_usize),
        preceded(
            complete::char(' '),
            map_parser(take(1usize), complete::anychar),
        ),
        preceded(tag(": "), take_while1(|_| true)),
    ))(line)
    .map(|(_, (first, second, character, password))| PasswordLine {
        first: first,
        second: second,
        character: character,
        password: password.to_string(),
    })
    .map_err(|_| SimpleError::new("Parse failure").into())
}
