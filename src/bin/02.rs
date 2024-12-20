advent_of_code::solution!(2);
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut vecs = Vec::new();
    for line in input.lines() {
        let nums = line.split_whitespace();
        let mut levels = Vec::new();
        for charnum in nums {
            levels.push(charnum.parse::<u64>().unwrap());
        }
        vecs.push(levels);
    }
    vecs
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_input(input);

    let mut safe_reports = 0u64;
    for report in reports {
        let mut decreasing = true;
        let mut increasing = true;
        let mut is_safe = true;
        for (a, b) in report.into_iter().tuple_windows() {
            // set flags for increase/decrease
            if decreasing && a > b {
                decreasing = false;
            }
            if increasing && b > a {
                increasing = false;
            }
            // check diff
            let diff = a.abs_diff(b);
            if !(1..=3).contains(&diff) {
                is_safe = false;
                break;
            }
            // break if both not decreasing and not increasing
            if !decreasing && !increasing {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
