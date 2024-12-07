use std::{env, process};

use aoc_2024::{Config, PuzzleInput, exercises};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let input = PuzzleInput::build(&config).unwrap_or_else(|err| {
        println!("Problem reading input: {err}");
        process::exit(1);
    });

    exercises::run(&input).unwrap_or_else( |err| {
        println!("Problem running the exercise {err} ");
        process::exit(1);
    }
    )
}
