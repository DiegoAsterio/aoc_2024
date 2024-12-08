use std::{iter::zip, str::FromStr};

use crate::PuzzleInput;

#[derive(Debug)]
#[derive(PartialEq)]
enum SafetyLevel {
    Safe,
    Unsafe
}

pub fn run (input: &PuzzleInput) -> Result<(), &'static str> {
    let reports = parse_input(&input.text)?;

    if input.iteration == 0 {
        solve_fst_puzzle(reports)
    }
    else {
        Err("Unable to solve puzzle for day 2")
    }
}

fn parse_input(input: &String) -> Result<Vec<Vec<i32>>, &'static str> {
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        let levels: Result<Vec<i32>, <i32 as FromStr>::Err> = line.split(" ")
                                   .map(|x| x.parse::<i32>())
                                   .into_iter()
                                   .collect();
        match levels {
            Ok(report) => reports.push(report),
            Err(_) => return Err("Error parsing line")
        }
    }

    Ok(reports)
}

fn solve_fst_puzzle(reports: Vec<Vec<i32>>) -> Result<(), &'static str>{
    let num_safe_reports = count_safe_reports(reports);

    println!("{num_safe_reports}");

    Ok(())
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> usize {
    reports.iter().map(asses_level).filter(|x| {*x == SafetyLevel::Safe}).count()
}

fn asses_level(report: &Vec<i32>) -> SafetyLevel {
    if is_monotone(report) && adjacent_levels_dont_change_a_lot(report){
        SafetyLevel::Safe
    } else {
        SafetyLevel::Unsafe
    }
}

fn is_monotone(sequence: &Vec<i32>) -> bool {
    return only_increases(sequence) || only_decreases(sequence)
}

fn only_increases(xs: &Vec<i32>) -> bool {
    let mut increases: bool = true;
    if xs.len() >= 2 {
        for (x0, x1) in zip(xs.iter(), xs[1..].iter()){
            increases &= x0 <= x1
        }
    }
    increases
}

fn only_decreases(xs: &Vec<i32>) -> bool {
    let mut decreases: bool = true;
    if xs.len() >= 2 {
        for (x0, x1) in zip(xs.iter(), xs[1..].iter()){
            decreases &= x0 >= x1
        }
    }
    decreases
}

fn does_not_differ_a_lot(x: &i32, y: &i32) -> bool{
    let d = x.abs_diff(*y);
    1 <= d && d <= 3
}

fn adjacent_levels_dont_change_a_lot(xs: &Vec<i32>) -> bool {
    let mut meets_conditions: bool = true;
    if xs.len() >= 2 {
        for (x0, x1) in zip(xs.iter(), xs[1..].iter()){
            meets_conditions &= does_not_differ_a_lot(x0, x1)
        }
    }
    meets_conditions
}

#[cfg(test)]
mod tests {
    use crate::exercises::day_two::asses_level;

    use super::{adjacent_levels_dont_change_a_lot, count_safe_reports, does_not_differ_a_lot, only_decreases, only_increases, SafetyLevel};

    #[test]
    fn test_only_increases_over_increasing_sequence(){
        let should_be = true;

        let xs = vec![1, 2, 3];

        let only_increases_p =  only_increases(&xs);

        assert_eq!(should_be, only_increases_p)
    }

    #[test]
    fn test_only_increases_over_constant_sequence(){
        let should_be = true;

        let xs = vec![1, 1, 1];

        let only_increases_p = only_increases(&xs);

        assert_eq!(should_be, only_increases_p);

    }

    #[test]
    fn test_only_increases_over_decreasing_sequence(){
        let should_be = false;

        let xs = vec![3, 2, 1];

        let only_increases_p = only_increases(&xs);

        assert_eq!(should_be, only_increases_p);
    }

    #[test]
    fn test_only_increases_over_trivial_sequence(){
        let should_be = true;

        let xs = vec![1];

        let only_increases_p = only_increases(&xs);

        assert_eq!(should_be, only_increases_p)
    }

    #[test]
    fn test_only_increases_over_empty_sequence(){
        let should_be = true;

        let xs = vec![];

        let only_increases_p = only_increases(&xs);

        assert_eq!(should_be, only_increases_p)
    }

    #[test]
    fn test_only_decreases_over_increasing_sequence(){
        let should_be = false;

        let xs = vec![1, 2, 3];

        let only_decreases_p = only_decreases(&xs);

        assert_eq!(should_be, only_decreases_p);
    }

    #[test]
    fn test_only_decreases_over_constant_sequence(){
        let should_be = true;

        let xs = vec![1, 1, 1];

        let only_decreases_p = only_decreases(&xs);

        assert_eq!(should_be, only_decreases_p);
    }

    #[test]
    fn test_only_decreases_over_decreasing_sequence(){
        let should_be = true;

        let xs = vec![3, 2, 1];

        let only_decreases_p = only_decreases(&xs);

        assert_eq!(should_be, only_decreases_p);
    }

    #[test]
    fn test_only_decreases_over_trivial_sequence(){
        let should_be = true;

        let xs = vec![1];

        let only_decreases_p = only_decreases(&xs);

        assert_eq!(should_be, only_decreases_p);
    }

    #[test]
    fn test_only_decreases_over_empty_sequence(){
        let should_be = true;

        let xs = vec![];

        let only_decreases_p = only_decreases(&xs);

        assert_eq!(should_be, only_decreases_p);
    }

    #[test]
    fn test_does_not_differ() {
        let should_be = false;

        let result_p = does_not_differ_a_lot(&1, &1);

        assert_eq!(should_be, result_p);
    }

    #[test]
    fn test_does_not_differ_a_lot_one() {
        let should_be = true;

        let result_p = does_not_differ_a_lot(&1, &2);

        assert_eq!(should_be, result_p);
    }
    #[test]
    fn test_does_not_differ_a_lot_two() {
        let should_be = true;

        let result_p = does_not_differ_a_lot(&1, &3);

        assert_eq!(should_be, result_p);
    }
    #[test]
    fn test_does_not_differ_a_lot_three() {
        let should_be = true;

        let result_p = does_not_differ_a_lot(&1, &4);

        assert_eq!(should_be, result_p);
    }

    #[test]
    fn test_does_differ_a_lot() {
        let should_be = false;

        let result_p = does_not_differ_a_lot(&1, &5);

        assert_eq!(should_be, result_p);
    }

    #[test]
    fn test_levels_dont_change_a_lot() {
        let should_be = true;

        let xs = vec![1, 3, 6, 7, 9];

        let result_p = adjacent_levels_dont_change_a_lot(&xs);

        assert_eq!(should_be, result_p);
    }

    #[test]
    fn asses_first_level() {
        let should_be = SafetyLevel::Safe;

        let xs = vec![7, 6, 4, 2, 1];

        let assessment = asses_level(&xs);

        assert_eq!(should_be, assessment)
    }

    #[test]
    fn asses_second_level() {
        let should_be = SafetyLevel::Unsafe;

        let xs = vec![1, 2, 7, 8, 9];

        let assessment = asses_level(&xs);

        assert_eq!(should_be, assessment)
    }

    #[test]
    fn asses_last_level() {
        let should_be = SafetyLevel::Safe;

        let xs = vec![1, 3, 6, 7, 9];

        let assessment = asses_level(&xs);

        assert_eq!(should_be, assessment)
    }

    #[test]
    fn count_safe_reports_from_the_example(){
        let should_be = 2;

        let reports = vec![
                        vec![7, 6, 4, 2, 1],
                        vec![1, 2, 7, 8, 9],
                        vec![9, 7, 6, 2, 1],
                        vec![1, 3, 2, 4, 5],
                        vec![8, 6, 4, 4, 1],
                        vec![1, 3, 6, 7, 9]
                     ];

        let num_safe_reports = count_safe_reports(reports);

        assert_eq!(should_be, num_safe_reports)
    }

}
