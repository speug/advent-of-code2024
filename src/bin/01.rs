advent_of_code::solution!(1);
use std::collections::HashMap;
use std::iter::zip;

fn input_to_columns(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input.lines() {
        let mut nums = line.split_whitespace();

        if let (Some(x), Some(y)) = (nums.next(), nums.next()) {
            column1.push(x.parse::<u64>().unwrap());
            column2.push(y.parse::<u64>().unwrap());
        }
    }

    assert_eq!(column1.len(), column2.len());
    (column1, column2)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut column1, mut column2) = input_to_columns(input);
    column1.sort();
    column2.sort();

    let mut sum_of_difference = 0u64;
    for (a, b) in zip(column1, column2) {
        sum_of_difference += a.abs_diff(b)
    }
    Some(sum_of_difference)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (column1, column2) = input_to_columns(input);
    let mut counts = HashMap::new();

    for &item in &column2 {
        *counts.entry(item).or_insert(0) += 1u64;
    }

    let mut out = 0u64;
    for &key in &column1 {
        let multiplier = counts.get(&key).unwrap_or(&0u64);
        out += key * multiplier;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
