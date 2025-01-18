advent_of_code::solution!(16);
use std::collections::{HashMap, HashSet};
use std::u32;

use advent_of_code::get_neighboring_indices_2d;

type Vertex = ((usize, usize), (isize, isize));
fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        grid.push(row);
    }
    grid
}

fn djikstra(grid: &[Vec<char>]) -> u32 {
    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut q: Vec<(usize, usize)> = Vec::new();
    let h = grid.len();
    let w = grid[0].len();
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                '.' | 'E' => {
                    q.push((i, j));
                    distances.insert((i, j), u32::MAX);
                }
                'S' => {
                    q.push((i, j));
                    distances.insert((i, j), 0);
                }
                _ => {}
            }
        }
    }
    while !q.is_empty() {
        let u = *q.iter().min_by_key(|pos| distances[pos]).unwrap();
        if let Some(min_pos) = q.iter().position(|&pos| (pos.0 == u.0) && (pos.1 == u.1)) {
            q.remove(min_pos);
        }
        let direction: (isize, isize) = if grid[u.0][u.1] == 'S' {
            (0, 1)
        } else {
            let prev_pos = prev.get(&u).unwrap();
            (
                u.0 as isize - prev_pos.0 as isize,
                u.1 as isize - prev_pos.1 as isize,
            )
        };
        let neighs: Vec<(usize, usize)> = get_neighboring_indices_2d(u.0, u.1, &h, &w, false)
            .into_iter()
            .filter(|&(x, y)| grid[x][y] != '#')
            .collect();
        for v in neighs {
            let dist = if (v.0 as isize == u.0 as isize + direction.0)
                && (v.1 as isize == u.1 as isize + direction.1)
            {
                1
            } else {
                1001
            };
            let alt = distances.get(&u).unwrap() + dist;
            if alt < *distances.get(&v).unwrap() {
                prev.insert(v, u);
                distances.insert(v, alt);
            }
        }
    }
    for (i, row) in grid.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if *v == 'E' {
                return *distances.get(&(i, j)).unwrap();
            }
        }
    }
    unreachable!("Could not find end node!");
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let end_distance = djikstra(&grid);
    Some(end_distance)
}

fn dfs(prev: &HashMap<Vertex, Vec<Vertex>>, end: Vertex) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut s = Vec::new();
    s.push(end);
    while let Some(v) = s.pop() {
        if visited.insert(v) {
            if let Some(prevs) = prev.get(&v) {
                for w in prevs {
                    if !visited.contains(&w) {
                        s.push(*w)
                    }
                }
            }
        }
    }
    let visited_coords: HashSet<(usize, usize)> = visited.iter().map(|(pos, _)| *pos).collect();
    visited_coords
}

fn djikstra_all(grid: &[Vec<char>]) -> u32 {
    // initialise collections
    let mut distances: HashMap<Vertex, u32> = HashMap::new();
    let mut prev: HashMap<Vertex, Vec<Vertex>> = HashMap::new();
    // this is a collection which just holds all the vertices; used to iterate over
    let mut vertices: HashSet<Vertex> = HashSet::new();
    // collection of univisited nodes
    let mut q: Vec<Vertex> = Vec::new();
    // collect end vertices for the end check
    let mut end_vertices = Vec::new();
    // E, S, W, N
    let deltas = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                '.' => {
                    for d in deltas.clone().into_iter() {
                        let previous_tile =
                            &grid[(i as isize - d.0) as usize][(j as isize - d.1) as usize];
                        // check if the coord - direction combo can exist
                        if *previous_tile != '#' {
                            q.push(((i, j), d));
                            distances.insert(((i, j), d), u32::MAX);
                            vertices.insert(((i, j), d));
                        }
                    }
                }
                'E' => {
                    for d in deltas.clone().into_iter() {
                        let previous_tile =
                            &grid[(i as isize - d.0) as usize][(j as isize - d.1) as usize];
                        // check if the coord - direction combo can exist
                        if *previous_tile != '#' {
                            q.push(((i, j), d));
                            distances.insert(((i, j), d), u32::MAX);
                            vertices.insert(((i, j), d));
                            end_vertices.push(((i, j), d));
                        }
                    }
                }
                'S' => {
                    // the start vertex only has a single direction (east)
                    q.push(((i, j), (0, 1)));
                    distances.insert(((i, j), (0, 1)), 0);
                    vertices.insert(((i, j), (0, 1)));
                }
                _ => {}
            }
        }
    }
    while !q.is_empty() {
        // pop u for Q with the lowest distance
        let (u_idx, u): (usize, Vertex) = q
            .iter()
            .enumerate()
            .min_by_key(|(_, v)| distances[v])
            .map(|(idx, v)| (idx, *v))
            .unwrap();
        q.swap_remove(u_idx);
        let u_direction: (isize, isize) = u.1;
        let neighs: Vec<Vertex> = deltas
            .iter()
            .filter_map(|&(dx, dy)| {
                // check if the neighbour exists in the collection of vertices
                let nx = (u.0.0 as isize + dx) as usize;
                let ny = (u.0.1 as isize + dy) as usize;
                let nv = ((nx, ny), (dx, dy));
                if vertices.contains(&nv) {
                    Some(nv)
                } else {
                    None
                }
            })
            .collect();
        for nv in neighs {
            let dist = if (nv.0.0 as isize == (u.0.0 as isize + u_direction.0))
                && (nv.0.1 as isize == (u.0.1 as isize + u_direction.1))
            {
                1
            } else {
                1001
            };
            let alt = distances.get(&u).unwrap() + dist;
            if alt <= *distances.get(&nv).unwrap_or(&u32::MAX) {
                prev.entry(nv).or_insert_with(Vec::new).push(u);
                if distances.insert(nv, alt).is_none() {
                    // we should never add any new distances at this point!
                    panic!(
                        "Inserted a new value into the distances dict: {:?} (u was {:?})!",
                        nv, u
                    );
                };
            }
        }
    }
    let mut all_visits: HashSet<(usize, usize)> = HashSet::new();
    // find the shortest path to end
    let mut min_distance = u32::MAX;
    for v in end_vertices.iter() {
        if let Some(dist) = distances.get(&v) {
            if *dist <= min_distance {
                min_distance = *dist;
            }
        }
    }
    // combine the shortest paths (in case you can enter the goal from 2 directions)
    for v in end_vertices.iter() {
        if let Some(dist) = distances.get(&v) {
            if *dist == min_distance {
                let visited = dfs(&prev, *v);
                all_visits.extend(visited);
            }
        }
    }
    all_visits.len() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let visited = djikstra_all(&grid);
    Some(visited)
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
        assert_eq!(result, Some(45));
    }
}
