use std::{collections::HashMap, fs};

pub fn read_input() -> String {
    fs::read_to_string("src/inputs/day01.txt").expect("Should have been able to read the file")
}

pub fn part1(input: &str) -> i64 {
    let (mut numbers1, mut numbers2): (Vec<i64>, Vec<i64>) = input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            Some((
                iter.next()?.parse::<i64>().ok()?,
                iter.next()?.parse::<i64>().ok()?
            ))
        })
        .unzip();

    numbers1.sort_unstable();
    numbers2.sort_unstable();

    numbers1.iter()
        .zip(numbers2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let (numbers1, numbers2): (Vec<i64>, Vec<i64>) = input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            Some((
                iter.next()?.parse::<i64>().ok()?,
                iter.next()?.parse::<i64>().ok()?
            ))
        })
        .unzip();

    let mut count_map: HashMap<i64, (i64, i64)> = HashMap::new();

    for &num in &numbers1 {
        count_map.entry(num)
            .and_modify(|(count1, _)| *count1 += 1)
            .or_insert((1, 0));
    }

    for &num in &numbers2 {
        count_map.entry(num)
            .and_modify(|(_, count2)| *count2 += 1)
            .or_insert((0, 1));
    }

    count_map.into_iter()
        .map(|(key, (count1, count2))| key * count1 * count2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3 4
                                 4 3
                                 2 5
                                 1 3
                                 3 9
                                 3 3";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part1_file() {
        let input = read_input();
        assert_eq!(part1(&input), 2113135);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 31);
    }

    #[test]
    fn test_part2_file() {
        let input = read_input();
        assert_eq!(part2(&input), 19097157);
    }
}