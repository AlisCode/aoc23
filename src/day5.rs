use aoc_parse::{
    parser,
    prelude::{alpha, repeat_sep, u64},
    Parser,
};

#[derive(Debug, PartialEq)]
struct AlmanacRange {
    dst: u64,
    src: u64,
    len: u64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<AlmanacRange>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    let seeds = parser!("seeds: " repeat_sep(u64, " "));
    let range = parser!(dst:u64 " " src:u64 " " len:u64 => AlmanacRange { dst, src, len });
    let map = parser!(alpha* "-to-" alpha* " map:\n" repeat_sep(range, "\n"));
    let double_line_sep = parser!("\n\n");

    let full = parser!(
        seeds
        double_line_sep
        repeat_sep(map, double_line_sep)
    );

    let (seeds, _, maps) = full.parse(input).expect("Failed to parse input");
    let maps: Vec<Vec<AlmanacRange>> = maps
        .into_iter()
        .map(|(_ty_a, _ty_b, mut ranges)| {
            ranges.sort_by_key(|r| r.src);
            ranges
        })
        .collect();

    Almanac { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &Almanac) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| {
            input.maps.iter().fold(*seed, |acc, curr| {
                curr.iter()
                    .find_map(|range| {
                        (range.src..range.src + range.len)
                            .contains(&acc)
                            .then(|| range.dst + acc - range.src)
                    })
                    .unwrap_or(acc)
            })
        })
        .min()
        .expect("Failed to find minimum location")
}

#[aoc(day5, part2)]
fn part2(input: &Almanac) -> u64 {
    input
        .seeds
        .chunks(2)
        .filter_map(|pair| {
            let [start, len] = &pair else {
                panic!("chunks(2) should yield arrays of len 2")
            };
            input
                .maps
                .iter()
                .fold(vec![(*start, *len)], |acc, curr| {
                    acc.into_iter()
                        .flat_map(|(start, len)| {
                            // Some black magic going on here. Somehow this works.
                            // Goal: return all ranges mapped using the current map
                            let mut index = start;
                            let mut remaining_len = len;
                            let mut ranges = Vec::new();
                            for range in curr {
                                if remaining_len == 0 {
                                    return ranges;
                                }
                                if range.src >= index + remaining_len {
                                    // the remaining ranges are all after the input terminates.
                                    // Let's return.
                                    ranges.push((index, remaining_len));
                                    return ranges;
                                }
                                if range.src <= index {
                                    // skip all ranges that are before the range we're trying to
                                    // map
                                    if range.src + range.len < index {
                                        continue;
                                    }
                                    // index is contained in the range :
                                    // Add a mapped range from index to the end of the range or the
                                    // mapping range, whatever comes first
                                    let end = (range.src + range.len).min(index + remaining_len);
                                    let range_len = end - index;
                                    let eaten_off_range = index - range.src;
                                    ranges.push((range.dst + eaten_off_range, range_len));

                                    remaining_len -= range_len;
                                    index = end;
                                    continue;
                                }
                                if range.src >= index {
                                    // range start is included : we need to add an "unmapped" range
                                    ranges.push((index, range.src - index));
                                    let end = (range.src + range.len).min(index + remaining_len);
                                    let range_len = end - range.src;
                                    // and then a mapped range
                                    ranges.push((range.dst, range_len));
                                    remaining_len -= range_len + range.src - index;
                                    index = range.src + range.len;
                                }
                            }

                            // In case no ranges were found to map the current one
                            if ranges.len() == 0 {
                                vec![(index, remaining_len)]
                            } else {
                                ranges
                            }
                        })
                        .collect()
                })
                .into_iter()
                .map(|(seed, _)| seed)
                .min()
        })
        .min()
        .expect("Failed to find minimum location")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn day5_parse() {
        let input = parse(&INPUT);
        assert_eq!(input.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            input.maps[0],
            vec![
                AlmanacRange {
                    dst: 52,
                    src: 50,
                    len: 48
                },
                AlmanacRange {
                    dst: 50,
                    src: 98,
                    len: 2
                },
            ]
        );
    }

    #[test]
    fn day5() {
        let input = parse(&INPUT);
        assert_eq!(part1(&input), 35);
        assert_eq!(part2(&input), 46);
    }
}
