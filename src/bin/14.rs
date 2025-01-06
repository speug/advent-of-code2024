advent_of_code::solution!(14);
use core::fmt;
use regex::Regex;
use std::collections::HashMap;

struct Robot {
    pos: (i16, i16),
    v: (i16, i16),
}

impl Robot {
    fn step(&mut self, grid_w: i16, grid_h: i16) {
        self.pos = (
            (((self.pos.0 + self.v.0) % grid_w) + grid_w) % grid_w,
            (((self.pos.1 + self.v.1) % grid_h) + grid_h) % grid_h,
        );
    }
}

impl fmt::Debug for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Robot pos: {:?}, velocity: {:?}", self.pos, self.v)
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let line_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = Vec::new();
    for line in input.lines() {
        let opt_captures = line_regex.captures(line);
        if opt_captures.is_none() {
            break;
        }
        let captures = opt_captures.unwrap();
        let p_x = captures.get(1).unwrap().as_str().parse::<i16>().unwrap();
        let p_y = captures.get(2).unwrap().as_str().parse::<i16>().unwrap();
        let v_x = captures.get(3).unwrap().as_str().parse::<i16>().unwrap();
        let v_y = captures.get(4).unwrap().as_str().parse::<i16>().unwrap();
        let r = Robot {
            pos: (p_x, p_y),
            v: (v_x, v_y),
        };
        robots.push(r);
    }
    robots
}

pub fn part_one(input: &str) -> Option<u64> {
    let robots = parse_input(input);
    let (w, h) = (101, 103);
    let mut end_coords = HashMap::new();
    for mut r in robots.into_iter() {
        for _ in 0..100 {
            r.step(w, h);
        }
        *end_coords.entry(r.pos).or_insert(0) += 1;
    }
    let (middle_x, middle_y) = (w / 2, h / 2);
    let mut quadrants = vec![0, 0, 0, 0];
    for (pos, count) in end_coords.iter() {
        if pos.0 < middle_x && pos.1 < middle_y {
            quadrants[0] += count;
        } else if pos.0 > middle_x && pos.1 < middle_y {
            quadrants[1] += count;
        } else if pos.0 < middle_x && pos.1 > middle_y {
            quadrants[2] += count;
        } else if pos.0 > middle_x && pos.1 > middle_y {
            quadrants[3] += count;
        }
    }
    Some((quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // just print/display?
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
