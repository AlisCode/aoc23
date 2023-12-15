use aoc_parse::{
    parser,
    prelude::{alpha, usize},
    Parser,
};

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input.split(",").map(|seq| seq.replace("\n", "")).collect()
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> usize {
    input.iter().map(hash).sum()
}

fn hash(input: &String) -> usize {
    input
        .chars()
        .fold(0, |acc, curr| ((acc + curr as usize) * 17) % 256)
}

#[derive(Debug)]
enum Operation {
    Remove(String),
    Set(String, usize),
}

#[aoc(day15, part2)]
fn part2(input: &[String]) -> usize {
    let parse_remove = parser!(l:alpha+ "-" => Operation::Remove(l.into_iter().collect()));
    let parse_set = parser!(l:alpha+ "=" n:usize => Operation::Set(l.into_iter().collect(), n));
    let parse_op = parser!({
        op:parse_remove => op,
        op:parse_set => op,
    });
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for op in input {
        let op = parse_op.parse(op).expect("Failed to parse op");
        match op {
            Operation::Remove(label) => {
                let box_idx = hash(&label);
                boxes[box_idx] = boxes[box_idx]
                    .clone()
                    .into_iter()
                    .filter(|(l, _)| &label != l)
                    .collect();
            }
            Operation::Set(label, length) => {
                let box_idx = hash(&label);
                let mut added = false;
                for (l, len) in &mut boxes[box_idx] {
                    if l == &label {
                        *len = length;
                        added = true;
                    }
                }
                if !added {
                    boxes[box_idx].push((label, length));
                }
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, content)| {
            content
                .into_iter()
                .enumerate()
                .map(|(j, (_label, len))| (1 + i) * (j + 1) * len)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn day15() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 1320);
        assert_eq!(part2(&input), 145);
    }
}
