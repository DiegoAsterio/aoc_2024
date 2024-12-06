use std::iter::zip;

use crate::ExerciseInput;

pub fn run(input: &ExerciseInput) -> Result<(), &'static str>{
    if input.iteration == 0{
        solve_first_problem(&input.text)
    }
    else {
        Err("Unable to run exercise")
    }
}

fn solve_first_problem(input: &String) -> Result<(), &'static str>{
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    for line in input.lines(){
        let values: Vec<i32> = line.split("   ")
                                      .map(|x| x.parse::<i32>().unwrap())
                                      .collect();
        if values.len() != 2 {
            return Err("incorrect input");
        }
        else {
            list_one.push(values[0]);
            list_two.push(values[1]);
        }
    }

    let result = calculate_distance(list_one, list_two);

    println!("{result}");

    Ok(())
}

fn calculate_distance(mut list_one: Vec<i32>, mut list_two: Vec<i32>) -> u32{
    list_one.sort();
    list_two.sort();

    let mut ret: u32 = 0;
    for (x1, x2) in zip(list_one, list_two){
        ret = ret + x1.abs_diff(x2);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completes_day_1_example(){
        let should_be = 11;

        let list_one = vec![3, 4, 2, 1, 3, 3];
        let list_two = vec![4, 3, 5, 3, 9, 3];

        let result = calculate_distance(list_one, list_two);

        assert_eq!(should_be, result);
    }
}
