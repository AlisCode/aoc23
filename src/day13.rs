#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(str::to_owned).collect())
        .collect()
}

fn parse_pattern(pattern: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut cols = vec![0; pattern[0].len()];
    let lines: Vec<u32> = pattern
        .iter()
        .enumerate()
        .map(|(j, line)| {
            let mut x = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    x |= 1 << i;
                    cols[i] |= 1 << j;
                }
            }
            x
        })
        .collect();

    (lines, cols)
}

fn scan_values(values: &[u32]) -> Option<usize> {
    (0..values.len() - 1)
        .find(|i| {
            let a = &values[0..*i + 1];
            let b = &values[*i + 1..];
            a.iter().rev().zip(b.iter()).all(|(aa, bb)| aa == bb)
        })
        .map(|x| x + 1)
}

fn scan_values_with_smudge(values: &[u32]) -> Option<usize> {
    (0..values.len() - 1)
        .find(|i| {
            let a = &values[0..*i + 1];
            let b = &values[*i + 1..];
            a.iter()
                .rev()
                .zip(b.iter())
                .map(|(aa, bb)| (*aa ^ *bb).count_ones())
                .sum::<u32>()
                == 1
        })
        .map(|x| x + 1)
}

fn scan(lines: &[u32], cols: &[u32], scanner: fn(&[u32]) -> Option<usize>) -> ScanResult {
    scanner(lines)
        .map(ScanResult::Horizontal)
        .or(scanner(cols).map(ScanResult::Vertical))
        .expect("Scan failed")
}

fn scan_and_summarize_notes(input: &[Vec<String>], scanner: fn(&[u32]) -> Option<usize>) -> usize {
    let mut vertical = 0;
    let mut horizontal = 0;
    for pattern in input {
        let (lines, cols) = parse_pattern(pattern);
        match scan(&lines, &cols, scanner) {
            ScanResult::Vertical(v) => vertical += v,
            ScanResult::Horizontal(h) => horizontal += h,
        }
    }
    100 * horizontal + vertical
}

#[derive(Debug, PartialEq)]
enum ScanResult {
    Horizontal(usize),
    Vertical(usize),
}

#[aoc(day13, part1)]
fn part1(input: &[Vec<String>]) -> usize {
    scan_and_summarize_notes(input, scan_values)
}

#[aoc(day13, part2)]
fn part2(input: &[Vec<String>]) -> usize {
    scan_and_summarize_notes(input, scan_values_with_smudge)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn day13() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 405);
        assert_eq!(part2(&input), 400);
    }

    #[test]
    fn day13_scan() {
        let input = parse(INPUT);

        let (lines, cols) = parse_pattern(&input[0]);
        assert_eq!(scan_values(&lines), None);
        assert_eq!(scan_values(&cols), Some(5));
        assert_eq!(scan(&lines, &cols, scan_values), ScanResult::Vertical(5));

        let (lines, cols) = parse_pattern(&input[1]);
        assert_eq!(scan_values(&lines), Some(4));
        assert_eq!(scan_values(&cols), None);
        assert_eq!(scan(&lines, &cols, scan_values), ScanResult::Horizontal(4));

        let (lines, cols) = parse_pattern(&input[0]);
        assert_eq!(scan_values_with_smudge(&lines), Some(3));
        assert_eq!(scan_values_with_smudge(&cols), None);
        assert_eq!(
            scan(&lines, &cols, scan_values_with_smudge),
            ScanResult::Horizontal(3)
        );

        let (lines, cols) = parse_pattern(&input[1]);
        assert_eq!(scan_values_with_smudge(&lines), Some(1));
        assert_eq!(scan_values_with_smudge(&cols), None);
        assert_eq!(
            scan(&lines, &cols, scan_values_with_smudge),
            ScanResult::Horizontal(1)
        );
    }
}
