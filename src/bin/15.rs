use std::collections::HashMap;

use advent_of_code::prettyprint_grid;

advent_of_code::solution!(15);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(isize, isize)>) {
    let mut grid = Vec::new();
    let mut lines = input.lines();
    loop {
        let row: Vec<char> = lines.next().unwrap().chars().collect();
        if row.is_empty() {
            break;
        }
        grid.push(row);
    }

    let mut direction_chars = Vec::new();
    while let Some(row) = lines.next() {
        let line_dirs: Vec<char> = row.chars().collect();
        direction_chars.extend(line_dirs);
    }
    let char_to_delta: HashMap<char, (isize, isize)> =
        [('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]
            .iter()
            .cloned()
            .collect();
    let directions = direction_chars
        .iter()
        .filter_map(|&c| char_to_delta.get(&c).copied())
        .collect();
    (grid, directions)
}

fn process_move(
    grid: &mut Vec<Vec<char>>,
    robot_position: &mut (usize, usize),
    direction: (isize, isize),
) {
    let candidate_pos = (
        robot_position.0 as isize + direction.0,
        robot_position.1 as isize + direction.1,
    );
    let candidate_tile = grid[candidate_pos.0 as usize][candidate_pos.1 as usize];
    let mut pointer = candidate_pos;
    loop {
        match grid[pointer.0 as usize][pointer.1 as usize] {
            '.' => {
                if candidate_tile == 'O' {
                    // swap stone into the free spot
                    grid[pointer.0 as usize][pointer.1 as usize] = 'O';
                }
                // move self to free pos
                grid[candidate_pos.0 as usize][candidate_pos.1 as usize] = '@';
                grid[robot_position.0][robot_position.1] = '.';
                *robot_position = (candidate_pos.0 as usize, candidate_pos.1 as usize);
                return;
            }
            '#' => {
                return;
            }
            'O' => {
                // continue to move the pointer
                pointer = (pointer.0 + direction.0, pointer.1 + direction.1);
            }
            _ => {
                unreachable!("Found a new character!")
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, directions) = parse_input(input);
    // find initial robot position
    let mut robot_opt = None;
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == '@') {
            robot_opt = Some((row_idx, col_idx));
        }
    }
    let mut robot_position = robot_opt.unwrap();
    for d in directions {
        process_move(&mut grid, &mut robot_position, d);
        // print!("{}", prettyprint_grid(&grid));
        // println!();
    }
    // calculate score
    let mut gps = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                gps += 100 * i + j;
            }
        }
    }
    Some(gps as u64)
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
