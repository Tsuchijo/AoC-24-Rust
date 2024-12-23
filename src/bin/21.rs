use core::num;
use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let number_re = Regex::new(r"([0-9]+)").unwrap();
    let keypad: HashMap<char, (u32, u32)> = [
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]
    .iter()
    .cloned()
    .collect();

    let dir_pad: HashMap<char, (u32, u32)> = [
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('V', (1, 1)),
        ('>', (2, 1)),
    ].iter().cloned().collect();
    let num_layers = 2;
    let input_codes: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut complexities = 0;
    for code in input_codes {
        let mut position = keypad.get(&'A').unwrap().clone();
        let mut dirs: Vec<char> = Vec::new();
        for character in code.clone() {
            let next_position = keypad.get(&character).unwrap();
            dirs.append(&mut get_dirs(position, *next_position, true));
            position = *next_position;
        }
        println!("Code: {:?}, Dirs: {:?}", code, dirs.iter().collect::<String>());
        for _ in 0..num_layers {
            let mut position = dir_pad.get(&'A').unwrap().clone();
            let mut new_dirs: Vec<char> = Vec::new();
            for dir in dirs {
                let next_position = dir_pad.get(&dir).unwrap();
                new_dirs.append(&mut get_dirs(position, *next_position, false));
                position = *next_position;
            }
            dirs = new_dirs;
        }
        let numbers: u32 = number_re.captures(code.iter().collect::<String>().as_str()).unwrap()[0].parse().unwrap();
        complexities += numbers * dirs.len() as u32;
        println!("Number: {}, Length: {}", numbers, dirs.len());
        println!("dirs: {:?}", dirs.iter().collect::<String>()); 
    }
    return Some(complexities)
}

fn get_dirs(start_position: (u32, u32), end_position: (u32, u32), is_numpad: bool) -> Vec<char> {
    let mut dirs: Vec<char> = Vec::new();
    let (mut x, mut y) = start_position;
    let (end_x, end_y) = end_position;
    while x != end_x {
        if x < end_x {
            dirs.push('>');
            x += 1;
        } else {
            x -= 1;
            if (x,y) == (0,3) && is_numpad {
                // If overlaps reset and then do over, prioritizing vertical movement
                (x, y) = start_position;
                dirs.clear();
                while y != end_y {
                    if y < end_y {
                        y += 1;
                        dirs.push('V');
                    } else {
                        dirs.push('^');
                        y -= 1;
                    }
                }
            }
            else if !is_numpad && (x,y) == (0,0) {
                if start_position == (1,0){
                    dirs.push('V');
                    y += 1;
                }
                else {
                    dirs.pop();
                    dirs.push('V');
                    dirs.push('<');
                    y += 1;
                }
                dirs.push('<');
            }
            else {
                dirs.push('<');
            }
        }
    }
    while y != end_y {
        if y < end_y {
            y += 1;
            dirs.push('V');
        } else {
            dirs.push('^');
            y -= 1;
        }
    }
    dirs.push('A');
    return dirs;
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
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
