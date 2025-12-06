use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn operate<'a>(&self, numbers: impl Iterator<Item = &'a u64>) -> u64 {
        match self {
            Operation::Add => numbers.sum(),
            Operation::Multiply => numbers.product(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    groups: Vec<Group>,
}

#[derive(Debug, PartialEq)]
struct Group {
    numbers: Vec<u64>,
    operation: Operation,
}

fn parse1(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, space0, space1, u64},
        combinator::map,
        multi::separated_list1,
        sequence::{delimited, separated_pair},
    };
    let add = map(tag("+"), |_| Operation::Add);
    let multiply = map(tag("*"), |_| Operation::Multiply);
    let operation = alt((add, multiply));
    let operation_row = delimited(space0, separated_list1(space1, operation), space0);

    let number_row = delimited(space0, separated_list1(space1, u64), space0);
    let number_rows = separated_list1(line_ending, number_row);

    let rows = separated_pair(number_rows, line_ending, operation_row);

    map(rows, |(grid, operations)| InputData {
        groups: operations
            .into_iter()
            .enumerate()
            .map(|(i, operation)| Group {
                numbers: grid.iter().map(|r| r[i]).collect_vec(),
                operation,
            })
            .collect_vec(),
    })
    .parse(input)
}

fn parse2(input: &str) -> ParseResult<'_, InputData> {
    // I hate this!
    let mut data: Vec<String> = vec![];
    for line in input.lines() {
        if data.is_empty() {
            data = line.chars().rev().map(|c| c.to_string()).collect_vec();
        } else {
            data.iter_mut()
                .zip(line.chars().rev())
                .for_each(|(d, c)| d.push(c));
        }
    }
    let input = data.join("\n");

    fn inner(input: &str) -> ParseResult<'_, InputData> {
        use nom::{
            Parser,
            branch::alt,
            bytes::complete::tag,
            character::complete::{line_ending, space0, space1, u64},
            combinator::map,
            multi::separated_list1,
            sequence::{delimited, pair},
        };

        let add = map(tag("+"), |_| Operation::Add);
        let multiply = map(tag("*"), |_| Operation::Multiply);
        let operation = alt((add, multiply));

        let empty_row = delimited(line_ending, space1, line_ending);

        let number_row = delimited(space0, u64, space0);
        let number_rows = separated_list1(line_ending, number_row);

        let group = map(pair(number_rows, operation), |(numbers, operation)| Group {
            numbers,
            operation,
        });

        map(separated_list1(empty_row, group), |groups| InputData {
            groups,
        })
        .parse(input)
    }
    let (_, data) = inner(&input).unwrap();
    Ok(("", data))
}

#[allow(clippy::unnecessary_wraps)]
fn operate(input: &InputData) -> AocResult<u64> {
    Ok(input
        .groups
        .iter()
        .map(|Group { numbers, operation }| operation.operate(numbers.iter()))
        .sum())
}

aoc_main!(parse1, parse2, operate, operate);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_parser1() {
        assert_parser!(
            parse1,
            INPUT,
            InputData {
                groups: vec![
                    Group {
                        numbers: vec![123, 45, 6],
                        operation: Operation::Multiply
                    },
                    Group {
                        numbers: vec![328, 64, 98],
                        operation: Operation::Add
                    },
                    Group {
                        numbers: vec![51, 387, 215],
                        operation: Operation::Multiply
                    },
                    Group {
                        numbers: vec![64, 23, 314],
                        operation: Operation::Add
                    },
                ]
            }
        );
    }

    #[test]
    fn test_parser2() {
        assert_parser!(
            parse2,
            INPUT,
            InputData {
                groups: vec![
                    Group {
                        numbers: vec![4, 431, 623],
                        operation: Operation::Add
                    },
                    Group {
                        numbers: vec![175, 581, 32],
                        operation: Operation::Multiply
                    },
                    Group {
                        numbers: vec![8, 248, 369],
                        operation: Operation::Add
                    },
                    Group {
                        numbers: vec![356, 24, 1],
                        operation: Operation::Multiply
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse1, operate, INPUT, 4277556);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse2, operate, INPUT, 3263827);
    }
}
