use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut warehouse: Vec<Vec<char>> = input
        .lines()
        .filter_map(|s: &str| {
            if s.contains('#') {
                return Some(s.chars().collect());
            }
            None
        })
        .collect();
    let instructions: Vec<char> = input
        .lines()
        .filter_map(|s: &str| {
            if !s.contains('#') {
                return Some(s.chars().collect::<Vec<char>>());
            }
            None
        })
        .flatten()
        .collect();
    let mut robot: (usize, usize) = (0, 0);
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                robot = (i, j);
            }
        }
    }
    //println!("{}", char_matrix_to_string_matrix(&warehouse));

    for instruction in instructions {
        let direction: (i32, i32) = match instruction {
            '^' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => panic!(),
        };
        let mut next_pos = (
            (robot.0 as i32 + direction.0) as usize,
            (robot.1 as i32 + direction.1) as usize,
        );
        if warehouse[next_pos.1][next_pos.0] == '#' {
            continue;
        } else if warehouse[next_pos.1][next_pos.0] == '.' {
            warehouse[robot.1][robot.0] = '.';
            robot = next_pos;
            warehouse[robot.1][robot.0] = '@';
        } else {
            let mut blocks_to_move: Vec<(usize, usize)> = Vec::new();
            while warehouse[next_pos.1][next_pos.0] == 'O' {
                blocks_to_move.push(next_pos);
                next_pos = (
                    (next_pos.0 as i32 + direction.0) as usize,
                    (next_pos.1 as i32 + direction.1) as usize,
                );
            }
            if warehouse[next_pos.1][next_pos.0] == '#' {
                continue;
            }
            for block in blocks_to_move.clone() {
                warehouse[block.1][block.0] = '.';
            }
            for block in blocks_to_move {
                warehouse[(block.1 as i32 + direction.1) as usize]
                    [(block.0 as i32 + direction.0) as usize] = 'O';
            }
            warehouse[robot.1][robot.0] = '.';
            robot = (
                (robot.0 as i32 + direction.0) as usize,
                (robot.1 as i32 + direction.1) as usize,
            );
            warehouse[robot.1][robot.0] = '@';
        }
    }

    //println!("{}", char_matrix_to_string_matrix(&warehouse));

    return Some(get_gps(&warehouse));
}

fn char_matrix_to_string_matrix(char_matrix: &Vec<Vec<char>>) -> String {
    return char_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn get_gps(warehouse: &Vec<Vec<char>>) -> u32 {
    let mut gps = 0;
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' || *c == '[' {
                gps += 100 * i + j;
            }
        }
    }
    return gps as u32;
}

pub fn part_two(input: &str) -> Option<u32> {
    let init_warehouse: Vec<Vec<char>> = input
        .lines()
        .filter_map(|s: &str| {
            if s.contains('#') {
                return Some(s.chars().collect());
            }
            None
        })
        .collect();
    let instructions: Vec<char> = input
        .lines()
        .filter_map(|s: &str| {
            if !s.contains('#') {
                return Some(s.chars().collect::<Vec<char>>());
            }
            None
        })
        .flatten()
        .collect();

    let mut warehouse: Vec<Vec<char>> = Vec::new();
    for line in init_warehouse {
        let mut new_line: Vec<char> = Vec::new();
        for c in line {
            match c {
                '@' => {
                    new_line.push('@');
                    new_line.push('.');
                }
                'O' => {
                    new_line.push('[');
                    new_line.push(']');
                }
                '.' => {
                    new_line.push('.');
                    new_line.push('.');
                }
                '#' => {
                    new_line.push('#');
                    new_line.push('#');
                }
                _ => (),
            }
        }
        warehouse.push(new_line);
    }

    println!("{}", char_matrix_to_string_matrix(&warehouse));

    let mut robot: (usize, usize) = (0, 0);
    for (i, row) in warehouse.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                robot = (j, i);
            }
        }
    }

    for instruction in instructions {
        let direction: (i32, i32) = match instruction {
            '^' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => panic!(),
        };
        let next_pos = (
            (robot.0 as i32 + direction.0) as usize,
            (robot.1 as i32 + direction.1) as usize,
        );
        if warehouse[next_pos.1][next_pos.0] == '#' {
            continue;
        } else if warehouse[next_pos.1][next_pos.0] == '.' {
            warehouse[robot.1][robot.0] = '.';
            robot = next_pos;
            warehouse[robot.1][robot.0] = '@';
        } else {
            let mut blocks_to_move: HashMap<(usize, usize), char> = HashMap::new();
            let mut spaces_to_check: VecDeque<(usize, usize)> = VecDeque::new();
            spaces_to_check.push_back(next_pos);
            let mut can_move = true;
            while !spaces_to_check.is_empty() {
                let space = spaces_to_check.pop_front().unwrap();
                if blocks_to_move.contains_key(&space) {
                    continue;
                }
                let next_space = (
                    (space.0 as i32 + direction.0) as usize,
                    (space.1 as i32 + direction.1) as usize,
                );
                match warehouse[space.1][space.0] {
                    '#' => can_move = false,
                    '[' => {
                        blocks_to_move.insert(space, '[');
                        spaces_to_check.push_back(next_space);
                        let other_block: (usize, usize) = (space.0 + 1, space.1);
                        spaces_to_check.push_back(other_block);
                    }
                    ']' => {
                        blocks_to_move.insert(space, ']');
                        spaces_to_check.push_back(next_space);
                        let other_block: (usize, usize) = (space.0 - 1, space.1);
                        spaces_to_check.push_back(other_block);
                    }
                    _ => (),
                }
            }
            if !can_move {
                continue;
            }
            for (block, _) in blocks_to_move.clone() {
                warehouse[block.1][block.0] = '.';
            }
            for (block, char) in blocks_to_move {
                warehouse[(block.1 as i32 + direction.1) as usize]
                    [(block.0 as i32 + direction.0) as usize] = char;
            }
            warehouse[robot.1][robot.0] = '.';
            robot = (
                (robot.0 as i32 + direction.0) as usize,
                (robot.1 as i32 + direction.1) as usize,
            );
            warehouse[robot.1][robot.0] = '@';
        }
    }
    println!("{}", char_matrix_to_string_matrix(&warehouse));
    return Some(get_gps(&warehouse) as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
