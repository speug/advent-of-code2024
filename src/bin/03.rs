advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let mut output: u64 = 0;
    let mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for line in input.lines() {
        let matches: Vec<_> = mul_re.find_iter(line).map(|mat| mat.as_str()).collect();

        for mat in matches {
            if let Some(captures) = num_re.captures(mat) {
                let x = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let y = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                output += x * y;
            }
        }
    }
    Some(output)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
