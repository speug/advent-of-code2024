advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let row = line.chars().collect();
        rows.push(row);
    }
    rows
}

fn count_xmas(grid: Vec<Vec<char>>, i: usize, j: usize, height: usize, width: usize) -> u64 {
    let mut out = 0u64;
    let offsets: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1), // normal
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1), // diags
    ];
    let word = ['X', 'M', 'A', 'S'];
    for (dx, dy) in offsets {
        let mut next_idx: usize = 1;
        loop {
            let nx = i as isize + dx * next_idx as isize;
            let ny = j as isize + dy * next_idx as isize;
            if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                if word[next_idx] == grid[nx as usize][ny as usize] {
                    if next_idx == 3 {
                        out += 1;
                        break;
                    } else {
                        next_idx += 1;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let mut out = 0u64;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'X' {
                out += count_xmas(grid.clone(), i, j, height, width);
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
