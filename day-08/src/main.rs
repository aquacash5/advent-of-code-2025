use std::{collections::BTreeMap, ops::ControlFlow};

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn square_distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    points: Vec<Point>,
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        bytes::complete::tag,
        character::complete::{line_ending, u64},
        combinator::map,
        multi::separated_list1,
    };

    let point = map((u64, tag(","), u64, tag(","), u64), |(x, _, y, _, z)| {
        Point { x, y, z }
    });
    let points = separated_list1(line_ending, point);
    map(points, |points| InputData { points }).parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let (append, iterations) = if input.points.len() == 20 {
        (0, 10)
    } else {
        (1, 1000)
    };
    let points: BTreeMap<Point, usize> = input
        .points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    Ok(input
        .points
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| a.square_distance(b))
        .try_fold((1usize, points), |(count, mut points), (a, b)| {
            let a_c = points[a];
            let b_c = points[b];
            if count == iterations {
                ControlFlow::Break(points)
            } else if a_c == b_c {
                ControlFlow::Continue((count + append, points))
            } else {
                let min = a_c.min(b_c);
                points
                    .iter_mut()
                    .for_each(|(_p, c)| *c = if *c == a_c || *c == b_c { min } else { *c });
                ControlFlow::Continue((count + 1, points))
            }
        })
        .break_value()
        .unwrap()
        .values()
        .counts()
        .into_values()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    let points: BTreeMap<Point, usize> = input
        .points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    Ok(input
        .points
        .iter()
        .tuple_combinations()
        .sorted_by_key(|(a, b)| a.square_distance(b))
        .try_fold(points, |mut points, (a, b)| {
            let a_c = points[a];
            let b_c = points[b];
            if a_c == b_c {
                ControlFlow::Continue(points)
            } else {
                let min = a_c.min(b_c);
                points
                    .iter_mut()
                    .for_each(|(_p, c)| *c = if *c == a_c || *c == b_c { min } else { *c });
                if points.values().all_equal() {
                    ControlFlow::Break(a.x * b.x)
                } else {
                    ControlFlow::Continue(points)
                }
            }
        })
        .break_value()
        .unwrap())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                points: vec![
                    Point {
                        x: 162,
                        y: 817,
                        z: 812
                    },
                    Point {
                        x: 57,
                        y: 618,
                        z: 57
                    },
                    Point {
                        x: 906,
                        y: 360,
                        z: 560
                    },
                    Point {
                        x: 592,
                        y: 479,
                        z: 940
                    },
                    Point {
                        x: 352,
                        y: 342,
                        z: 300
                    },
                    Point {
                        x: 466,
                        y: 668,
                        z: 158
                    },
                    Point {
                        x: 542,
                        y: 29,
                        z: 236
                    },
                    Point {
                        x: 431,
                        y: 825,
                        z: 988
                    },
                    Point {
                        x: 739,
                        y: 650,
                        z: 466
                    },
                    Point {
                        x: 52,
                        y: 470,
                        z: 668
                    },
                    Point {
                        x: 216,
                        y: 146,
                        z: 977
                    },
                    Point {
                        x: 819,
                        y: 987,
                        z: 18
                    },
                    Point {
                        x: 117,
                        y: 168,
                        z: 530
                    },
                    Point {
                        x: 805,
                        y: 96,
                        z: 715
                    },
                    Point {
                        x: 346,
                        y: 949,
                        z: 466
                    },
                    Point {
                        x: 970,
                        y: 615,
                        z: 88
                    },
                    Point {
                        x: 941,
                        y: 993,
                        z: 340
                    },
                    Point {
                        x: 862,
                        y: 61,
                        z: 35
                    },
                    Point {
                        x: 984,
                        y: 92,
                        z: 344
                    },
                    Point {
                        x: 425,
                        y: 690,
                        z: 689
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 40);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 25272);
    }
}
