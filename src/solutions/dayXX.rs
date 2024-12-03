use std::{collections::HashMap, fs};

pub fn read_input() -> String {
    fs::read_to_string("src/inputs/day01.txt").expect("Should have been able to read the file")
}

pub fn part1(input: &str) -> i64 {
    
    todo!();
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
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part1_file() {
        let input = read_input();
        assert_eq!(part1(&input), 80085);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 80085);
    }

    #[test]
    fn test_part2_file() {
        let input = read_input();
        assert_eq!(part2(&input), 80085);
    }
}