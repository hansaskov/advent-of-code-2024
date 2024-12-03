use std::{collections::HashMap, fs};

pub fn read_input() -> String {
    fs::read_to_string("src/inputs/day02.txt").expect("Should have been able to read the file")
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let report: Vec<_> = line
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect();

            let diff_report: Vec<_> = report
                .windows(2)
                .map(|levels| levels[0] - levels[1])
                .collect();

            let is_all_ascending = diff_report.iter().all(|v| v.is_positive());
            let is_all_decending = diff_report.iter().all(|v| v.is_negative());
            let is_more_than_0 = diff_report.iter().all(|v| v.abs() >= 1);
            let is_less_than_4 = diff_report.iter().all(|v| v.abs() <= 3);

            if (is_all_ascending || is_all_decending) && is_more_than_0 && is_less_than_4 {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
                                 1 2 7 8 9
                                 9 7 6 2 1
                                 1 3 2 4 5
                                 8 6 4 4 1
                                 1 3 6 7 9";

    #[test]
    fn day02_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    

    #[test]
    fn day02_part1_file() {
        let input = read_input();
        assert_eq!(part1(&input), 390);
    }

    /*
    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 80085);
    }

    #[test]
    fn test_part2_file() {
        let input = read_input();
        assert_eq!(part2(&input), 80085);
    }
    */
}
