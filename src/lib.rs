use std::fs;
use std::error::Error;

pub mod exercises;

pub struct Config {
    day: String,
    iteration: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let day = args[1].clone();
        let iteration = args[2].clone();

        Ok(Config { day, iteration })
    }
}

pub struct ExerciseInput {
    iteration: u8,
    text: String
}

impl ExerciseInput {
    pub fn build(config: &Config) -> Result<ExerciseInput, Box<dyn Error>> {
        let file_path = format!("./data/day/{}/input", config.day);

        let iteration = config.iteration.parse::<u8>()?;

        let text = fs::read_to_string(file_path)?;

        Ok(ExerciseInput {iteration, text})
    }

}
