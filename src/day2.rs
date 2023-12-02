use aoc_parse::{parser, prelude::*};

#[derive(Debug, PartialEq)]
pub struct GameInfo {
    id: usize,
    distributions: Vec<GameDistribution>,
}

impl GameInfo {
    fn is_possible(&self) -> bool {
        self.distributions.iter().all(GameDistribution::is_possible)
    }

    fn fewest(&self) -> GameDistribution {
        self.distributions.iter().fold(
            GameDistribution {
                red: 0,
                green: 0,
                blue: 0,
            },
            |prev, curr| GameDistribution {
                red: prev.red.max(curr.red),
                green: prev.green.max(curr.green),
                blue: prev.blue.max(curr.blue),
            },
        )
    }
}

#[derive(Debug, PartialEq)]
struct GameDistribution {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameDistribution {
    pub fn new(set_colors: Vec<SetColor>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        set_colors.into_iter().for_each(|set| match set {
            SetColor::Red(x) => {
                red = x;
            }
            SetColor::Green(x) => {
                green = x;
            }
            SetColor::Blue(x) => {
                blue = x;
            }
        });
        GameDistribution { red, green, blue }
    }

    pub fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

enum SetColor {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<GameInfo> {
    let cube_red = parser!(usize " red");
    let cube_green = parser!(usize " green");
    let cube_blue = parser!(usize " blue");
    let cube = parser!({
        red:cube_red => SetColor::Red(red),
        green:cube_green => SetColor::Green(green),
        blue:cube_blue => SetColor::Blue(blue),
    });
    let cubes = repeat_sep(cube, ", ");
    let distributions = repeat_sep(cubes, "; ");
    let line = parser!("Game " usize ": " distributions);
    let full = lines(line);
    full.parse(input)
        .expect("Failed to parse input")
        .into_iter()
        .map(|(id, set_colors)| GameInfo {
            id,
            distributions: set_colors.into_iter().map(GameDistribution::new).collect(),
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[GameInfo]) -> usize {
    input
        .iter()
        .filter_map(|info| info.is_possible().then(|| info.id))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[GameInfo]) -> usize {
    input.iter().map(|info| info.fewest().power()).sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day2_parse() {
        assert_eq!(
            parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            vec![GameInfo {
                id: 1,
                distributions: vec![
                    GameDistribution {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    GameDistribution {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    GameDistribution {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ],
            }]
        );
    }

    const INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn day2() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 8);
        assert_eq!(part2(&input), 2286);
    }
}
