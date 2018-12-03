extern crate clap;

mod aoc_utils;
mod day1_part1;


use std::io;
use clap::{Arg, SubCommand, App};

fn main() -> io::Result<()>{

    let app_name = "Advent of Code 2018 (-h, --help for help)";

    println!("{}", &app_name);

    let arg_matches = App::new(app_name.to_string())
        .version("0.1")
        .author("Per Arneng")
        .subcommand(
            SubCommand::with_name("day1-part1")
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .value_name("FILE")
                    .takes_value(true)
                    .help("the input frequencies")
                    .required(true)
                )
        ).get_matches();

        if let Some(day1_part1_matches)
            = arg_matches.subcommand_matches("day1-part1") {

            let input = day1_part1_matches.value_of("input").unwrap();
            day1_part1::start(input)?;
        }

        Ok(())
}
