use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut reg_b = 0;
    let mut reg_c = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let mut reg_a = lines[0]
        .split("Register A: ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let instructions: Vec<u64> = lines[4][9..]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut program_counter = 0;
    let mut output: Vec<u64> = Vec::new();
    while program_counter < instructions.len() {
        match instructions[program_counter] {
            0 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_a = reg_a / 2_u64.pow(operand as u32);
                program_counter += 2;
            }

            1 => {
                let operand = instructions[program_counter + 1];
                reg_b = reg_b ^ operand;
                program_counter += 2;
            }

            2 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_b = operand % 8;
                program_counter += 2;
            }

            3 => {
                let operand = instructions[program_counter + 1];
                if reg_a == 0 {
                    program_counter += 2;
                } else {
                    program_counter = operand as usize;
                }
            }

            4 => {
                reg_b = reg_b ^ reg_c;
                program_counter += 2;
            }

            5 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                output.push(operand % 8);
                program_counter += 2;
            }

            6 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_b = reg_a / 2_u64.pow(operand as u32);
                program_counter += 2;
            }

            7 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_c = reg_a / 2_u64.pow(operand as u32);
                program_counter += 2;
            }

            _ => {}
        }
    }
    let mut output_string = output
        .iter()
        .map(|x| {
            let mut str = (*x).to_string();
            str.push(',');
            return str;
        })
        .collect::<String>();
    output_string.pop();
    //println!("{:?}", output_string);
    Some(output_string)
}

fn combo_operand(operand: u64, reg_a: u64, reg_b: u64, reg_c: u64) -> u64 {
    return match operand {
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => operand,
    };
}

fn test_program(reg_a: u64, reg_b: u64, reg_c: u64, instructions: Vec<u64>) -> Vec<u64> {
    let mut program_counter = 0;
    let mut output: Vec<u64> = Vec::new();
    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;
    while program_counter < instructions.len() {
        match instructions[program_counter] {
            0 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_a = reg_a / 2_u64.pow(operand as u32);
                program_counter += 2;
            }

            1 => {
                let operand = instructions[program_counter + 1];
                reg_b = reg_b ^ operand;
                program_counter += 2;
            }

            2 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_b = operand % 8;
                program_counter += 2;
            }

            3 => {
                let operand = instructions[program_counter + 1];
                if reg_a == 0 {
                    program_counter += 2;
                } else {
                    program_counter = operand as usize;
                }
            }

            4 => {
                reg_b = reg_b ^ reg_c;
                program_counter += 2;
            }

            5 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                output.push(operand % 8);
                program_counter += 2;
            }

            6 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_b = reg_a / 2_u64.pow(operand as u32);
                program_counter += 2;
            }

            7 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_c = reg_a / 2_u64.pow(operand as u32);
                program_counter += 2;
            }

            _ => {}
        }
    }
    return output;
}

pub fn part_two(input: &str) -> Option<u64> {
    //let mut reg_a: u64 = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let instructions: Vec<u64> = lines[4][9..]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut register_stack: Vec<u64> = Vec::new();
    register_stack.push(0);
    for instruction in instructions.clone().iter().rev() {
        //println!("Finding Instruction: {:?}", instruction);
        let mut place_holder: Vec<u64> = Vec::new();
        for i in (0..8) {
            for reg_a in register_stack.clone() {
                let test_reg_a = reg_a * 8 + i;
                let output = test_program(test_reg_a, 0, 0, instructions.clone());
                if output[0] == *instruction {
                    //println!("Output: {:?}", output);
                    place_holder.push(reg_a * 8 + i);
                }
            }
        }
        register_stack = place_holder;
    }
    let reg_a = *register_stack.iter().min().unwrap();
    // println!("{:?}", test_program(reg_a, 0, 0, instructions));
    // println!("{:?}", register_stack);
    return Some(reg_a as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let test: String = "4,6,3,5,6,3,5,2,1,0".to_string();
        //assert_eq!(result, Some(test));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
