use std::collections::HashSet;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let antenna_ids: HashSet<char> = input.chars().filter(|c| c != &'.').collect();
    let mut nodes: HashSet<(i32, i32)> = HashSet::new();
    for c_id in antenna_ids.iter() {
        let coords: Vec<(u32, u32)> = input
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == *c_id {
                            return Some((x as u32, y as u32));
                        }
                        None
                    })
                    .next()
            })
            .collect();

        for (i, (x, y)) in coords.iter().enumerate() {
            for j in i + 1..coords.len() {
                let (x2, y2) = coords[j];
                let dx = x2 as i32 - *x as i32;
                let dy = y2 as i32 - *y as i32;
                let node_1 = (x2 as i32 + dx, y2 as i32 + dy);
                let node_2 = (*x as i32 - dx, *y as i32 - dy);
                nodes.insert(node_1);
                nodes.insert(node_2);
            }
        }
    }
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    nodes.retain(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height);
    return Some(nodes.len() as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let antenna_ids: HashSet<char> = input.chars().filter(|c| c != &'.').collect();
    let mut nodes: HashSet<(i32, i32)> = HashSet::new();
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    for c_id in antenna_ids.iter() {
        let coords: Vec<(u32, u32)> = input
            .lines()
            .enumerate()
            .filter_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == *c_id {
                            return Some((x as u32, y as u32));
                        }
                        None
                    })
                    .next()
            })
            .collect();

        for (i, (x, y)) in coords.iter().enumerate() {
            for j in i + 1..coords.len() {
                let (x2, y2) = coords[j];
                let dx = x2 as i32 - *x as i32;
                let dy = y2 as i32 - *y as i32;
                for k in 0..=width {
                    let node_1 = (x2 as i32 + dx * k, y2 as i32 + dy * k);
                    let node_2 = (*x as i32 - dx * k, *y as i32 - dy * k);
                    if (node_1.0 < 0 || node_1.1 < 0 || node_1.0 >= width || node_1.1 >= height)
                        && (node_2.0 < 0 || node_2.1 < 0 || node_2.0 >= width || node_2.1 >= height)
                    {
                        break;
                    }
                    nodes.insert(node_1);
                    nodes.insert(node_2);
                }
            }
        }
    }
    nodes.retain(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height);
    return Some(nodes.len() as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
