use aoc_parse::{
    parser,
    prelude::{i64, repeat_sep},
    Parser,
};

struct RaceInfo {
    time: i64,
    distance: i64,
}

// Parsing is reimplemented for part2. Nice troll, Eric.
#[aoc_generator(day6, part1)]
fn parse(input: &str) -> Vec<RaceInfo> {
    let manyspaces = parser!(" "*);
    let times = parser!("Time:" manyspaces times:repeat_sep(i64, manyspaces) => times);
    let distances =
        parser!("Distance:" manyspaces distances:repeat_sep(i64, manyspaces) => distances);
    let full = parser!(times "\n" distances);
    let (times, distances) = full.parse(input).expect("Failed to parse");
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| RaceInfo { time, distance })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[RaceInfo]) -> i64 {
    // NOTE: I don't know if any of this makes sense
    // to anyone reading this, but it's my train of thought
    // when figuring out the math. I'll leave it here for the record.
    //
    // x(pressed)
    // y(travel) = pressed*(time-pressed)
    // y = x*(time-x)
    // y = x*t - x*x

    // solve:
    // y = x*t - x*x - distance > 0 // NOTE: I mean >, not >=
    // y = -1x² + 1x*t - distance

    // form:
    // y = ax² + bx + c
    // with:
    // a = -1
    // b = t
    // c = -distance
    //
    // delta = b² - 4ac
    // delta = t*t - 4*distance
    // (-t +- sqrt(delta)) / -2
    //
    input
        .iter()
        .map(|race| {
            let delta = race.time * race.time - 4 * race.distance;
            let sqrt_delta = (delta as f64).sqrt();

            let sol1 = ((-race.time as f64 - sqrt_delta) / -2.).floor() as i64;
            // hack: if the distance is EXACTLY equal, we cant say we've BEATEN the record.
            // We need to press less.
            let distance_sol1 = sol1 * (race.time - sol1);
            let sol1 = if distance_sol1 == race.distance {
                sol1 - 1
            } else {
                sol1
            };

            let sol2 = ((-race.time as f64 + sqrt_delta) / -2.).ceil() as i64;
            let distance_sol2 = sol2 * (race.time - sol2);
            let sol2 = if distance_sol2 == race.distance {
                // hack: if the distance is EXACTLY equal, we cant say we've BEATEN the record
                // We need to press more.
                sol2 + 1
            } else {
                sol2
            };
            sol1 - sol2 + 1
        })
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse()
        .expect("Failed to parse number");
    let distance = lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse()
        .expect("Failed to parse number");
    part1(&vec![RaceInfo { time, distance }])
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn day6() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 288);
        assert_eq!(part2(INPUT), 71503);
    }
}
