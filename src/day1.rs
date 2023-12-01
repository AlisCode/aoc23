#[aoc(day1, part1)]
fn part1<'a>(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_digit(10));
            let first = chars.next().expect("No digit in line");
            let last = chars.last().unwrap_or_else(|| first);
            format!("{first}{last}")
                .parse::<i32>()
                .expect("Failed to parse number")
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

    // const INPUT_TWO: &'static str = "two1nine
    // eightwothree
    // abcone2threexyz
    // xtwone3four
    // 4nineeightseven2
    // zoneight234
    // 7pqrstsixteen";

    #[test]
    fn day1() {
        assert_eq!(part1(&INPUT_ONE), 142);
        // assert_eq!(part2(&INPUT_TWO), 281);
    }
}
