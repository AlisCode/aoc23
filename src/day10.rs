use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    L,
    J,
    Seven,
    F,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_from(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
            Direction::South => (x, y + 1),
            Direction::North => (x, y - 1),
        }
    }

    fn rev(self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::North => Direction::South,
        }
    }
}

impl Tile {
    fn connections(&self) -> [Direction; 2] {
        match self {
            Tile::Vertical => [Direction::North, Direction::South],
            Tile::Horizontal => [Direction::West, Direction::East],
            Tile::L => [Direction::North, Direction::East],
            Tile::J => [Direction::North, Direction::West],
            Tile::Seven => [Direction::South, Direction::West],
            Tile::F => [Direction::South, Direction::East],
            Tile::Start => panic!("Asked for connections on tile Start"),
        }
    }

    fn next_direction(&self, coming_from: Direction) -> Option<Direction> {
        match self.connections() {
            [from, to] if from == coming_from => Some(to),
            [to, from] if from == coming_from => Some(to),
            _ => None,
        }
    }
}

fn neighbors((x, y): (i32, i32)) -> [((i32, i32), Direction); 4] {
    [
        ((x + 1, y), Direction::West),
        ((x - 1, y), Direction::East),
        ((x, y + 1), Direction::North),
        ((x, y - 1), Direction::South),
    ]
}

struct FollowPath<'a> {
    input: &'a HashMap<(i32, i32), Tile>,
    pos: (i32, i32),
    coming_from: Direction,
}

impl<'a> Iterator for FollowPath<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let tile = self.input.get(&self.pos)?;
        if tile == &Tile::Start {
            return None;
        }
        let dir = tile.next_direction(self.coming_from)?;
        let next_pos = dir.move_from(self.pos);
        self.coming_from = dir.rev();
        self.pos = next_pos;
        Some(next_pos)
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> HashMap<(i32, i32), Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let tile = match c {
                    '|' => Some(Tile::Vertical),
                    '-' => Some(Tile::Horizontal),
                    'L' => Some(Tile::L),
                    'J' => Some(Tile::J),
                    '7' => Some(Tile::Seven),
                    'F' => Some(Tile::F),
                    'S' => Some(Tile::Start),
                    _ => None,
                };
                tile.map(|tile| ((x as i32, y as i32), tile))
            })
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &HashMap<(i32, i32), Tile>) -> usize {
    let start = input
        .iter()
        .find_map(|(k, v)| (v == &Tile::Start).then(|| *k))
        .expect("Failed to find start position");
    let starting_neighbors: Vec<_> = neighbors(start)
        .into_iter()
        .filter_map(|(pos, coming_from)| {
            input
                .get(&pos)
                .and_then(|tile| tile.next_direction(coming_from).map(|_| (pos, coming_from)))
        })
        .collect();
    // sanity check
    assert_eq!(starting_neighbors.len(), 2);
    let [a, b] = starting_neighbors.as_slice() else {
        panic!("Impossible starting neighbors");
    };
    let path_a = FollowPath {
        input: &input,
        pos: a.0,
        coming_from: a.1,
    };
    let path_b = FollowPath {
        input: &input,
        pos: b.0,
        coming_from: b.1,
    };
    path_a
        .zip(path_b)
        .enumerate()
        .find_map(|(step, (pos_a, pos_b))| (pos_a == pos_b).then_some(step))
        .expect("Failed to find intersection of path")
        + 2 // to account for start and first position
}

#[aoc(day10, part2)]
fn part2(input: &HashMap<(i32, i32), Tile>) -> usize {
    let start = input
        .iter()
        .find_map(|(k, v)| (v == &Tile::Start).then(|| *k))
        .expect("Failed to find start position");
    let starting_neighbors: Vec<_> = neighbors(start)
        .into_iter()
        .filter_map(|(pos, coming_from)| {
            input
                .get(&pos)
                .and_then(|tile| tile.next_direction(coming_from).map(|_| (pos, coming_from)))
        })
        .collect();
    // sanity check
    assert_eq!(starting_neighbors.len(), 2);
    let (pos, coming_from) = starting_neighbors[0];
    let path = FollowPath {
        input: &input,
        pos,
        coming_from,
    };
    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;
    let path: HashSet<(i32, i32)> = path
        .chain(Some(start))
        .chain(Some(starting_neighbors[0].0))
        .chain(Some(starting_neighbors[1].0))
        .inspect(|(x, y)| {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        })
        .collect();

    let is_inside = |x: i32, y: i32| -> bool {
        if path.contains(&(x, y)) {
            return false;
        }

        let (north, south) = (min_x - 1..=x).fold((0, 0), |(north, south), xx| {
            if !path.contains(&(xx, y)) {
                return (north, south);
            }
            let connections = match input.get(&(xx, y)) {
                Some(Tile::Start) => {
                    // special case of start
                    [starting_neighbors[0].1.rev(), starting_neighbors[1].1.rev()]
                }
                Some(x) => x.connections(),
                None => panic!("Tile is in path"),
            };
            let north = north
                + if connections.contains(&Direction::North) {
                    1
                } else {
                    0
                };
            let south = south
                + if connections.contains(&Direction::South) {
                    1
                } else {
                    0
                };
            (north, south)
        });
        north.min(south) % 2 == 1
    };

    (min_x..=max_x)
        .map(move |x| (min_y..=max_y).filter(|y| is_inside(x, *y)).count())
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const SIMPLE: &'static str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const COMPLEX: &'static str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const PART_TWO_SAMPLE_ONE: &'static str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const PART_TWO_SAMPLE_TWO: &'static str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    #[test]
    fn day10() {
        // part1
        let simple = parse(SIMPLE);
        let complex = parse(COMPLEX);
        assert_eq!(part1(&simple), 4);
        assert_eq!(part1(&complex), 8);

        // part2
        let part_two_sample_one = parse(PART_TWO_SAMPLE_ONE);
        assert_eq!(part2(&part_two_sample_one), 4);

        let part_two_sample_two = parse(PART_TWO_SAMPLE_TWO);
        assert_eq!(part2(&part_two_sample_two), 8);
    }
}
