use regex::Regex;

use crate::PuzzleInput;

pub fn run(input: &PuzzleInput) -> Result<(), &'static str>{
    let memory_trace = input.text.clone();

    let operations = select_valid_operations(&memory_trace);

    let result: i32 = operations.iter().map(|(op,f,s)| {BinaryOperation::new(op, *f, *s).apply()}).sum();

    println!("{}", result);

    Ok(())
}

fn select_valid_operations(s: &str) -> Vec<(&str, i32, i32)>{
    let re = Regex::new(r"(mul)\((\d*\d*\d),(\d*\d*\d)\)").unwrap();

    re.captures_iter(s).map(|caps| {
        let (_, [operator, fst, snd]) = caps.extract();
        (operator, fst.parse::<i32>().unwrap(), snd.parse::<i32>().unwrap())
    }).collect()
}

enum BinaryOperator {
    Mul
}

impl BinaryOperator {
    fn new(operator: &str) -> BinaryOperator {
        if operator == "mul" {
            BinaryOperator::Mul
        }
        else {
            panic!("Error while initializing binary operator.");
        }
    }
}

struct BinaryOperation {
    operator: BinaryOperator,
    fst: i32,
    snd: i32
}

impl BinaryOperation {
    fn new(operator: &str, fst: i32, snd: i32) -> BinaryOperation{
        Self {
            operator: BinaryOperator::new(operator),
            fst,
            snd,
        }
    }

    fn apply(&self) -> i32 {
        match self.operator {
            BinaryOperator::Mul => self.fst * self.snd
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryOperation, select_valid_operations};

    #[test]
    fn select_valid_operations_test(){
        let corrupted_memory =  String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let should_be = vec![("mul",2,4), ("mul",5,5), ("mul",11,8), ("mul",8,5)];

        let correctly_formed_operations = select_valid_operations(&corrupted_memory);

        assert_eq!(should_be, correctly_formed_operations)
    }

    #[test]
    fn calculate_operation_test(){
        let example_number_one = BinaryOperation::new("mul", 44, 46);

        let should_be = 2024;

        assert_eq!(should_be, example_number_one.apply())
    }
}
