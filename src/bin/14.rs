advent_of_code::solution!(14);
use colored::Colorize;
use core::fmt;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use regex::Regex;
use std::{collections::HashMap, io::Error};

#[derive(Clone)]
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

    fn step_back(&mut self, grid_w: i16, grid_h: i16) {
        self.pos = (
            (((self.pos.0 - self.v.0) % grid_w) + grid_w) % grid_w,
            (((self.pos.1 - self.v.1) % grid_h) + grid_h) % grid_h,
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

fn calculate_safety_score(robots: &Vec<Robot>, grid_w: i16, grid_h: i16) -> u64 {
    let mut end_coords = HashMap::new();
    for r in robots.iter() {
        *end_coords.entry(r.pos).or_insert(0) += 1;
    }
    let (middle_x, middle_y) = (grid_w / 2, grid_h / 2);
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
    (quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let robots = parse_input(input);
    let (w, h) = (101, 103);
    for mut r in robots.clone().into_iter() {
        for _ in 0..100 {
            r.step(w, h);
        }
    }
    Some(calculate_safety_score(&robots, w, h))
}

fn prettyprint_grid(robots: &Vec<Robot>, grid_width: i16, grid_height: i16) -> Result<(), Error> {
    disable_raw_mode()?;
    let mut robot_counts = HashMap::new();
    for r in robots.iter() {
        *robot_counts.entry(r.pos).or_insert(0) += 1;
    }
    for i in 0..grid_height {
        for j in 0..grid_width {
            if robot_counts.contains_key(&(i, j)) {
                let count = robot_counts.get(&(i, j)).unwrap();
                let count_char = if *count > 9 {
                    "#".to_string()
                } else {
                    count.to_string()
                };
                print!("{}", count_char.green());
            } else {
                print!(".");
            }
            if j == grid_width - 1 {
                println!();
            }
        }
    }
    enable_raw_mode()?;
    Ok(())
}

fn step_all(robots: &mut Vec<Robot>, grid_w: i16, grid_h: i16, time: &mut u16) {
    for r in robots.iter_mut() {
        r.step(grid_w, grid_h);
    }
    *time += 1;
    println!("Grid at time t={}", time);
    let _ = prettyprint_grid(robots, grid_w, grid_h);
}

fn step_all_back(robots: &mut Vec<Robot>, grid_w: i16, grid_h: i16, time: &mut u16) {
    if *time > 0 {
        for r in robots.iter_mut() {
            r.step_back(grid_w, grid_h);
        }
        *time -= 1;
    }
    println!("Grid at time t={}", time);
    let _ = prettyprint_grid(robots, grid_w, grid_h);
}

fn step_until_safer(robots: &mut Vec<Robot>, grid_w: i16, grid_h: i16, time: &mut u16) {
    let max_iter: u16 = 10000;
    let mut iter = 0u16;
    let mut safety = calculate_safety_score(robots, grid_w, grid_h);
    while (iter < max_iter) && (calculate_safety_score(robots, grid_w, grid_h) >= safety) {
        for r in robots.iter_mut() {
            r.step(grid_w, grid_h);
        }
        *time += 1;
        iter += 1;
    }
    println!("Grid at time t={}", time);
    let _ = prettyprint_grid(robots, grid_w, grid_h);
}

fn grid_visualiser(
    mut robots: Vec<Robot>,
    grid_width: i16,
    grid_height: i16,
) -> Result<u16, Error> {
    let mut time = 0u16;
    enable_raw_mode()?;

    println!(
        "Press right arrow to step forward in time, left arrow to go back or enter to output current time. Press 'q' to quit."
    );
    loop {
        if event::poll(std::time::Duration::from_secs(1))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Left => step_all_back(&mut robots, grid_width, grid_height, &mut time),
                    KeyCode::Right => step_all(&mut robots, grid_width, grid_height, &mut time),
                    KeyCode::Up => {
                        step_until_safer(&mut robots, grid_width, grid_height, &mut time)
                    }
                    KeyCode::Enter => {
                        println!("Enter pressed, tree found.");
                        break;
                    }
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(time)
}

pub fn part_two(input: &str) -> Option<u64> {
    // just print/display?
    let mut cache = None;
    if cache.is_none() {
        let robots = parse_input(input);
        let (w, h) = (101, 103);
        cache = grid_visualiser(robots, w, h).ok();
    }
    Some(cache.unwrap() as u64)
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
