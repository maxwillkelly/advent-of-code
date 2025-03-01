// Advent of Code: 2024, Day 2
// Problem: https://adventofcode.com/2024/day/2
// Solution:
//   Part 1: Read the input file, split the lines into an array of strings, and then split the strings
//          into an array of integers. For each array of integers, check if the levels are increasing or
//          decreasing. If they are, check if the levels differ by one to three. If both conditions are
//          met, then the report is safe. Count the number of safe reports.
//   Part 2: For each report, remove one level at a time and check if the report is safe. If it is, then
//          count that as part of the number of safe reports with the problem dampner.

use std::fs;

fn main() {
    let file =
        fs::read_to_string("./input/input.txt").expect("Should have been able to read the file");

    let lines = file.lines();

    let reports = lines.map(|line| {
        let level_strings = line.split_whitespace().collect::<Vec<&str>>();
        let report = level_strings
            .iter()
            .map(|part| part.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        return report;
    });

    let number_of_safe_reports = reports.clone().fold(0, |accumulator, report| {
        let all_levels_increasing = are_all_levels_increasing(&report);
        let all_levels_decreasing = are_all_levels_decreasing(&report);

        if !(all_levels_increasing || all_levels_decreasing) {
            return accumulator;
        }

        let levels_differ_by_one_to_three = do_reports_differ_by_one_to_three(&report);

        if levels_differ_by_one_to_three {
            return accumulator + 1;
        }

        return accumulator;
    });

    let number_of_safe_reports_with_problem_dampner =
        reports.clone().fold(0, |accumulator, report| {
            for (index, _level) in report.clone().into_iter().enumerate() {
                let mut report_without_level = report.clone();
                report_without_level.remove(index);

                let all_levels_increasing = are_all_levels_increasing(&report_without_level);
                let all_levels_decreasing = are_all_levels_decreasing(&report_without_level);

                if !(all_levels_increasing || all_levels_decreasing) {
                    continue;
                }

                let levels_differ_by_one_to_three =
                    do_reports_differ_by_one_to_three(&report_without_level);

                if levels_differ_by_one_to_three {
                    return accumulator + 1;
                }
            }

            return accumulator;
        });

    // Output: 314
    println!("Number of safe reports: {}", number_of_safe_reports);
    println!(
        "Number of safe reports with problem dampner: {}",
        number_of_safe_reports_with_problem_dampner
    );
}

fn are_all_levels_increasing(report: &Vec<i32>) -> bool {
    for level in 1..report.len() {
        let current_level = report[level];
        let previous_level = report[level - 1];

        if current_level < previous_level {
            return false;
        }
    }

    return true;
}

fn are_all_levels_decreasing(report: &Vec<i32>) -> bool {
    for level in 1..report.len() {
        let current_level = report[level];
        let previous_level = report[level - 1];

        if current_level > previous_level {
            return false;
        }
    }

    return true;
}

fn do_reports_differ_by_one_to_three(report: &Vec<i32>) -> bool {
    for level in 1..report.len() {
        let current_level = report[level];
        let previous_level = report[level - 1];
        let difference = current_level - previous_level;
        let absolute_difference = difference.abs();

        if absolute_difference > 3 {
            return false;
        }

        if absolute_difference < 1 {
            return false;
        }
    }

    return true;
}
