advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let mut output: u64 = 0;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let matches: Vec<_> = mul_re.find_iter(input).map(|mat| mat.as_str()).collect();

    for mat in matches {
        if let Some(captures) = mul_re.captures(mat) {
            let x = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            output += x * y;
        }
    }
    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    // add do() in the beginning of line
    // split at every do()
    // for each chunk, split at dont(); work on first part
    let mut output: u64 = 0;
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let line_split = input.split("do()");
    for do_part in line_split {
        let until_dont = do_part.split("don't()").next().unwrap();
        let matches: Vec<_> = mul_re
            .find_iter(until_dont)
            .map(|mat| mat.as_str())
            .collect();

        for mat in matches {
            if let Some(captures) = mul_re.captures(mat) {
                let x = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let y = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                output += x * y;
            }
        }
    }
    Some(output)
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
        assert_eq!(result, Some(48));
    }
}
