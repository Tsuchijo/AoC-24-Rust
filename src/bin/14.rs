advent_of_code::solution!(14);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut width = 102;
    let mut height = 104;
    let time = 100;

    if input.len() < 200 {
        width = 12;
        height = 8;
    }

    let re = Regex::new("p=([0-9]+),([0-9]+) v=([0-9]+)||(-[0-9]+),([0-9]+)||(-[0-9]+)").unwrap();
    let robots: Vec<(i32, i32, i32 , i32)> = re.captures_iter(input).map(|caps|{
        let p1 = caps[1].parse().unwrap();
        let p2 = caps[2].parse().unwrap();
        let v1 = caps[3].parse().unwrap();
        let v2 = caps[4].parse().unwrap();
        return (p1, p2, v1, v2);
    }).collect();

    println!("{:?}", robots);

    let final_robot: Vec<(i32, i32)> = robots.iter().map( |robot |{
        let (p1, p2, v1, v2) = robot;
        let new_1 = (((*p1 + *v1) * time) % width);
        let new_2 = (((*p2 + *v2) * time) % height);
        return (new_1, new_2)
    }).collect();

    let mut vis_matrix: Vec<Vec<char>> = Vec::new();
    for y in 0..height {
        vis_matrix.push(Vec::new());
        for x in 0..width {
            vis_matrix[y as usize].push('.')
        }
    }

    let mid_width = width/2;
    let mid_height = height/2;
    let mut quadrants = (0,0,0,0);

    for robot in final_robot {
        vis_matrix[robot.1 as usize][ robot.0 as usize] = '#';
        if robot.0 < mid_width && robot.1 < mid_height {
            quadrants.0 += 1;
        }
        if robot.0 > mid_width && robot.1 < mid_height {
            quadrants.1 += 1;
        }
        if robot.0 < mid_width && robot.1 > mid_height {
            quadrants.2 += 1;
        }
        if robot.0 > mid_width && robot.1 > mid_height {
            quadrants.3 += 1;
        }
    }

    println!("{}", char_matrix_to_string_matrix(&vis_matrix));

    return Some(quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3);

}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn char_matrix_to_string_matrix(char_matrix: &Vec<Vec<char>>) -> String {
    return char_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
