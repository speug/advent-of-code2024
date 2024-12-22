advent_of_code::solution!(6);
use advent_of_code::prettyprint_grid;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
struct Guard {
    orig_position: (isize, isize),
    position: (isize, isize),
    orig_direction: Direction,
    direction: Direction,
    grid: Vec<Vec<char>>,
    grid_height: isize,
    grid_width: isize,
    visited: u64,
}

impl Guard {
    fn new(x: isize, y: isize, direction: Direction, mut grid: Vec<Vec<char>>) -> Self {
        grid[x as usize][y as usize] = 'X';
        Self {
            orig_position: (x, y),
            position: (x, y),
            direction,
            orig_direction: direction,
            grid: grid.clone(),
            grid_height: grid.clone().len() as isize,
            grid_width: grid[0].clone().len() as isize,
            visited: 1,
        }
    }

    fn step(&mut self) -> GuardStatus {
        let (nx, ny) = match self.direction {
            Direction::North => (self.position.0 - 1, self.position.1),
            Direction::East => (self.position.0, self.position.1 + 1),
            Direction::South => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0, self.position.1 - 1),
        };
        if nx < 0 || nx >= self.grid_height || ny < 0 || ny >= self.grid_width {
            return GuardStatus::Out;
        } else if self.grid[nx as usize][ny as usize] == '#' {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            return GuardStatus::Turning;
        } else if (nx, ny) == self.orig_position && self.direction == self.orig_direction {
            return GuardStatus::Looping;
        } else {
            self.position = (nx, ny);
            if self.grid[nx as usize][ny as usize] != 'X' {
                self.visited += 1;
                self.grid[nx as usize][ny as usize] = 'X';
            }
            return GuardStatus::In;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(&input);
    let mut guardopt: Option<Guard> = None;
    for (i, row) in grid.clone().into_iter().enumerate() {
        for (j, elem) in row.into_iter().enumerate() {
            if elem == '^' {
                guardopt = Some(Guard::new(
                    i as isize,
                    j as isize,
                    Direction::North,
                    grid.clone(),
                ));
            }
        }
    }
    let mut guard = guardopt.unwrap();
    loop {
        let status = guard.step();
        if status == GuardStatus::Out {
            break;
        }
        // } else if status == GuardStatus::In {
        // println!("{}", prettyprint_grid(&guard.grid));
        // }
    }
    Some(guard.visited)
}

pub fn part_two(input: &str) -> Option<u64> {
    // find indices that can be visited
    // for each point visited, change into obstacle; loop until looping or out
    // iterate
    None
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
        assert_eq!(result, None);
    }
}
