use itertools::{Itertools, chain};
use ndarray::Array2;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
}

impl Point {
    fn neighbors(&self) -> Vec<Self> {
        chain!(
            self.up().and_then(Self::left),
            self.up(),
            self.up().and_then(Self::right),
            self.left(),
            self.right(),
            self.down().and_then(Self::left),
            self.down(),
            self.down().and_then(Self::right)
        )
        .collect_vec()
    }

    fn up(self) -> Option<Self> {
        let Self {
            col,
            max_row,
            max_col,
            ..
        } = self;
        self.row.checked_sub(1).map(|row| Self {
            row,
            col,
            max_row,
            max_col,
        })
    }

    fn down(self) -> Option<Self> {
        let Self {
            col,
            max_row,
            max_col,
            ..
        } = self;
        self.row
            .checked_add(1)
            .filter(|&row| row < max_row)
            .map(|row| Self {
                row,
                col,
                max_row,
                max_col,
            })
    }

    fn left(self) -> Option<Self> {
        let Self {
            row,
            max_row,
            max_col,
            ..
        } = self;
        self.col.checked_sub(1).map(|col| Self {
            row,
            col,
            max_row,
            max_col,
        })
    }

    fn right(self) -> Option<Self> {
        let Self {
            row,
            max_row,
            max_col,
            ..
        } = self;
        self.col
            .checked_add(1)
            .filter(|&col| col < max_col)
            .map(|col| Self {
                row,
                col,
                max_row,
                max_col,
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Storage {
    Empty,
    Roll,
}

impl Storage {
    const fn take(&mut self) {
        *self = Self::Empty;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct InputData {
    warehouse: Array2<Storage>,
}

impl InputData {
    fn neighbors_count(&self, (row, col): (usize, usize)) -> usize {
        let location = Point {
            row,
            col,
            max_row: self.warehouse.nrows(),
            max_col: self.warehouse.ncols(),
        };
        location
            .neighbors()
            .into_iter()
            .filter(|loc| self.warehouse[[loc.row, loc.col]] == Storage::Roll)
            .count()
    }
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::map,
        multi::{many1, separated_list1},
    };

    let empty = map(tag("."), |_| Storage::Empty);
    let roll = map(tag("@"), |_| Storage::Roll);
    let location = alt((empty, roll));
    let row = many1(location);
    let warehouse = separated_list1(line_ending, row);

    map(warehouse, |rows| {
        let row_len = rows.len();
        let col_len = rows[0].len();

        InputData {
            warehouse: Array2::from_shape_fn((row_len, col_len), |(r, c)| rows[r][c]),
        }
    })
    .parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .warehouse
        .indexed_iter()
        .filter(|&(loc, &s)| s == Storage::Roll && input.neighbors_count(loc) < 4)
        .count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let mut count = 0usize;
    let mut changed = true;
    let mut input: InputData = input.clone();

    while changed {
        let removals = input
            .warehouse
            .indexed_iter()
            .filter(|&(loc, &s)| s == Storage::Roll && input.neighbors_count(loc) < 4)
            .map(|(loc, _)| loc)
            .collect_vec();
        for &loc in &removals {
            input.warehouse[loc].take();
        }
        changed = !removals.is_empty();
        count += removals.len();
    }

    Ok(count)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use ndarray::array;

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
        use Storage::*;

        assert_parser!(
            parse,
            INPUT,
            InputData {
                warehouse: array![
                    [
                        Empty, Empty, Roll, Roll, Empty, Roll, Roll, Roll, Roll, Empty
                    ],
                    [
                        Roll, Roll, Roll, Empty, Roll, Empty, Roll, Empty, Roll, Roll
                    ],
                    [Roll, Roll, Roll, Roll, Roll, Empty, Roll, Empty, Roll, Roll],
                    [
                        Roll, Empty, Roll, Roll, Roll, Roll, Empty, Empty, Roll, Empty
                    ],
                    [Roll, Roll, Empty, Roll, Roll, Roll, Roll, Empty, Roll, Roll],
                    [Empty, Roll, Roll, Roll, Roll, Roll, Roll, Roll, Empty, Roll],
                    [
                        Empty, Roll, Empty, Roll, Empty, Roll, Empty, Roll, Roll, Roll
                    ],
                    [Roll, Empty, Roll, Roll, Roll, Empty, Roll, Roll, Roll, Roll],
                    [Empty, Roll, Roll, Roll, Roll, Roll, Roll, Roll, Roll, Empty],
                    [
                        Roll, Empty, Roll, Empty, Roll, Roll, Roll, Empty, Roll, Empty
                    ]
                ]
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
