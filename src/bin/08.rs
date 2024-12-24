advent_of_code::solution!(8);
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

fn distance(pos1: &(isize, isize), pos2: &(isize, isize)) -> isize {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    // build a list of stations
    let mut stations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, label) in row.iter().enumerate() {
            if *label != '.' {
                stations
                    .entry(*label)
                    .or_insert_with(Vec::new)
                    .push((i as isize, j as isize));
            }
        }
    }
    let mut antinodes = 0u64;
    // double counting antinodes here! should store antinodes in hashset
    for (station, antenna_coords) in stations.iter() {
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
            if antinode1_x >= 0
                && antinode1_x < height as isize
                && antinode1_y >= 0
                && antinode1_y < width as isize
            {
                antinodes += 1;
                println!(
                    "Antinode for station {:?} found at {:?}",
                    station,
                    (antinode1_x, antinode1_y)
                );
            }
            if antinode2_x >= 0
                && antinode2_x < height as isize
                && antinode2_y >= 0
                && antinode2_y < width as isize
            {
                antinodes += 1;
                println!(
                    "Antinode for station {:?} found at {:?}",
                    station,
                    (antinode2_x, antinode2_y)
                );
            }
        }
    }
    Some(antinodes)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
