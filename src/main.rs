extern crate clap;

mod aoc_utils;
mod day1;
mod day2;

use std::io;
use clap::{Arg, SubCommand, App};

fn sub_command_with_input<'a>(name:&'a str, input_desc:&'a str) -> App<'a,'a> {
    SubCommand::with_name(name)
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .takes_value(true)
            .help(input_desc)
            .required(true)
        ) as App
}

fn main() -> io::Result<()>{

    let app_name = "Advent of Code 2018 (-h, --help for help)";

    println!("{}", &app_name);

    let arg_matches = App::new(app_name.to_string())
        .version("0.1")
        .author("Per Arneng")
        .subcommand(
            sub_command_with_input("day1-part1", "the input frequencies")
        ).subcommand(
            sub_command_with_input("day1-part2", "the input frequencies")

        ).subcommand(
        sub_command_with_input("day2-part1", "the input id's")
        ).subcommand(
        sub_command_with_input("day2-part2", "the input id's")
        ).get_matches();

        if let Some(day1_part1_matches)
            = arg_matches.subcommand_matches("day1-part1") {

            let input = day1_part1_matches.value_of("input").unwrap();
            day1::part1::start(input)?;
        }

        if let Some(day1_part2_matches)
            = arg_matches.subcommand_matches("day1-part2") {

            let input = day1_part2_matches.value_of("input").unwrap();
            day1::part2::start(input)?;
        }

        if let Some(day2_part1_matches)
            = arg_matches.subcommand_matches("day2-part1") {

            let input = day2_part1_matches.value_of("input").unwrap();
            day2::part1::start(input)?;
        }

        if let Some(day2_part2_matches)
            = arg_matches.subcommand_matches("day2-part2") {

            let input = day2_part2_matches.value_of("input").unwrap();
            day2::part2::start(input)?;
        }

        Ok(())
}
