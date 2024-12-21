pub mod template;

// Use this file to add helper functions and additional modules.

pub fn get_neighboring_indices_2d(
    i: usize,
    j: usize,
    &height: &usize,
    &width: &usize,
    diagonals: bool,
) -> Vec<(usize, usize)> {
    let offsets: Vec<(isize, isize)> = if diagonals {
        Vec::from([
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1), // normal
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1), // diags
        ])
    } else {
        Vec::from([(-1, 0), (1, 0), (0, -1), (0, 1)])
    };
    offsets
        .iter()
        .filter_map(|&(dx, dy)| {
            let nx = i as isize + dx;
            let ny = j as isize + dy;
            if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .collect()
}
