use advent_of_code::get_neighboring_indices_2d;
use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|v| v.to_digit(10).unwrap() as u8)
            .collect();
        rows.push(row);
    }
    rows
}

fn calculate_trailhead_score(start_pos: (usize, usize), grid: &[Vec<u8>]) -> u64 {
    let mut end_locations = HashSet::new();
    let h = grid.len();
    let w = grid[0].len();
    fn inner(
        curr_pos: (usize, usize),
        target_height: u8,
        grid: &[Vec<u8>],
        height: &usize,
        width: &usize,
        ends: &mut HashSet<(usize, usize)>,
    ) {
        let neighbours = get_neighboring_indices_2d(curr_pos.0, curr_pos.1, height, width, false);
        for (nx, ny) in neighbours {
            if grid[nx][ny] == target_height {
                if target_height == 9 {
                    ends.insert((nx, ny));
                } else {
                    inner((nx, ny), target_height + 1, grid, height, width, ends);
                }
            }
        }
    }
    inner(start_pos, 1, grid, &h, &w, &mut end_locations);
    end_locations.len() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut out = 0u64;
    for (i, row) in grid.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == 0 {
                out += calculate_trailhead_score((i, j), &grid)
            }
        }
    }
    Some(out)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
