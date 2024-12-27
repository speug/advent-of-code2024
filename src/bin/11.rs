use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
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

fn build_lookup(steps: u8) -> HashMap<(u8, u8), u64> {
    let mut digits_cache: HashMap<(u8, u8), u64> = HashMap::new();
    // build trivial cases (calculated manually)
    for digit in 0..10 {
        if digit == 0 {
            digits_cache.insert((0, 1), 1);
            digits_cache.insert((0, 2), 1);
            digits_cache.insert((0, 3), 2);
            digits_cache.insert((0, 4), 4);
        } else if digit < 5 {
            digits_cache.insert((digit, 1), 1);
            digits_cache.insert((digit, 2), 2);
            digits_cache.insert((digit, 3), 4);
        } else {
            digits_cache.insert((digit, 1), 1);
            digits_cache.insert((digit, 2), 1);
            digits_cache.insert((digit, 3), 2);
            digits_cache.insert((digit, 4), 4);
            digits_cache.insert((digit, 5), 8);
        }
    }
    // build breakdowns
    let mut breakdown_with_lag: HashMap<u8, (Vec<u8>, u8)> = HashMap::new();
    for digit in 0..10 {
        if digit == 0 {
            // 0 is just a one with a lag of 1
            breakdown_with_lag.insert(0, (vec![1], 1));
        } else if digit < 5 {
            // digits 1-4 all break down to 4 digits with a lag of 3 (1, 2, 4)
            let breakdown = (digit as u32 * 2024)
                .to_string()
                .chars()
                .map(|v| v.to_digit(10).unwrap() as u8)
                .collect();
            breakdown_with_lag.insert(digit, (breakdown, 3));
        } else {
            // digits 5-9 break down to 8 digits with a lag of 5 (1, 1, 2, 4, 8)
            let breakdown = (digit as u32 * 2024 * 2024)
                .to_string()
                .chars()
                .map(|v| v.to_digit(10).unwrap() as u8)
                .collect();
            breakdown_with_lag.insert(digit, (breakdown, 5));
        }
    }
    // extrapolate using the primitive cases and breakdowns
    for step in 4..=steps {
        for digit in 0..10 {
            if digits_cache.contains_key(&(digit, step)) {
                continue;
            }
            let (breakdown, lag) = &breakdown_with_lag[&digit];
            // the new value is just the sum of the broken down digit values with some lag
            // e.g. f(1, n) = f(2, n-3) + f(0, n-3) + f(2, n-3) + f(4, n-3)
            let next_value = breakdown
                .iter()
                .map(|d| digits_cache.get(&(*d, step - *lag)).unwrap())
                .sum();
            digits_cache.insert((digit, step), next_value);
        }
    }
    digits_cache
}

fn blink_dynamic(stones: &[u64], steps: u8) -> u64 {
    let digit_values = build_lookup(steps);
    for s in 1..=steps {
        println!("{:?}", digit_values.get(&(1, s)).unwrap());
    }
    let mut out = 0u64;
    let mut new_stones = stones.to_vec();
    for blink in 0..steps {
        if new_stones.is_empty() {
            break;
        }
        let mut new_new_stones = Vec::new();
        for (i, stone) in new_stones.into_iter().enumerate() {
            if stone < 10 {
                out += digit_values.get(&(stone as u8, steps - blink)).unwrap();
            } else {
                new_new_stones.push(stone);
            }
        }
        new_stones = blink_bf(&new_new_stones);
    }
    println!("{:?}", new_stones);
    out + new_stones.len() as u64
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
    for i in 0..25 {
        if i < 6 {
            println!("{:?}", stones);
        }
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
