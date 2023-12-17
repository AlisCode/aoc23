use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;

struct Map {
    map: HashMap<(i32, i32), i32>,
    width: i32,
    height: i32,
}

#[aoc_generator(day17)]
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
                let weight = c as i32 - '0' as i32;
                Some(((x as i32, y as i32), weight))
            })
        })
        .collect();
    Map { map, width, height }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MoveInDirection {
    None,
    Top(i32),
    Left(i32),
    Down(i32),
    Right(i32),
}

impl MoveInDirection {
    fn count_steps(self) -> i32 {
        match self {
            MoveInDirection::None => 0,
            MoveInDirection::Top(x) => x,
            MoveInDirection::Left(x) => x,
            MoveInDirection::Down(x) => x,
            MoveInDirection::Right(x) => x,
        }
    }

    fn is_going_back(self, direction: Direction) -> bool {
        match (self, direction) {
            (MoveInDirection::None, _) => false,
            (MoveInDirection::Top(_), Direction::Down) => true,
            (MoveInDirection::Left(_), Direction::Right) => true,
            (MoveInDirection::Down(_), Direction::Top) => true,
            (MoveInDirection::Right(_), Direction::Left) => true,
            _ => false,
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

    fn first_move(self) -> MoveInDirection {
        match self {
            Direction::Top => MoveInDirection::Top(1),
            Direction::Left => MoveInDirection::Left(1),
            Direction::Down => MoveInDirection::Down(1),
            Direction::Right => MoveInDirection::Right(1),
        }
    }
}

fn acc_move_in_dir(move_in_dir: MoveInDirection, direction: Direction) -> MoveInDirection {
    match (move_in_dir, direction) {
        (MoveInDirection::None, dir) => dir.first_move(),
        (MoveInDirection::Top(x), Direction::Top) => MoveInDirection::Top(x + 1),
        (MoveInDirection::Top(_), dir) => dir.first_move(),
        (MoveInDirection::Left(x), Direction::Left) => MoveInDirection::Left(x + 1),
        (MoveInDirection::Left(_), dir) => dir.first_move(),
        (MoveInDirection::Down(x), Direction::Down) => MoveInDirection::Down(x + 1),
        (MoveInDirection::Down(_), dir) => dir.first_move(),
        (MoveInDirection::Right(x), Direction::Right) => MoveInDirection::Right(x + 1),
        (MoveInDirection::Right(_), dir) => dir.first_move(),
    }
}

fn new_node_in_dir(
    node: &Node,
    direction: Direction,
    input: &Map,
    max_steps: i32,
) -> Option<(Node, i32)> {
    if node.move_in_dir.is_going_back(direction) {
        return None;
    }

    let delta = direction.delta();
    let pos = (node.pos.0 + delta.0, node.pos.1 + delta.1);
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= input.width || pos.1 > input.height {
        return None;
    }

    let move_in_dir = acc_move_in_dir(node.move_in_dir, direction);
    (move_in_dir.count_steps() <= max_steps).then(|| {
        (
            Node { pos, move_in_dir },
            *input
                .map
                .get(&pos)
                .expect("Failed to find weight at coords"),
        )
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    pos: (i32, i32),
    move_in_dir: MoveInDirection,
}

#[aoc(day17, part1)]
fn part1(input: &Map) -> i32 {
    let start = Node {
        pos: (0, 0),
        move_in_dir: MoveInDirection::None,
    };
    let success = |node: &Node| node.pos.0 == input.width - 1 && node.pos.1 == input.height;
    let successors = |node: &Node| {
        new_node_in_dir(node, Direction::Top, input, 3)
            .into_iter()
            .chain(new_node_in_dir(node, Direction::Left, input, 3))
            .chain(new_node_in_dir(node, Direction::Down, input, 3))
            .chain(new_node_in_dir(node, Direction::Right, input, 3))
    };
    let (_, cost) = dijkstra(&start, successors, success).expect("Failed to find path");
    cost
}

#[aoc(day17, part2)]
fn part2(input: &Map) -> i32 {
    let start = Node {
        pos: (0, 0),
        move_in_dir: MoveInDirection::None,
    };
    let success = |node: &Node| {
        node.pos.0 == input.width - 1
            && node.pos.1 == input.height
            && node.move_in_dir.count_steps() >= 4
    };
    let successors = |node: &Node| {
        match node.move_in_dir {
            // Special case for the start
            MoveInDirection::None => new_node_in_dir(node, Direction::Right, input, 10)
                .into_iter()
                .chain(new_node_in_dir(node, Direction::Down, input, 10))
                .collect::<Vec<_>>(),
            MoveInDirection::Top(x) if x < 4 => new_node_in_dir(node, Direction::Top, input, 10)
                .into_iter()
                .collect::<Vec<_>>(),
            MoveInDirection::Left(x) if x < 4 => new_node_in_dir(node, Direction::Left, input, 10)
                .into_iter()
                .collect::<Vec<_>>(),
            MoveInDirection::Down(x) if x < 4 => new_node_in_dir(node, Direction::Down, input, 10)
                .into_iter()
                .collect::<Vec<_>>(),
            MoveInDirection::Right(x) if x < 4 => {
                new_node_in_dir(node, Direction::Right, input, 10)
                    .into_iter()
                    .collect::<Vec<_>>()
            }
            _ => new_node_in_dir(node, Direction::Top, input, 10)
                .into_iter()
                .chain(new_node_in_dir(node, Direction::Left, input, 10))
                .chain(new_node_in_dir(node, Direction::Down, input, 10))
                .chain(new_node_in_dir(node, Direction::Right, input, 10))
                .collect::<Vec<_>>(),
        }
    };
    let (_, cost) = dijkstra(&start, successors, success).expect("Failed to find path");
    cost
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const INPUT_PART_TWO: &'static str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn day17() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 102);
        assert_eq!(part2(&input), 94);

        let input_part_two = parse(INPUT_PART_TWO);
        assert_eq!(part2(&input_part_two), 71);
    }
}
