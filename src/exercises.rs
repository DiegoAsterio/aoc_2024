use crate::PuzzleInput;

mod day_one;

pub fn run(input: &PuzzleInput) -> Result<(), &'static str> {
    if input.day == 1 {
        day_one::run(input)
    }
    else {
        Err("The day requested has not been solved yet!")
    }
}
