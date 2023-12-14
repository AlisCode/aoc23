use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Platform {
    width: usize,
    height: usize,
    stones: Vec<Stone>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Stone {
    x: usize,
    y: usize,
    kind: StoneKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum StoneKind {
    Round,
    Cube,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Platform {
    let mut width = 0;
    let mut height = 0;
    let mut stones = Vec::new();
    for (y, line) in input.lines().enumerate() {
        width = line.len();
        height += 1;
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => stones.push(Stone {
                    x,
                    y,
                    kind: StoneKind::Cube,
                }),
                'O' => stones.push(Stone {
                    x,
                    y,
                    kind: StoneKind::Round,
                }),
                '.' => (),
                _ => unreachable!(),
            }
        }
    }
    Platform {
        width,
        height,
        stones,
    }
}

impl Platform {
    pub fn tilt_north(&mut self) {
        self.stones.sort_by_key(|stone| (stone.x, stone.y));
        let mut obstacles = vec![0; self.width];
        for s in &mut self.stones {
            match s.kind {
                StoneKind::Cube => obstacles[s.x] = s.y + 1,
                StoneKind::Round => {
                    s.y = obstacles[s.x];
                    obstacles[s.x] = s.y + 1;
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        self.stones.sort_by_key(|stone| (stone.y, stone.x));
        let mut obstacles = vec![0; self.height];
        for s in &mut self.stones {
            match s.kind {
                StoneKind::Cube => obstacles[s.y] = s.x + 1,
                StoneKind::Round => {
                    s.x = obstacles[s.y];
                    obstacles[s.y] = s.x + 1;
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        self.stones
            .sort_by(|a, b| a.x.cmp(&b.x).then_with(|| b.y.cmp(&a.y)));
        let mut obstacles = vec![self.height - 1; self.width];
        for s in &mut self.stones {
            match s.kind {
                StoneKind::Cube => {
                    obstacles[s.x] = s.y.max(1) - 1;
                }
                StoneKind::Round => {
                    s.y = obstacles[s.x];
                    obstacles[s.x] = s.y.max(1) - 1;
                }
            }
        }
    }

    pub fn tilt_east(&mut self) {
        self.stones
            .sort_by(|a, b| a.y.cmp(&b.y).then_with(|| b.x.cmp(&a.x)));
        let mut obstacles = vec![self.width - 1; self.height];
        for s in &mut self.stones {
            match s.kind {
                StoneKind::Cube => obstacles[s.y] = s.x.max(1) - 1,
                StoneKind::Round => {
                    s.x = obstacles[s.y];
                    obstacles[s.y] = s.x.max(1) - 1;
                }
            }
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn load(&self) -> usize {
        self.stones
            .iter()
            .map(|stone| match stone.kind {
                StoneKind::Round => self.height - stone.y,
                StoneKind::Cube => 0,
            })
            .sum()
    }
}

#[aoc(day14, part1)]
fn part1(platform: &Platform) -> usize {
    let mut platform = platform.clone();
    platform.tilt_north();
    platform.load()
}

#[aoc(day14, part2)]
fn part2(platform: &Platform) -> usize {
    // Cycle detection of when we've seen the same Platform already.
    // Then infer the state at step 1_000_000_000
    let mut platform = platform.clone();
    let mut cache = Vec::new();
    let mut platforms_seen = HashMap::new();
    let first_repeat = (0..)
        .find_map(|_| {
            platform.cycle();
            match platforms_seen.entry(platform.clone()) {
                Entry::Vacant(vacant) => {
                    let idx = cache.len();
                    cache.push(platform.clone());
                    vacant.insert(idx);
                    None
                }
                Entry::Occupied(occupied) => Some(*occupied.get()),
            }
        })
        .expect("Failed to find a repeating platform");
    let index = (1_000_000_000 - cache.len() - 1) % (cache.len() - first_repeat) + first_repeat;
    cache[index].load()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn day14() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 136);
        assert_eq!(part2(&input), 64);
    }
}
