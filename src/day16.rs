use maplit::hashset;
use std::collections::{HashMap, HashSet};

pub struct Map {
    map: HashMap<(i32, i32), Tile>,
    width: i32,
    height: i32,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Map {
    let mut height = 0;
    let mut width = 0;
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            height = y as i32;
            width = line.len() as i32;
            line.chars().enumerate().filter_map(move |(x, c)| {
                let tile = match c {
                    '|' => Some(Tile::Vertical),
                    '-' => Some(Tile::Horizontal),
                    '/' => Some(Tile::Slash),
                    '\\' => Some(Tile::BackSlash),
                    '.' => None,
                    _ => unimplemented!(),
                }?;
                Some(((x as i32, y as i32), tile))
            })
        })
        .collect();
    Map { map, width, height }
}

enum Tile {
    Vertical,
    Horizontal,
    Slash,
    BackSlash,
}

impl Tile {
    pub fn reflect(&self, dir: Direction) -> Vec<Direction> {
        match self {
            Tile::Vertical => match dir {
                Direction::Top | Direction::Down => vec![dir],
                Direction::Left | Direction::Right => vec![Direction::Top, Direction::Down],
            },
            Tile::Horizontal => match dir {
                Direction::Left | Direction::Right => vec![dir],
                Direction::Top | Direction::Down => vec![Direction::Left, Direction::Right],
            },
            Tile::Slash => match dir {
                Direction::Top => vec![Direction::Right],
                Direction::Left => vec![Direction::Down],
                Direction::Down => vec![Direction::Left],
                Direction::Right => vec![Direction::Top],
            },
            Tile::BackSlash => match dir {
                Direction::Top => vec![Direction::Left],
                Direction::Left => vec![Direction::Top],
                Direction::Down => vec![Direction::Right],
                Direction::Right => vec![Direction::Down],
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Top,
    Left,
    Down,
    Right,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Top => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct LightSource {
    start: (i32, i32),
    direction: Direction,
}

fn solve_for_start_position(start: LightSource, input: &Map) -> usize {
    let mut sources = vec![start.clone()];
    let mut seen = hashset![start];
    let mut energized: HashSet<(i32, i32)> = HashSet::default();

    while let Some(LightSource { start, direction }) = sources.pop() {
        let mut pos = start;
        let delta = direction.delta();
        'ray: loop {
            if pos.0 < 0 || pos.1 < 0 || pos.0 >= input.width || pos.1 > input.height {
                break 'ray;
            }
            energized.insert(pos);

            if let Some(tile) = input.map.get(&pos) {
                for s in tile.reflect(direction).into_iter().map(|dir| {
                    let delta = dir.delta();
                    let start = (pos.0 + delta.0, pos.1 + delta.1);
                    LightSource {
                        start,
                        direction: dir,
                    }
                }) {
                    if !seen.contains(&s) {
                        sources.push(s.clone());
                        seen.insert(s);
                    }
                }
                break 'ray;
            }

            pos.0 += delta.0;
            pos.1 += delta.1;
        }
    }
    energized.len()
}

#[aoc(day16, part1)]
fn part1(input: &Map) -> usize {
    solve_for_start_position(
        LightSource {
            start: (0, 0),
            direction: Direction::Right,
        },
        input,
    )
}

#[aoc(day16, part2)]
fn part2(input: &Map) -> usize {
    // Brute force. It's december 16th, we all have lives ok?
    // Idea to optimize: using a hashmap that stores LightSource -> resulting Ray (path stored as
    // hashset maybe?) to avoid recomputing paths we already went through.
    let right = (0..input.height).map(|y| LightSource {
        start: (0, y),
        direction: Direction::Right,
    });
    let top = (0..input.width).map(|x| LightSource {
        start: (x, input.height - 1),
        direction: Direction::Top,
    });
    let left = (0..input.height).map(|y| LightSource {
        start: (input.width - 1, y),
        direction: Direction::Left,
    });
    let down = (0..input.width).map(|x| LightSource {
        start: (x, 0),
        direction: Direction::Down,
    });
    right
        .chain(top)
        .chain(left)
        .chain(down)
        .map(|start| solve_for_start_position(start, input))
        .max()
        .expect("Failed to find max")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    const TRICKY: &'static str = r#"\.
..
..
.."#;

    #[test]
    fn day16() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 46);
        assert_eq!(part2(&input), 51);

        let tricky = parse(TRICKY);
        assert_eq!(part1(&tricky), 4);
    }
}
