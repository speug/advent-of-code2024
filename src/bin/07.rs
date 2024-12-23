use itertools::Itertools;
use std::{f32::DIGITS, path::is_separator, result};

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut out = Vec::new();
    for line in input.lines() {
        let mut linesplit = line.split(":");
        let res = linesplit.next().unwrap().parse::<u64>().unwrap();
        let nums = linesplit
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        out.push((res, nums))
    }
    out
}

#[derive(PartialEq, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn count_digits(mut num: u64) -> u32 {
    let mut count = 0;
    if num == 0 {
        return 1;
    }
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}

fn check_equation(res: &u64, nums: &[u64], operators: Vec<&Operator>) -> bool {
    let mut out = nums[0];
    if nums.len() - 1 != operators.len() {
        panic!("Operator length different from num length!")
    }
    for (i, oper) in operators.iter().enumerate() {
        if **oper == Operator::Add {
            out += nums[i + 1];
        } else if **oper == Operator::Multiply {
            out *= nums[i + 1]
        } else if **oper == Operator::Concatenate {
            out = out * 10_u64.pow(count_digits(nums[i + 1].clone())) + nums[i + 1];
        }
    }
    out == *res
}

fn check_all_operations(res: &u64, nums: &[u64], allowed_operators: Vec<Operator>) -> bool {
    let num_operators = nums.len() - 1;
    for operators in
        itertools::repeat_n(&allowed_operators, num_operators).multi_cartesian_product()
    {
        let valid = check_equation(res, nums, operators);
        if valid {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    let mut out = 0u64;
    for (res, nums) in parsed {
        if check_all_operations(&res, &nums, vec![Operator::Add, Operator::Multiply]) {
            out += res as u64;
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    let mut out = 0u64;
    for (res, nums) in parsed {
        if check_all_operations(&res, &nums, vec![
            Operator::Add,
            Operator::Multiply,
            Operator::Concatenate,
        ]) {
            out += res as u64;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
