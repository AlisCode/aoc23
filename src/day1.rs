use aho_corasick::AhoCorasick;

fn find_first_and_last(line: &str, finder: &AhoCorasick, patterns: &[&str]) -> i32 {
    let mut matches = finder.find_overlapping_iter(line);
    let first = matches.next().expect("No digits found").pattern();
    let last = matches.last().map(|f| f.pattern()).unwrap_or_else(|| first);
    to_digit(patterns[first]) * 10 + to_digit(patterns[last])
}

const PART1_PATTERNS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
const PART2_PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

#[aoc(day1, part1)]
fn part1<'a>(input: &str) -> i32 {
    let finder = AhoCorasick::new(PART1_PATTERNS).expect("Failed to build finder");
    input
        .lines()
        .map(|line| find_first_and_last(line, &finder, &PART1_PATTERNS))
        .sum()
}

#[aoc(day1, part2)]
fn part2<'a>(input: &str) -> i32 {
    let finder = AhoCorasick::new(PART2_PATTERNS).expect("Failed to build finder");
    input
        .lines()
        .map(|line| find_first_and_last(line, &finder, &PART2_PATTERNS))
        .sum()
}

fn to_digit(value: &str) -> i32 {
    match value {
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
        x => panic!("Unknown digit {x}"),
    }
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
