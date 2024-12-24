advent_of_code::solution!(9);
use std::iter::repeat;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u8, i16)> {
    let mut out = Vec::new();
    let mut file_indicator = 0i16;
    for (i, elem) in input.chars().into_iter().enumerate() {
        let num = elem.to_digit(10).unwrap() as u8;
        if i % 2 == 0 {
            out.push((num, file_indicator));
            file_indicator += 1;
        } else {
            out.push((num, -1));
        }
    }
    out
}

fn filesystem_to_string(fs: Vec<(u8, i16)>) -> String {
    fs.iter()
        .map(|&(size, elem)| {
            if elem == -1 {
                ".".repeat(size as usize)
            } else {
                elem.to_string().repeat(size as usize)
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut fs = parse_input(input);
    println!("{}", filesystem_to_string(fs));
    None
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
        assert_eq!(result, Some((1928)));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
