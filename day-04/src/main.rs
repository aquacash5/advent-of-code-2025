use std::collections::BTreeSet;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn neighbors(&self) -> Vec<Self> {
        vec![
            self.up().left(),
            self.up(),
            self.up().right(),
            self.left(),
            self.right(),
            self.down().left(),
            self.down(),
            self.down().right(),
        ]
    }

    fn up(&self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct InputData {
    warehouse: BTreeSet<Point>,
}

impl InputData {
    fn neighbors_count(&self, point: &Point) -> usize {
        point
            .neighbors()
            .into_iter()
            .filter(|loc| self.warehouse.contains(loc))
            .count()
    }
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    Ok((
        "",
        InputData {
            warehouse: input
                .lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.bytes().enumerate().filter_map(move |(col, c)| {
                        if c == b'@' {
                            Some(Point {
                                row: row as i32,
                                col: col as i32,
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        },
    ))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .warehouse
        .iter()
        .filter(|p| input.neighbors_count(p) < 4)
        .count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let mut count = 0usize;
    let mut changed = true;
    let mut input = input.clone();

    while changed {
        let warehouse: BTreeSet<Point> = input
            .warehouse
            .iter()
            .filter(|p| input.neighbors_count(p) >= 4)
            .copied()
            .collect();
        changed = warehouse.len() != input.warehouse.len();
        count += input.warehouse.len() - warehouse.len();
        input.warehouse = warehouse;
    }

    Ok(count)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                warehouse: [
                    Point { row: 0, col: 2 },
                    Point { row: 0, col: 3 },
                    Point { row: 0, col: 5 },
                    Point { row: 0, col: 6 },
                    Point { row: 0, col: 7 },
                    Point { row: 0, col: 8 },
                    Point { row: 1, col: 0 },
                    Point { row: 1, col: 1 },
                    Point { row: 1, col: 2 },
                    Point { row: 1, col: 4 },
                    Point { row: 1, col: 6 },
                    Point { row: 1, col: 8 },
                    Point { row: 1, col: 9 },
                    Point { row: 2, col: 0 },
                    Point { row: 2, col: 1 },
                    Point { row: 2, col: 2 },
                    Point { row: 2, col: 3 },
                    Point { row: 2, col: 4 },
                    Point { row: 2, col: 6 },
                    Point { row: 2, col: 8 },
                    Point { row: 2, col: 9 },
                    Point { row: 3, col: 0 },
                    Point { row: 3, col: 2 },
                    Point { row: 3, col: 3 },
                    Point { row: 3, col: 4 },
                    Point { row: 3, col: 5 },
                    Point { row: 3, col: 8 },
                    Point { row: 4, col: 0 },
                    Point { row: 4, col: 1 },
                    Point { row: 4, col: 3 },
                    Point { row: 4, col: 4 },
                    Point { row: 4, col: 5 },
                    Point { row: 4, col: 6 },
                    Point { row: 4, col: 8 },
                    Point { row: 4, col: 9 },
                    Point { row: 5, col: 1 },
                    Point { row: 5, col: 2 },
                    Point { row: 5, col: 3 },
                    Point { row: 5, col: 4 },
                    Point { row: 5, col: 5 },
                    Point { row: 5, col: 6 },
                    Point { row: 5, col: 7 },
                    Point { row: 5, col: 9 },
                    Point { row: 6, col: 1 },
                    Point { row: 6, col: 3 },
                    Point { row: 6, col: 5 },
                    Point { row: 6, col: 7 },
                    Point { row: 6, col: 8 },
                    Point { row: 6, col: 9 },
                    Point { row: 7, col: 0 },
                    Point { row: 7, col: 2 },
                    Point { row: 7, col: 3 },
                    Point { row: 7, col: 4 },
                    Point { row: 7, col: 6 },
                    Point { row: 7, col: 7 },
                    Point { row: 7, col: 8 },
                    Point { row: 7, col: 9 },
                    Point { row: 8, col: 1 },
                    Point { row: 8, col: 2 },
                    Point { row: 8, col: 3 },
                    Point { row: 8, col: 4 },
                    Point { row: 8, col: 5 },
                    Point { row: 8, col: 6 },
                    Point { row: 8, col: 7 },
                    Point { row: 8, col: 8 },
                    Point { row: 9, col: 0 },
                    Point { row: 9, col: 2 },
                    Point { row: 9, col: 4 },
                    Point { row: 9, col: 5 },
                    Point { row: 9, col: 6 },
                    Point { row: 9, col: 8 },
                ]
                .into_iter()
                .collect()
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 13);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 43);
    }
}
