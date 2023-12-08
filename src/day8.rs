use std::collections::HashMap;

use aoc_parse::{
    parser,
    prelude::{alnum, repeat_sep},
    Parser,
};

pub struct Map {
    network: HashMap<String, Node>,
    instructions: Vec<Instruction>,
}

enum Instruction {
    Left,
    Right,
}

struct NodeInfo {
    name: String,
    left: String,
    right: String,
}

struct Node {
    left: String,
    right: String,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Map {
    let instruction = parser!({
        "L" => Instruction::Left,
        "R" => Instruction::Right,
    });
    let instructions = parser!(instruction*);
    let node = parser!(name:alnum* " = (" left:alnum* ", " right:alnum* ")"
    => NodeInfo {
        name: name.into_iter().collect(),
        left: left.into_iter().collect(),
        right: right.into_iter().collect(),
    });
    let (instructions, nodes) =
        parser!(instructions:instructions "\n\n" nodes:repeat_sep(node, "\n")
        => (instructions, nodes ))
        .parse(input)
        .expect("Failed to parse input");
    let network: HashMap<String, Node> = nodes
        .iter()
        .map(|node| {
            (
                node.name.to_owned(),
                Node {
                    left: node.left.to_owned(),
                    right: node.right.to_owned(),
                },
            )
        })
        .collect();
    Map {
        network,
        instructions,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Map) -> usize {
    find_path_length_for_node(&input.network, &input.instructions, "AAA", |node| {
        node == "ZZZ"
    })
}

fn find_path_length_for_node<F: Fn(&str) -> bool>(
    network: &HashMap<String, Node>,
    instructions: &[Instruction],
    start: &str,
    finished: F,
) -> usize {
    let mut current = start;
    instructions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(step, instr)| {
            if finished(current) {
                return Some(step);
            }
            match instr {
                Instruction::Left => current = &network[current].left,
                Instruction::Right => current = &network[current].right,
            }
            None
        })
        .expect("Failed to find ZZZ")
}

#[aoc(day8, part2)]
fn part2(input: &Map) -> usize {
    input
        .network
        .keys()
        .filter_map(|name| {
            name.ends_with("A").then(|| {
                find_path_length_for_node(&input.network, &input.instructions, name, |node| {
                    node.ends_with("Z")
                })
            })
        })
        .reduce(|a, b| num::integer::lcm(a, b))
        .expect("Failed to find answer")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_TWO: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_THREE: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn day8() {
        let input = parse(INPUT);
        let input_two = parse(INPUT_TWO);
        let input_three = parse(INPUT_THREE);
        assert_eq!(part1(&input), 2);
        assert_eq!(part1(&input_two), 6);
        assert_eq!(part2(&input_three), 6);
    }
}
