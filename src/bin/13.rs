advent_of_code::solution!(13);
use core::fmt;
use std::cmp::min;
use std::mem::swap;

use regex::Regex;

struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
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

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Machine - A X+{} Y+{} - B X+{} Y+{} - Prize X={} Y={}",
            self.a.0, self.a.1, self.b.0, self.b.1, self.prize.0, self.prize.1
        )
    }
}

fn parse_input(input: &str, prize_offset: u64) -> Vec<Machine> {
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
            .parse::<u64>()
            .unwrap();
        let a_y = captures
            .as_ref()
            .expect("No value!")
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let b_line = lines.next().unwrap();
        let captures = button_regex.captures(b_line);
        let b_x = captures
            .as_ref()
            .expect("No value!")
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let b_y = captures
            .as_ref()
            .expect("No value!")
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let prize_line = lines.next().unwrap();
        let captures = prize_regex.captures(prize_line);
        let p_x = captures
            .as_ref()
            .expect("No value!")
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let p_y = captures
            .as_ref()
            .expect("No value!")
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let m = Machine {
            a: (a_x, a_y),
            b: (b_x, b_y),
            prize: (p_x + prize_offset, p_y + prize_offset),
        };
        out.push(m);
        // skip the empty line
        lines.next();
    }
    out
}

// from wikipedia
fn binary_gcd(mut u: u64, mut v: u64) -> u64 {
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        if u > v {
            swap(&mut u, &mut v);
        }
        v -= u;

        if v == 0 {
            return u << k;
        }

        v >>= v.trailing_zeros();
    }
}

fn cheapest_win(m: &Machine, upper_limit: i64) -> Option<u64> {
    // infeasibility check
    let d_x = binary_gcd(m.a.0, m.b.0);
    if m.prize.0 % d_x != 0 {
        return None;
    }
    let d_y = binary_gcd(m.a.1, m.b.1);
    if m.prize.1 % d_y != 0 {
        return None;
    }
    // check for solutions using cramer's rule
    let s_den: i64 = m.prize.0 as i64 * m.b.1 as i64 - m.b.0 as i64 * m.prize.1 as i64;
    let t_den: i64 = m.a.0 as i64 * m.prize.1 as i64 - m.prize.0 as i64 * m.a.1 as i64;
    let numerator = m.a.0 as i64 * m.b.1 as i64 - m.a.1 as i64 * m.b.0 as i64;
    // unique solution
    if s_den != 0 || t_den != 0 || numerator != 0 {
        if (s_den % numerator == 0) && (t_den % numerator == 0) {
            let (s, t) = (s_den / numerator, t_den / numerator);
            if (s < 0) || (t < 0) {
                return None;
            }
            if upper_limit == 0 || (s < upper_limit && t < upper_limit) {
                return Some(3 * s as u64 + t as u64);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    panic!("Multiple solutions! {:}", m);
}

pub fn part_one(input: &str) -> Option<u64> {
    // X, Y can only be reached if X % gcd(a.x, b.x) == 0 and same for y
    // also, gcd(a.x, b.x) is the smallest coeff in the linear combs
    let machines = parse_input(input, 0);
    // println!("{:?}", machines);
    let mut out = 0u64;
    for m in machines {
        let cost = cheapest_win(&m, 100);
        if cost.is_some() {
            out += cost.unwrap() as u64;
        }
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input, 10000000000000);
    // println!("{:?}", machines);
    let mut out = 0u64;
    for m in machines {
        let cost = cheapest_win(&m, 0);
        if cost.is_some() {
            out += cost.unwrap() as u64;
        }
    }
    Some(out)
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
