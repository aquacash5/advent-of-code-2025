use itertools::Itertools;
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
    fn is_line(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    fn area(&self) -> usize {
        self.a
            .x
            .abs_diff(self.b.x)
            .strict_add(1)
            .strict_mul(self.a.y.abs_diff(self.b.y).strict_add(1))
    }

    fn x_between(&self, other: &Rectangle) -> bool {
        let (a, b) = (self.a.x.min(self.b.x), self.a.x.max(self.b.x));
        ((a + 1)..b).contains(&other.a.x) || ((a + 1)..b).contains(&other.b.x)
    }

    fn y_between(&self, other: &Rectangle) -> bool {
        let (a, b) = (self.a.y.min(self.b.y), self.a.y.max(self.b.y));
        ((a + 1)..b).contains(&other.a.y) || ((a + 1)..b).contains(&other.b.y)
    }

    fn x_straddles(&self, other: &Rectangle) -> bool {
        let (a, b) = (self.a.x.min(self.b.x), self.a.x.max(self.b.x));
        let (c, d) = (other.a.x.min(other.b.x), other.a.x.max(other.b.x));
        c.le(&a) && d.ge(&b)
    }

    fn y_straddles(&self, other: &Rectangle) -> bool {
        let (a, b) = (self.a.y.min(self.b.y), self.a.y.max(self.b.y));
        let (c, d) = (other.a.y.min(other.b.y), other.a.y.max(other.b.y));
        c.le(&a) && d.ge(&b)
    }

    #[allow(clippy::nonminimal_bool)]
    fn intersects(&self, other: &Rectangle) -> bool {
        let x_between = self.x_between(other);
        let y_between = self.y_between(other);
        (x_between && y_between)
            || (x_between && self.y_straddles(other))
            || (y_between && self.x_straddles(other))
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
        .map(|(a, b)| Rectangle { a, b }.area())
        .max()
        .unwrap())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let lines = input
        .points
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(a, b)| Rectangle { a, b })
        .filter(Rectangle::is_line)
        .collect_vec();
    Ok(input
        .points
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| Rectangle { a, b })
        .filter(|r| !r.is_line())
        .filter(|r| !lines.iter().any(|l| r.intersects(l)))
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
