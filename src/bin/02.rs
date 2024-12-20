advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut vecs = Vec::new();
    for line in input.lines() {
        let nums = line.split_whitespace();
        let mut levels = Vec::new();
        for charnum in nums {
            levels.push(charnum.parse::<i32>().unwrap());
        }
        vecs.push(levels);
    }
    vecs
}

fn check_single_report(report: Vec<i32>) -> bool {
    let mut decreasing = true;
    let mut increasing = true;
    for i in 0..report.len() - 1 {
        // set flags for increase/decrease
        let a = &report[i];
        let b = &report[i + 1];
        if decreasing && a >= b {
            decreasing = false;
        }
        if increasing && b >= a {
            increasing = false;
        }
        // check diff
        let diff = (1..=3).contains(&a.abs_diff(*b));
        if !diff || (!decreasing && !increasing) {
            // recursively check single difference collections
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = parse_input(input);
    let mut safe_reports = 0u64;
    for report in reports {
        if check_single_report(report) {
            safe_reports += 1;
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    let reports = parse_input(input);
    let mut safe_reports = 0u64;
    for report in reports.clone() {
        if check_single_report(report.clone()) {
            safe_reports += 1;
        } else {
            for ri in 0..report.len() {
                let mut remove_i = report.clone();
                remove_i.remove(ri);
                if check_single_report(remove_i.clone()) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    Some(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }
}
