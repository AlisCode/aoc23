use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    value: i64,
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    x: i32,
}

#[derive(Debug)]
struct Schematic {
    numbers: HashMap<i32, Vec<Part>>,
    symbols: HashMap<i32, Vec<Symbol>>,
}

struct ParsingNumber {
    value: i64,
    x: i32,
}

// Probably the worst code of my carreer
#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let mut schematic = Schematic {
        numbers: Default::default(),
        symbols: Default::default(),
    };
    let mut y = 0;
    for line in input.lines() {
        let mut parsing_number: Option<ParsingNumber> = None;
        for (index, c) in line.chars().enumerate() {
            match c {
                '.' => match &mut parsing_number {
                    Some(ref mut number) => {
                        let ParsingNumber { value, x } = number;
                        schematic.numbers.entry(y).or_default().push(Part {
                            value: *value,
                            start: *x,
                            end: (index - 1) as i32,
                        });
                        parsing_number = None;
                    }
                    None => continue,
                },
                x if x.is_digit(10) => {
                    let parsed = (x as u8 - '0' as u8) as i64;
                    match &mut parsing_number {
                        Some(ref mut number) => {
                            number.value = number.value * 10 + parsed;
                        }
                        None => {
                            parsing_number = Some(ParsingNumber {
                                x: index as i32,
                                value: parsed,
                            });
                        }
                    }
                }
                symbol => {
                    schematic.symbols.entry(y).or_default().push(Symbol {
                        symbol,
                        x: index as i32,
                    });
                    match &mut parsing_number {
                        Some(ref mut number) => {
                            let ParsingNumber { value, x } = number;
                            schematic.numbers.entry(y).or_default().push(Part {
                                value: *value,
                                start: *x,
                                end: (index - 1) as i32,
                            });
                            parsing_number = None;
                        }
                        None => (),
                    }
                }
            }
        }
        match parsing_number {
            Some(ParsingNumber { value, x }) => {
                schematic.numbers.entry(y).or_default().push(Part {
                    value,
                    start: x,
                    end: (line.len() - 1) as i32,
                });
            }
            None => (),
        }
        y += 1;
    }
    schematic
}

#[aoc(day3, part1)]
fn part1(input: &Schematic) -> i64 {
    input
        .numbers
        .iter()
        .map(|(y, numbers)| {
            numbers
                .iter()
                .filter_map(|Part { value, start, end }| {
                    let range = start - 1..=end + 1;
                    // IDK why this couldnt work with an Iterator ?
                    for yy in y - 1..=y + 1 {
                        match input.symbols.get(&yy) {
                            Some(symbols) => {
                                if symbols
                                    .iter()
                                    .any(|Symbol { symbol: _, x }| range.contains(x))
                                {
                                    return Some(value);
                                }
                            }
                            None => (),
                        }
                    }
                    None
                })
                .sum::<i64>()
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Schematic) -> i64 {
    input
        .symbols
        .iter()
        .map(|(y, symbols)| {
            symbols
                .iter()
                .filter_map(|Symbol { symbol, x }| {
                    if *symbol != '*' {
                        return None;
                    }
                    let parts: Vec<i64> = (y - 1..=y + 1)
                        .flat_map(|yy| match input.numbers.get(&yy) {
                            Some(numbers) => numbers
                                .iter()
                                .filter_map(|Part { value, start, end }| {
                                    if (start - 1..=end + 1).contains(x) {
                                        Some(*value)
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            None => vec![],
                        })
                        .collect();
                    match parts.len() {
                        2 => Some(parts.into_iter().product::<i64>()),
                        _ => None,
                    }
                })
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn day3() {
        let input = parse(INPUT);
        dbg!(&input);
        assert_eq!(part1(&input), 4361);
        assert_eq!(part2(&input), 467835);
    }
}
