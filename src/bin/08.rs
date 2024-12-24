advent_of_code::solution!(8);
use advent_of_code::in_grid;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

fn build_station_catalogue(grid: &[Vec<char>]) -> HashMap<char, Vec<(isize, isize)>> {
    let mut stations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, label) in row.iter().enumerate() {
            if *label != '.' {
                stations
                    .entry(*label)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }
    stations
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let stations = build_station_catalogue(&grid);
    let mut antinodes = HashSet::new();
    for (_, antenna_coords) in stations.iter() {
        let antenna_combs = antenna_coords.iter().combinations(2);
        for antenna_pair in antenna_combs {
            let antenna_x_diff = antenna_pair[0].0 - antenna_pair[1].0;
            let antenna_y_diff = antenna_pair[0].1 - antenna_pair[1].1;
            let (antinode1_x, antinode1_y) = (
                antenna_pair[0].0 + antenna_x_diff,
                antenna_pair[0].1 + antenna_y_diff,
            );
            let (antinode2_x, antinode2_y) = (
                antenna_pair[1].0 - antenna_x_diff,
                antenna_pair[1].1 - antenna_y_diff,
            );
            if in_grid(antinode1_x, antinode1_y, height as isize, width as isize) {
                antinodes.insert((antinode1_x, antinode1_y));
            }
            if in_grid(antinode2_x, antinode2_y, height as isize, width as isize) {
                antinodes.insert((antinode2_x, antinode2_y));
            }
        }
    }
    Some(antinodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let stations = build_station_catalogue(&grid);
    let mut antinodes = HashSet::new();
    // just use while loops to find antinodes behind 1 and 2
    for (_, antenna_coords) in stations.iter() {
        let antenna_combs = antenna_coords.iter().combinations(2);
        for antenna_pair in antenna_combs {
            antinodes.insert(*antenna_pair[0]);
            antinodes.insert(*antenna_pair[1]);
            let antenna_x_diff = antenna_pair[0].0 - antenna_pair[1].0;
            let antenna_y_diff = antenna_pair[0].1 - antenna_pair[1].1;
            let (mut antinode1_x, mut antinode1_y) = (
                antenna_pair[0].0 + antenna_x_diff,
                antenna_pair[0].1 + antenna_y_diff,
            );
            while in_grid(antinode1_x, antinode1_y, height as isize, width as isize) {
                antinodes.insert((antinode1_x, antinode1_y));
                antinode1_x += antenna_x_diff;
                antinode1_y += antenna_y_diff;
            }
            let (mut antinode2_x, mut antinode2_y) = (
                antenna_pair[1].0 - antenna_x_diff,
                antenna_pair[1].1 - antenna_y_diff,
            );
            while in_grid(antinode2_x, antinode2_y, height as isize, width as isize) {
                antinodes.insert((antinode2_x, antinode2_y));
                antinode2_x -= antenna_x_diff;
                antinode2_y -= antenna_y_diff;
            }
        }
    }
    Some(antinodes.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
