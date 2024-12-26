advent_of_code::solution!(9);
use std::iter::repeat;

fn parse_input(input: &str, debug: bool) -> Vec<i16> {
    let mut total_disk_map_len = 0usize;
    let mut file_indicator = 0i16;
    let mut out = Vec::new();

    for (i, elem) in input.chars().enumerate() {
        let block_len_opt = elem.to_digit(10);
        if block_len_opt.is_none() {
            continue;
        }
        let block_len = block_len_opt.unwrap() as usize;

        if i % 2 == 0 {
            out.extend(repeat(file_indicator).take(block_len));
            file_indicator += 1;
        } else {
            out.extend(repeat(-1).take(block_len));
        }
        total_disk_map_len += block_len;
    }
    if debug {
        println!("Total length of the disk map is {:?}", total_disk_map_len);
        println!("Maximum file indicator is {:?}", file_indicator);
    }
    out
}

fn compact_disk_map(dm: &mut [i16]) {
    let mut left = 0;
    let mut right = dm.len() - 1;
    while dm[left] != -1 {
        left += 1;
    }
    while dm[right] == -1 {
        right -= 1;
    }
    while left < right {
        dm.swap(left, right);
        left += 1;
        right -= 1;
        while dm[left] != -1 {
            left += 1
        }
        while dm[right] == -1 {
            right -= 1;
        }
    }
}

fn compact_whole_files(dm: &mut [i16]) {
    let max_file_indicator = *dm.iter().max().unwrap();
    let mut file_sizes = vec![(0, 0); (max_file_indicator + 1) as usize];
    let mut free_blocks = Vec::new();
    let dm_len = dm.len();
    let mut i = 0;
    while i < dm_len {
        let mut block_size = 0;
        if dm[i] == -1 {
            while i < dm_len && dm[i] == -1 {
                block_size += 1;
                i += 1;
            }
            free_blocks.push((block_size, i - block_size));
        } else {
            let file_indicator = dm[i];
            while i < dm_len && dm[i] == file_indicator {
                block_size += 1;
                i += 1;
            }
            file_sizes[file_indicator as usize] = (block_size, i - block_size);
        }
    }
    for fi in (0..=max_file_indicator).rev() {
        // println!("{}", disk_map_to_string(&dm));
        let (block_size, file_start) = file_sizes[fi as usize];
        let valid_free_block = free_blocks
            .iter()
            .position(|&(fs, free_start)| fs >= block_size && free_start < file_start);
        if valid_free_block.is_some() {
            let free_idx = valid_free_block.unwrap();
            let (mut free_size, mut free_start) = free_blocks[free_idx];
            for bi in 0..block_size {
                dm.swap(free_start, file_start + bi);
                free_size -= 1;
                free_start += 1;
            }
            free_blocks[free_idx] = (free_size, free_start);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dm = parse_input(input, false);
    compact_disk_map(&mut dm);
    let mut checksum = 0u64;
    for (i, elem) in dm.iter().enumerate() {
        if *elem != -1 {
            checksum += (i as u64) * (*elem as u64);
        }
    }
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dm = parse_input(input, false);
    compact_whole_files(&mut dm);
    let mut checksum = 0u64;
    for (i, elem) in dm.iter().enumerate() {
        if *elem != -1 {
            checksum += (i as u64) * (*elem as u64);
        }
    }
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
