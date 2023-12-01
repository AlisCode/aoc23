use std::collections::HashMap;

#[aoc(day1, part1)]
fn part1<'a>(input: &str) -> i32 {
    input.lines().map(extract_first_and_last).sum()
}

fn extract_first_and_last(line: &str) -> i32 {
    let mut chars = line.chars().filter(|c| c.is_digit(10));
    let first = chars.next().expect("No digit in line");
    let last = chars.last().unwrap_or_else(|| first);
    format!("{first}{last}")
        .parse::<i32>()
        .expect("Failed to parse number")
}

#[aoc(day1, part2)]
fn part2<'a>(input: &str) -> i32 {
    let name_to_index: HashMap<&str, i32> = maplit::hashmap![
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
        .map(|line| {
            let mut digits = Vec::new();
            let mut i = 0;
            while i <= line.len() {
                for (prefix, value) in &name_to_index {
                    if line[i..].starts_with(prefix) {
                        digits.push(value);
                    }
                }
                i += 1;
            }
            digits[0] * 10 + *digits.last().expect("No digits in the line")
        })
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
