// Advent of Code: 2024, Day 3
// Problem: https://adventofcode.com/2024/day/3
// Solution:
//   Part 1: Parse a list of expressions of the form mul(x,y) and calculate the sum of all multiplications from these instructions
//   Part 2:

use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.lines().collect();

    // Create a regex to parse for all expessions of the format mul(x,y) where x and y are numbers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let line_totals = lines.iter().map(|line| {
        // Find all matches in the line
        let instructions = re.find_iter(line);
        // Iterate over all matches
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

    let total = line_totals.fold(0, |acc, x| acc + x);

    println!("Total: {}", total);
}
