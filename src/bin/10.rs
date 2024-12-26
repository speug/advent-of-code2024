use advent_of_code::get_neighboring_indices_2d;
use colored::Colorize;
use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|v| v.to_digit(10).unwrap_or(10) as u8)
            .collect();
        rows.push(row);
    }
    rows
}

fn count_distinct_endpoints(start_pos: (usize, usize), grid: &[Vec<u8>]) -> u64 {
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
                out += count_distinct_endpoints((i, j), &grid)
            }
        }
    }
    Some(out)
}

fn flatten_routes(routes: &HashSet<Vec<(usize, usize)>>) -> HashSet<(usize, usize)> {
    routes.iter().flat_map(|vec| vec.iter().cloned()).collect()
}

fn debug_print_routes(
    start_pos: (usize, usize),
    grid: &[Vec<u8>],
    routes: &HashSet<Vec<(usize, usize)>>,
) {
    let flat_routes = flatten_routes(routes);
    let w = grid[0].len();
    for (i, row) in grid.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if (i, j) == start_pos {
                print!("{}", elem.to_string().green().bold());
            } else if flat_routes.contains(&(i, j)) {
                print!("{}", elem.to_string().red());
            } else {
                let to_print = if *elem < 10 {
                    elem.to_string()
                } else {
                    ".".to_string()
                };
                print!("{}", to_print);
            }
            if j == w - 1 {
                println!();
            }
        }
    }
}

fn count_distinct_routes(start_pos: (usize, usize), grid: &[Vec<u8>], debug: bool) -> u64 {
    let mut routes = HashSet::new();
    let h = grid.len();
    let w = grid[0].len();
    let route = vec![start_pos];
    fn inner(
        curr_pos: (usize, usize),
        target_height: u8,
        grid: &[Vec<u8>],
        height: &usize,
        width: &usize,
        ends: &mut HashSet<Vec<(usize, usize)>>,
        route: &[(usize, usize)],
    ) {
        let neighbours = get_neighboring_indices_2d(curr_pos.0, curr_pos.1, height, width, false);
        for (nx, ny) in neighbours {
            if grid[nx][ny] == target_height {
                if target_height == 9 {
                    let mut new_route = route.to_owned();
                    new_route.push((nx, ny));
                    ends.insert(new_route.to_vec());
                } else {
                    let mut new_route = route.to_owned();
                    new_route.push((nx, ny));
                    inner(
                        (nx, ny),
                        target_height + 1,
                        grid,
                        height,
                        width,
                        ends,
                        &new_route,
                    );
                }
            }
        }
    }
    inner(start_pos, 1, grid, &h, &w, &mut routes, &route);
    if debug {
        debug_print_routes(start_pos, grid, &routes);
    }
    routes.len() as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut out = 0u64;
    for (i, row) in grid.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == 0 {
                out += count_distinct_routes((i, j), &grid, false)
            }
        }
    }
    Some(out)
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
        assert_eq!(result, Some(81));
    }
}
