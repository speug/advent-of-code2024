advent_of_code::solution!(6);
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq)]
enum GuardStatus {
    In,
    Out,
    Turning,
    Looping,
}

#[derive(Debug)]
struct Guard<'a> {
    x: isize,
    y: isize,
    direction: Direction,
    grid: &'a Vec<Vec<char>>,
    grid_height: isize,
    grid_width: isize,
    visited: HashSet<((isize, isize), Direction)>,
}

impl<'a> Guard<'a> {
    fn new(x: isize, y: isize, direction: Direction, grid: &'a Vec<Vec<char>>) -> Self {
        Self {
            x,
            y,
            direction,
            grid,
            grid_height: grid.clone().len() as isize,
            grid_width: grid[0].clone().len() as isize,
            visited: HashSet::from([((x, y), direction)]),
        }
    }

    fn step(&mut self) -> GuardStatus {
        let (nx, ny) = match self.direction {
            Direction::North => (self.x - 1, self.y),
            Direction::East => (self.x, self.y + 1),
            Direction::South => (self.x + 1, self.y),
            Direction::West => (self.x, self.y - 1),
        };
        if nx < 0 || nx >= self.grid_height || ny < 0 || ny >= self.grid_width {
            GuardStatus::Out
        } else if self.grid[nx as usize][ny as usize] == '#'
            || self.grid[nx as usize][ny as usize] == 'O'
        {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            GuardStatus::Turning
        } else {
            self.x = nx;
            self.y = ny;
            if self.visited.insert(((nx, ny), self.direction)) {
                GuardStatus::In
            } else {
                GuardStatus::Looping
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut guardopt: Option<Guard> = None;
    for (i, row) in grid.clone().into_iter().enumerate() {
        for (j, elem) in row.into_iter().enumerate() {
            if elem == '^' {
                guardopt = Some(Guard::new(i as isize, j as isize, Direction::North, &grid));
            }
        }
    }
    let mut guard = guardopt.unwrap();
    loop {
        let status = guard.step();
        if status == GuardStatus::Out {
            break;
        }
    }
    // collect all unique visited positions
    let unique_positions: HashSet<(isize, isize)> = guard
        .visited
        .iter()
        .map(|((x, y), _)| (*x, *y))
        .collect::<HashSet<_>>();
    Some(unique_positions.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // first, have one guard map out all visited grid points
    let mut grid = parse_input(input);
    let mut guardopt: Option<Guard> = None;
    let mut orig_position_opt = None;
    for (i, row) in grid.clone().into_iter().enumerate() {
        for (j, elem) in row.into_iter().enumerate() {
            if elem == '^' {
                orig_position_opt = Some((i, j));
                guardopt = Some(Guard::new(i as isize, j as isize, Direction::North, &grid));
                break;
            }
        }
    }
    let mut guard = guardopt.unwrap();
    let orig_position = orig_position_opt.unwrap();
    loop {
        let status = guard.step();
        if status == GuardStatus::Out {
            break;
        }
    }
    let unique_positions: HashSet<(isize, isize)> = guard
        .visited
        .iter()
        .map(|((x, y), _)| (*x, *y))
        .collect::<HashSet<_>>();
    // then, for each visited point, add an obstacle
    // spawn a new guard and have them walk until out or looping
    let mut loops = 0u64;
    for (i, j) in unique_positions {
        if i as usize == orig_position.0 && j as usize == orig_position.1 {
            continue;
        }
        grid[i as usize][j as usize] = 'O';
        let mut new_guard = Guard::new(
            orig_position.0 as isize,
            orig_position.1 as isize,
            Direction::North,
            &grid,
        );
        loop {
            let status = new_guard.step();
            if status == GuardStatus::Out {
                break;
            } else if status == GuardStatus::Looping {
                loops += 1;
                break;
            }
        }
        grid[i as usize][j as usize] = '.';
    }
    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
