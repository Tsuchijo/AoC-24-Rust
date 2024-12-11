advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut files = input
        .chars()
        .step_by(2)
        .enumerate()
        .map(|(i, c)| {
            let file_size = c.to_digit(10).unwrap();
            let mut vec: Vec<u32> = Vec::new();
            for _ in 0..file_size {
                vec.push(i as u32);
            }
            vec
        })
        .flatten()
        .collect::<Vec<u32>>();

    let spaces: Vec<(u32, u32)> = input
        .chars()
        .skip(1)
        .step_by(2)
        .zip(
            input
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .scan(0, |acc, d| {
                    *acc = *acc + d;
                    Some(*acc)
                })
                .step_by(2),
        )
        .filter_map(|(space_size, index)| match space_size.to_digit(10) {
            Some(x) => {
                if x != 0 {
                    Some((x, index))
                } else {
                    None
                }
            }
            None => None,
        })
        .collect();

    spaces.iter().for_each(|(space_size, index)| {
        for i in 0..*space_size {
            let file = match files.pop() {
                Some(x) => x,
                None => {
                    return;
                }
            };
            if *index + i > (files.len()) as u32 {
                files.push(file);
            } else {
                files.insert((*index + i) as usize, file);
            }
        }
    });
    //println!("{:?}", files);
    let checksum = files
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc as u64 + *x as u64 * i as u64);
    return Some(checksum);
}

pub fn part_two(input: &str) -> Option<u64> {
    // filespace is a tuple of (file_size, file_id, file_index)

    let mut filespace: Vec<(u32, u32, u32)> = Vec::new();

    let nums = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut num_sum = nums
        .iter()
        .scan(0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect::<Vec<u32>>();

    num_sum.insert(0, 0);

    for i in 0..nums.len() {
        filespace.push((nums[i], i as u32, num_sum[i] as u32));
    }

    let mut spaces: Vec<(u32, u32)> = filespace
        .iter()
        .skip(1)
        .step_by(2)
        .map(|(x, _, z)| (*x, *z))
        .collect::<Vec<(u32, u32)>>();

    filespace = filespace
        .iter()
        .step_by(2)
        .map(|(x, y, z)| (*x, *y / 2, *z))
        .collect::<Vec<(u32, u32, u32)>>();

    let mut filled_spaces: Vec<(u32, u32, u32)> = Vec::new();
    filespace = filespace
        .iter()
        .rev()
        .filter(|(file_size, file_id, file_index)| {
            let mut found: bool = false;
            spaces = spaces
                .iter_mut()
                .filter_map(|(space_size, space_index)| {
                    if file_size <= space_size && !found && file_index > space_index {
                        filled_spaces.push((*file_size, *file_id, *space_index));
                        *space_size -= file_size;
                        *space_index += file_size;
                        found = true;
                        if space_size == &0 {
                            None
                        } else {
                            Some((*space_size, *space_index))
                        }
                    } else {
                        Some((*space_size, *space_index))
                    }
                })
                .collect();
            return !found;
        })
        .cloned()
        .collect();

    filespace.append(&mut filled_spaces);
    filespace.sort_by(|a, b| a.2.cmp(&b.2));
    let checksum: u64 = filespace.iter().fold(0, |acc, (size, id, index)| {
        acc + (id * (size * index + (size * (size + 1) / 2) - size)) as u64
    });
    return Some(checksum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
