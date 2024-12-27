use advent_of_code::get_neighboring_indices_2d;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(12);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

fn find_area_and_perimeter(
    start_pos: (usize, usize),
    grid: &[Vec<char>],
    h: &usize,
    w: &usize,
) -> (HashSet<(usize, usize)>, u16, u16) {
    let mut area_members: HashMap<(usize, usize), u16> = HashMap::new();

    fn inner(
        pos: (usize, usize),
        grid: &[Vec<char>],
        h: &usize,
        w: &usize,
        area_members: &mut HashMap<(usize, usize), u16>,
    ) {
        let neighbours = get_neighboring_indices_2d(pos.0, pos.1, h, w, false);
        let perimeter = neighbours
            .iter()
            .map(|c| {
                if grid[c.0][c.1] == grid[pos.0][pos.1] {
                    0
                } else {
                    1
                }
            })
            .sum::<u16>()
            + (4 - neighbours.len() as u16);
        area_members.insert(pos, perimeter);
        for (nx, ny) in neighbours
            .iter()
            .filter(|(i, j)| grid[pos.0][pos.1] == grid[*i][*j])
        {
            if !area_members.contains_key(&(*nx, *ny)) {
                inner((*nx, *ny), grid, h, w, area_members);
            }
        }
    }

    inner(start_pos, grid, h, w, &mut area_members);
    let area = area_members.clone().len() as u16;
    let perimeter = area_members.values().sum::<u16>();
    (area_members.keys().cloned().collect(), area, perimeter)
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut areas: Vec<(char, HashSet<(usize, usize)>)> = Vec::new();
    let mut out = 0u64;
    for i in 0..h {
        for j in 0..w {
            if !areas.iter().any(|(_, hm)| hm.contains(&(i, j))) {
                let (area_members, area, perimeter) =
                    find_area_and_perimeter((i, j), &grid, &h, &w);
                out += (area as u64 * perimeter as u64) as u64;
                areas.push((grid[i][j], area_members));
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
