use crate::PuzzleInput;

mod day_one;
mod day_two;
mod day_three;


pub fn run(input: &PuzzleInput) -> Result<(), &'static str> {
    if input.day == 1 {
        day_one::run(input)
    }
    else if input.day == 2 {
        day_two::run(input)
    }
    else if input.day == 3 {
        day_three::run(input)
    }
    else {
        Err("The day requested has not been solved yet!")
    }
}
