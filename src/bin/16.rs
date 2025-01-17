advent_of_code::solution!(16);
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use advent_of_code::get_neighboring_indices_2d;

struct Vertex {
    pos: (usize, usize),
    passable: bool,
    start: bool,
    end: bool,
    repr: char,
    distance: u32,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pos.0.cmp(&other.pos.0))
            .then_with(|| self.pos.1.cmp(&other.pos.1))
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<Vec<Vertex>> {
    let mut grid = Vec::new();
    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        let mut row = Vec::new();
        for c in line.chars() {
            let v = Vertex {
                pos: (i, j),
                passable: c != '#',
                start: c == 'S',
                end: c == 'E',
                repr: c,
                distance: if c == 'S' { 0 } else { u32::MAX },
            };
            row.push(v);
            j += 1;
        }
        grid.push(row);
        i += 1;
    }
    grid
}

fn djikstra(grid: &mut [Vec<Vertex>]) -> u32 {
    let mut q = BinaryHeap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let h = grid.len();
    let w = grid[0].len();
    for row in grid.iter() {
        for v in row.into_iter() {
            if !v.start {
                q.push(*v);
            }
        }
    }
    while !q.is_empty() {
        let u = q.pop().unwrap();
        let direction: (isize, isize) = if u.start {
            (0, 1)
        } else {
            let prev_pos = prev.get(&u.pos).unwrap();
            (
                u.pos.0 as isize - prev_pos.0 as isize,
                u.pos.1 as isize - prev_pos.1 as isize,
            )
        };
        let neighs: Vec<(usize, usize)> =
            get_neighboring_indices_2d(u.pos.0, u.pos.1, &h, &w, false)
                .into_iter()
                .filter(|(x, y)| grid[*x][*y].passable)
                .collect();
        for v_coords in neighs {
            let mut v = grid
                .get_mut(v_coords.0)
                .and_then(|row| row.get_mut(v_coords.1))
                .unwrap();
            let dist = if (v.pos.0 as isize == u.pos.0 as isize + direction.0)
                && (v.pos.1 as isize == u.pos.1 as isize + direction.1)
            {
                1
            } else {
                1001
            };
            let alt = u.distance + dist;
            if alt < v.distance {
                prev.insert(v.pos, u.pos);
                v.distance = alt;
            }
        }
    }
    for row in grid.iter() {
        for v in row.into_iter() {
            if v.end {
                return v.distance;
            }
        }
    }
    unreachable!("Could not find end node!");
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
