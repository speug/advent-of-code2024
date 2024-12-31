advent_of_code::solution!(13);
use core::fmt;
use std::cmp::min;
use std::mem::swap;

use regex::Regex;

struct Machine {
    a: (u16, u16),
    b: (u16, u16),
    prize: (u16, u16),
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine - A X+{} Y+{} - B X+{} Y+{} - Prize X={} Y={}",
            self.a.0, self.a.1, self.b.0, self.b.1, self.prize.0, self.prize.1
        )
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut out = Vec::new();
    let mut lines = input.lines().peekable();
    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    loop {
        if lines.peek().is_none() {
            break;
        }
        let a_line = lines.next().unwrap();
        let captures = button_regex.captures(a_line);
        // sigh rust with all of this unwrap expect stuff
        let a_x = captures
            .as_ref()
            .expect("No value!")
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let a_y = captures
            .as_ref()
            .expect("No value!")
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let b_line = lines.next().unwrap();
        let captures = button_regex.captures(b_line);
        let b_x = captures
            .as_ref()
            .expect("No value!")
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let b_y = captures
            .as_ref()
            .expect("No value!")
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let prize_line = lines.next().unwrap();
        let captures = prize_regex.captures(prize_line);
        let p_x = captures
            .as_ref()
            .expect("No value!")
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let p_y = captures
            .as_ref()
            .expect("No value!")
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let m = Machine {
            a: (a_x, a_y),
            b: (b_x, b_y),
            prize: (p_x, p_y),
        };
        out.push(m);
        // skip the empty line
        lines.next();
    }
    out
}

// from wikipedia

fn extended_gcd(a: u16, b: u16) -> (u16, i16, i16) {
    let (mut s, mut old_s): (i16, i16) = (0, 1);
    let (mut r, mut old_r) = (b as i16, a as i16);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    let bezout_t = if b != 0 {
        (old_r - old_s * a as i16) / b as i16
    } else {
        0
    };
    // (gcd, s, t)
    (old_r as u16, old_s, bezout_t)
}

fn cheapest_win(m: &Machine) -> Option<u16> {
    // infeasibility check
    let (d_x, s, t) = extended_gcd(m.a.0, m.b.0);
    if m.prize.0 % d_x != 0 {
        return None;
    }
    let (d_y, s, t) = extended_gcd(m.a.1, m.b.1);
    if m.prize.1 % d_y != 0 {
        return None;
    }
    Some(1)
}

pub fn part_one(input: &str) -> Option<u64> {
    // X, Y can only be reached if X % gcd(a.x, b.x) == 0 and same for y
    // also, gcd(a.x, b.x) is the smallest coeff in the linear combs
    let machines = parse_input(input);
    println!("{:?}", machines);
    println!("{:?}", extended_gcd(94, 22));
    for m in machines {
        if cheapest_win(&m).is_some() {
            println!("{:?} is feasible", m);
        } else {
            println!("{:?} is infeasible", m);
        }
    }
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
