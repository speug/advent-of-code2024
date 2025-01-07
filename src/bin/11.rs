use advent_of_code::count_digits;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
}

fn blink_bf(stones: &[u64]) -> Vec<u64> {
    let mut new_stones = Vec::new();
    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
            continue;
        }
        let digits = count_digits(*stone);
        if digits % 2 == 0 {
            let right = stone / 10u64.pow(digits / 2);
            new_stones.push(right);
            let left = stone - (right * 10u64.pow(digits / 2));
            new_stones.push(left);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

fn blink_hash(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    for (stone, count) in stones.iter() {
        if *stone == 0 {
            *new_stones.entry(1).or_insert(0) += *count;
            continue;
        }
        let digits = count_digits(*stone);
        if digits % 2 == 0 {
            let right = stone / 10u64.pow(digits / 2);
            *new_stones.entry(right).or_insert(0) += *count;
            let left = stone - (right * 10u64.pow(digits / 2));
            *new_stones.entry(left).or_insert(0) += *count;
        } else {
            *new_stones.entry(*stone * 2024).or_insert(0) += *count;
        }
    }
    new_stones
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones = parse_input(input);
    // here, can just brute-force
    for _ in 0..25 {
        stones = blink_bf(&stones);
    }
    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones_list = parse_input(input);
    let mut stone_counts = HashMap::new();
    for stone in stones_list {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }
    for _ in 0..75 {
        stone_counts = blink_hash(&stone_counts);
    }
    let out = stone_counts.values().sum();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
