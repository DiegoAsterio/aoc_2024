use std::str::FromStr;

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
        let fault_tolerance = 0;
        solve_puzzle(reports, fault_tolerance)
    }
    else if input.iteration == 1 {
        let fault_tolerance = 1;
        solve_puzzle(reports, fault_tolerance)
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

fn solve_puzzle(reports: Vec<Vec<i32>>, fault_tolerance: i32) -> Result<(), &'static str>{
    let num_safe_reports = count_safe_reports(reports, fault_tolerance);

    println!("{num_safe_reports}");

    Ok(())
}

fn count_safe_reports(reports: Vec<Vec<i32>>, tolerance: i32) -> usize {
    reports.iter().map(|x| {assess_level(x, tolerance)}).filter(|y| {*y == SafetyLevel::Safe}).count()
}

fn assess_level(report: &Vec<i32>, tolerance: i32) -> SafetyLevel {
    if is_safe(report, tolerance){
        SafetyLevel::Safe
    } else {
        SafetyLevel::Unsafe
    }
}

fn is_safe(report: &Vec<i32>, tolerance: i32) -> bool{
    is_increasing_and_safe(report, tolerance) || is_increasing_and_safe(&report.iter().copied().rev().collect(), tolerance)
}

fn is_increasing_and_safe(report: &Vec<i32>, tolerance: i32) -> bool{
    if tolerance < 0 {
        return false;
    }
    for i in 0..report.len()-1 {
        let current = report[i];
        let next = report[i+1];
        let diff = next - current;

        if current >= next || diff > 3{
            let new_tolerance = tolerance - 1;
            let mut report_without_current = report.clone();
            let mut report_without_next =  report.clone();
            report_without_current.remove(i);
            report_without_next.remove(i+1);
            return is_increasing_and_safe(&report_without_current, new_tolerance) || is_increasing_and_safe(&report_without_next, new_tolerance)
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{assess_level, count_safe_reports, SafetyLevel};

    #[test]
    fn report_one_puzzle_one(){
        let tolerance = 0;

        let report = vec![7, 6, 4, 2, 1];
        let should_be = SafetyLevel::Safe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }
 #[test]
    fn report_two_puzzle_one(){
        let tolerance = 0;

        let report = vec![1, 2, 7, 8, 9];
        let should_be = SafetyLevel::Unsafe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }
 #[test]
    fn report_three_puzzle_one(){
        let tolerance = 0;

        let report = vec![9, 7, 6, 2, 1];
        let should_be = SafetyLevel::Unsafe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }
 #[test]
    fn report_four_puzzle_one(){
        let tolerance = 0;

        let report = vec![1, 3, 2, 4, 5];
        let should_be = SafetyLevel::Unsafe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }

 #[test]
    fn report_five_puzzle_one(){
        let tolerance = 0;

        let report = vec![8, 6, 4, 4, 1];
        let should_be = SafetyLevel::Unsafe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }

 #[test]
    fn report_six_puzzle_one(){
        let tolerance = 0;

        let report = vec![1, 3, 6, 7, 9];
        let should_be = SafetyLevel::Safe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }

#[test]
    fn count_safe_reports_from_puzzle_one_example(){
        let should_be = 2;

        let reports = vec![
                        vec![7, 6, 4, 2, 1],
                        vec![1, 2, 7, 8, 9],
                        vec![9, 7, 6, 2, 1],
                        vec![1, 3, 2, 4, 5],
                        vec![8, 6, 4, 4, 1],
                        vec![1, 3, 6, 7, 9]
                     ];

        let num_safe_reports = count_safe_reports(reports, 0);

        assert_eq!(should_be, num_safe_reports)
    }

#[test]
    fn report_two_puzzle_two(){
        let tolerance = 1;

        let report = vec![1, 2, 7, 8, 9];
        let should_be = SafetyLevel::Unsafe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }
 #[test]
    fn report_three_puzzle_two(){
        let tolerance = 1;

        let report = vec![9, 7, 6, 2, 1];
        let should_be = SafetyLevel::Unsafe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }
 #[test]
    fn report_four_puzzle_two(){
        let tolerance = 1;

        let report = vec![1, 3, 2, 4, 5];
        let should_be = SafetyLevel::Safe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }

 #[test]
    fn report_five_puzzle_two(){
        let tolerance = 1;

        let report = vec![8, 6, 4, 4, 1];
        let should_be = SafetyLevel::Safe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }

 #[test]
    fn report_six_puzzle_two(){
        let tolerance = 1;

        let report = vec![1, 3, 6, 7, 9];
        let should_be = SafetyLevel::Safe;

        let report_safety = assess_level(&report, tolerance);

        assert_eq!(should_be, report_safety);
    }

    #[test]
    fn count_safe_reports_from_puzzle_two_example(){
        let should_be = 4;

        let reports = vec![
                        vec![7, 6, 4, 2, 1],
                        vec![1, 2, 7, 8, 9],
                        vec![9, 7, 6, 2, 1],
                        vec![1, 3, 2, 4, 5],
                        vec![8, 6, 4, 4, 1],
                        vec![1, 3, 6, 7, 9]
                     ];

        let num_safe_reports = count_safe_reports(reports, 1);

        assert_eq!(should_be, num_safe_reports)
    }

}
