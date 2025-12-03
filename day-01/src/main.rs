#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Right(i64),
    Left(i64),
}

#[derive(Debug, PartialEq)]
struct InputData {
    instructions: Vec<Operation>,
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{i64, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::preceded,
    };

    let left = map(preceded(tag("L"), i64), Operation::Left);
    let right = map(preceded(tag("R"), i64), Operation::Right);
    let either = alt((left, right));
    let list = separated_list1(line_ending, either);
    map(list, |instructions| InputData { instructions }).parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .instructions
        .iter()
        .fold((50, 0), |(dial, zeros), turn| {
            let cur = match turn {
                Operation::Right(i) => dial - i,
                Operation::Left(i) => dial + i,
            }
            .rem_euclid(100);
            (cur, zeros + if cur == 0 { 1 } else { 0 })
        })
        .1)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<i64> {
    Ok(input
        .instructions
        .iter()
        .fold((50, 0), |(dial, zeros), turn| match turn {
            Operation::Left(i) => {
                let pass_zero = i.div_euclid(100);
                let i = i.rem_euclid(100);
                let new_dial = dial - i;
                let pass_zero = pass_zero + if new_dial <= 0 && dial != 0 { 1 } else { 0 };
                (new_dial.rem_euclid(100), zeros + pass_zero)
            }
            Operation::Right(i) => {
                let pass_zero = i.div_euclid(100);
                let i = i.rem_euclid(100);
                let new_dial = dial + i;
                let pass_zero = pass_zero + if new_dial >= 100 && dial != 0 { 1 } else { 0 };
                (new_dial.rem_euclid(100), zeros + pass_zero)
            }
        })
        .1)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_parser() {
        use Operation::*;

        assert_parser!(
            parse,
            INPUT,
            InputData {
                instructions: vec![
                    Left(68),
                    Left(30),
                    Right(48),
                    Left(5),
                    Right(60),
                    Left(55),
                    Left(1),
                    Left(99),
                    Right(14),
                    Left(82),
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 3);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 6);
    }
}
