// Advent of Code: 2024, Day 3
// Problem: https://adventofcode.com/2024/day/3
// Solution:
//   Part 1: Parse a list of expressions of the form mul(x,y) and calculate the sum of all multiplications from these instructions
//   Part 2: Parse a list of expressions of the form mul(x,y), do() and don't() and calculate the sum of all multiplications from these instructions

use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.lines().collect();

    calculate_total_part_1(&lines);
    calculate_total_part_2(&lines);
}

fn calculate_total_part_1(lines: &Vec<&str>) {
    // Create a regex to parse for all expessions of the format mul(x,y) where x and y are numbers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let line_totals = lines.iter().map(|line| {
        let instructions = re.find_iter(line);

        let sums = instructions.map(|instruction| {
            let operands_string = instruction.as_str().replace("mul(", "").replace(")", "");

            let operands = operands_string
                .split(",")
                .map(|x| x.parse::<i32>().unwrap());

            let multiplication = operands.fold(1, |acc, x| acc * x);
            multiplication
        });

        let line_total = sums.fold(0, |acc, x| acc + x);
        line_total
    });

    // Output: 184122457
    let total = line_totals.fold(0, |acc, x| acc + x);
    println!("Part 1 Total: {}", total);
}

fn calculate_total_part_2(lines: &Vec<&str>) {
    // Create a regex to parse for all expessions of the format mul(x,y) where x and y are numbers
    // And also parse for do() and don't() instructions
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let mut count_mul_instructions = true;

    let line_totals = lines.iter().map(|line| {
        let instructions = re.find_iter(line);

        let sums = instructions.map(|instruction| {
            let instruction_string = instruction.as_str();

            if instruction_string == "do()" {
                count_mul_instructions = true;
                return 0;
            }

            if instruction_string == "don't()" {
                count_mul_instructions = false;
                return 0;
            }

            if !count_mul_instructions {
                return 0;
            }

            let operands_string = instruction.as_str().replace("mul(", "").replace(")", "");

            let operands = operands_string
                .split(",")
                .map(|x| x.parse::<i32>().unwrap());

            let multiplication = operands.fold(1, |acc, x| acc * x);
            multiplication
        });

        let line_total = sums.fold(0, |acc, x| acc + x);
        line_total
    });

    // Output:
    let total = line_totals.fold(0, |acc, x| acc + x);
    println!("Part 2 Total: {}", total);
}
