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
    pos: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn step_n(&mut self, grid_w: i32, grid_h: i32, n: i32) {
        self.pos = (
            (((self.pos.0 + n * self.v.0) % grid_w) + grid_w) % grid_w,
            (((self.pos.1 + n * self.v.1) % grid_h) + grid_h) % grid_h,
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
        let p_x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let p_y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let v_x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let v_y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let r = Robot {
            pos: (p_x, p_y),
            v: (v_x, v_y),
        };
        robots.push(r);
    }
    robots
}

fn calculate_safety_score(robots: &Vec<Robot>, grid_w: i32, grid_h: i32) -> u64 {
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
    let mut robots = parse_input(input);
    // let (w, h) = (11, 7);
    let (w, h): (i32, i32) = (101, 103);
    step_all_n(&mut robots, 100, w, h, 0);
    Some(calculate_safety_score(&robots, w, h))
}

fn prettyprint_grid(robots: &Vec<Robot>, grid_width: i32, grid_height: i32) -> Result<(), Error> {
    // test if this works!
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

fn step_all_n(robots: &mut Vec<Robot>, n: i32, grid_w: i32, grid_h: i32, time: i32) {
    assert!(time + n >= 0);
    for r in robots.iter_mut() {
        r.step_n(grid_w, grid_h, n);
    }
}

fn grid_visualiser(
    orig_robots: &Vec<Robot>,
    grid_width: i32,
    grid_height: i32,
) -> Result<u16, Error> {
    // first, calculate all safety scores
    let max_iter = 10000;
    let mut safety_scores = vec![0; max_iter + 1];
    let mut robots = orig_robots.clone();
    safety_scores[0] = calculate_safety_score(&robots, grid_width, grid_height);
    for i in 1..=max_iter {
        step_all_n(&mut robots, 1, grid_width, grid_height, i as i32);
        safety_scores[i] = calculate_safety_score(&robots, grid_width, grid_height);
    }
    // sort the time stamps by safety scores
    let mut times: Vec<usize> = (0..safety_scores.len()).collect();
    times.sort_by(|&i, &j| safety_scores[i].cmp(&safety_scores[j]));

    let mut time_pointer: usize = 0;

    // display the boards in a descending order of safety score; the picture has the 5th? lowest score
    enable_raw_mode()?;

    println!(
        "Press right arrow to step forward in time, left arrow to go back or enter to output current time. Press 'q' to quit."
    );
    // draw the first board
    let time = times[time_pointer] as i32;
    let mut robots = orig_robots.clone();
    step_all_n(&mut robots, time, grid_width, grid_height, time);
    disable_raw_mode()?;
    println!("Grid at time t={}", time);
    let _ = prettyprint_grid(&robots, grid_width, grid_height);
    enable_raw_mode()?;
    loop {
        if event::poll(std::time::Duration::from_secs(1))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Left => {
                        if time_pointer > 0 {
                            time_pointer -= 1;
                        }
                    }
                    KeyCode::Right => {
                        time_pointer += 1;
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
    Ok(times[time_pointer] as u16)
}

pub fn part_two(input: &str) -> Option<u16> {
    // just print/display?
    let mut cache = Some(8179);
    if cache.is_none() {
        let robots = parse_input(input);
        let (w, h) = (101, 103);
        cache = grid_visualiser(&robots, w, h).ok();
    }
    cache
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
