use std::collections::HashMap;

fn find_first_and_last(input: &str, prefix_to_value: &HashMap<&str, usize>) -> usize {
    let mut first = (0, usize::MAX);
    let mut last = (0, 0);

    for (prefix, value) in prefix_to_value {
        if let Some(index) = input.find(prefix) {
            if index < first.1 {
                first.0 = *value;
                first.1 = index;
            }
        }
        if let Some(index) = input.rfind(prefix) {
            if index >= last.1 {
                last.0 = *value;
                last.1 = index;
            }
        }
    }

    first.0 * 10 + last.0
}

#[aoc(day1, part1)]
fn part1<'a>(input: &str) -> usize {
    let prefix_to_value: HashMap<&str, usize> = maplit::hashmap![
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
    ];
    input
        .lines()
        .map(|line| find_first_and_last(line, &prefix_to_value))
        .sum()
}

#[aoc(day1, part2)]
fn part2<'a>(input: &str) -> usize {
    let prefix_to_value: HashMap<&str, usize> = maplit::hashmap![
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
    ];
    input
        .lines()
        .map(|line| find_first_and_last(line, &prefix_to_value))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT_ONE: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT_TWO: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    // This evaluates to 83.
    // I spent a good hour figuring that out.
    const THE_ANNOYING_CASE: &'static str = "eighthree";

    #[test]
    fn day1() {
        assert_eq!(part1(&INPUT_ONE), 142);
        assert_eq!(part2(&INPUT_TWO), 281);
        assert_eq!(part2(&THE_ANNOYING_CASE), 83);
    }
}
