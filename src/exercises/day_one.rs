use std::{collections::HashMap, iter::zip};

use crate::PuzzleInput;

pub fn run(input: &PuzzleInput) -> Result<(), &'static str>{
    let (xs, ys) = parse_input(&input.text)?;

    if input.iteration == 0{
        solve_fst_puzzle(xs, ys)
    }
    else if input.iteration == 1{
        solve_snd_puzzle(xs, ys)
    }
    else {
        Err("Unable to run exercise")
    }
}

fn parse_input(input: &String) -> Result<(Vec<i32>, Vec<i32>), &'static str> {
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

    Ok((list_one, list_two))
}

fn solve_fst_puzzle(xs: Vec<i32>, ys: Vec<i32>) -> Result<(), &'static str>{

    let result = distance(xs, ys);

    println!("{result}");

    Ok(())
}

fn distance(mut list_one: Vec<i32>, mut list_two: Vec<i32>) -> u32 {
    list_one.sort();
    list_two.sort();

    let mut ret: u32 = 0;
    for (x1, x2) in zip(list_one, list_two){
        ret = ret + x1.abs_diff(x2);
    }
    ret
}

fn solve_snd_puzzle(xs: Vec<i32>, ys: Vec<i32>) -> Result<(), &'static str>{

    let result = similarity_score(xs, ys);

    println!("{result}");

    Ok(())
}

fn similarity_score(xs: Vec<i32>, ys: Vec<i32>) -> i32 {
    let ocurrence_counter: HashMap<i32,u16> = ys.iter().fold(HashMap::new(), |mut acc, y| {
        *acc.entry(*y).or_insert(0) += 1;
        acc
    });

    let mut score: i32 = 0;

    for x in xs {
        match ocurrence_counter.get(&x) {
            Some(ocurrences) => score += x * i32::from(*ocurrences),
            None => ()
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculates_distance(){
        let should_be = 11;

        let list_one = vec![3, 4, 2, 1, 3, 3];
        let list_two = vec![4, 3, 5, 3, 9, 3];

        let result = distance(list_one, list_two);

        assert_eq!(should_be, result);
    }

    #[test]
    fn test_calculates_similarity_score(){
        let should_be = 31;

        let xs = vec![3, 4, 2, 1, 3, 3];
        let ys = vec![4, 3, 5, 3, 9, 3];

        let similarity = similarity_score(xs, ys);

        assert_eq!(should_be, similarity)
    }
}
