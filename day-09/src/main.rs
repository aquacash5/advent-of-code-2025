use geo::{Contains, LineString, Polygon, Rect, coord};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.a
            .x
            .abs_diff(self.b.x)
            .strict_add(1)
            .strict_mul(self.a.y.abs_diff(self.b.y).strict_add(1))
    }
}

impl From<Rectangle> for Rect<f64> {
    fn from(Rectangle { a, b }: Rectangle) -> Self {
        Rect::new(
            coord! { x: a.x as f64, y: a.y as f64 },
            coord! { x: b.x as f64, y: b.y as f64 },
        )
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
        character::complete::{line_ending, usize},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
    };

    let point = map(separated_pair(usize, tag(","), usize), Point::from);
    map(separated_list1(line_ending, point), |points| InputData {
        points,
    })
    .parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .points
        .iter()
        .copied()
        .tuple_combinations()
        .par_bridge()
        .map(|(a, b)| Rectangle { a, b }.area())
        .max()
        .unwrap())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let tiles = Polygon::new(
        LineString::from(
            input
                .points
                .iter()
                .map(|Point { x, y }| (*x as f64, *y as f64))
                .collect_vec(),
        ),
        vec![],
    );

    Ok(input
        .points
        .iter()
        .copied()
        .tuple_combinations()
        .par_bridge()
        .map(|(a, b)| Rectangle { a, b })
        .filter(|r| tiles.contains(&Rect::<f64>::from(*r)))
        .map(|r| r.area())
        .max()
        .unwrap())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                points: vec![
                    Point { x: 7, y: 1 },
                    Point { x: 11, y: 1 },
                    Point { x: 11, y: 7 },
                    Point { x: 9, y: 7 },
                    Point { x: 9, y: 5 },
                    Point { x: 2, y: 5 },
                    Point { x: 2, y: 3 },
                    Point { x: 7, y: 3 }
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 50);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 24);
    }
}
