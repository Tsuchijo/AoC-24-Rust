advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut warehouse: Vec<Vec<char>> = input.lines().filter_map(|s: &str| {
        if s.contains('#') {
            return Some(s.chars().collect())
        }
        None
    }).collect();
    let instructions: Vec<char> = input.lines().filter_map(|s: &str| {
        if !s.contains('#') {
            return Some(s.chars().collect::<Vec<char>>())
        }
        None
    }).flatten().collect();
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
            '>' => (1,0),
            'v' => (0, 1),
            _ => panic!()
        };
        let mut next_pos = ((robot.0 as i32 + direction.0) as usize, (robot.1 as i32 + direction.1) as usize);
        if warehouse[next_pos.1][next_pos.0] == '#' {
            continue;
        }  
        else if warehouse[next_pos.1][next_pos.0] == '.' {
            warehouse[robot.1][robot.0]  = '.';
            robot = next_pos;
            warehouse[robot.1][robot.0]  = '@';
        }
        else {
            let mut blocks_to_move: Vec<(usize, usize)> = Vec::new();
            while warehouse[next_pos.1][next_pos.0] == 'O' {
                blocks_to_move.push(next_pos);
                next_pos = ((next_pos.0 as i32 + direction.0) as usize, (next_pos.1 as i32 + direction.1) as usize);
            }
            if warehouse[next_pos.1][next_pos.0] == '#' {
                continue;
            }
            for block in blocks_to_move.clone() {
                warehouse[block.1][block.0] = '.';
            }
            for block in blocks_to_move {
                warehouse[(block.1 as i32 + direction.1) as usize][(block.0 as i32 + direction.0) as usize] = 'O';
            }
            warehouse[robot.1][robot.0]  = '.';
            robot = ((robot.0 as i32 + direction.0) as usize, (robot.1 as i32 + direction.1) as usize);
            warehouse[robot.1][robot.0]  = '@';
        }
    }

    println!("{}", char_matrix_to_string_matrix(&warehouse));

    return Some(get_gps(&warehouse))
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
            if *c == 'O' {
                gps += 100*i + j;
            }
        }
    }
    return gps as u32;
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
