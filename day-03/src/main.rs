use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    batteries: Vec<String>,
}

fn find_largest(s: &str, i: usize) -> u64 {
    let mut acc = String::default();
    let mut start = 0usize;
    for r in 1..=i {
        let ignore = i - r;
        let end = s.len() - 1;
        let &(i, biggest) = s[start..=(end - ignore)]
            .chars()
            .enumerate()
            .max_set_by_key(|&(_, i)| i)
            .first()
            .expect("Must be empty");
        acc.push(biggest);
        start += i + 1;
    }
    acc.parse().expect("All numbers")
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        character::complete::{digit1, line_ending},
        combinator::map,
        multi::separated_list1,
    };

    map(separated_list1(line_ending, digit1), |batteries| {
        InputData {
            batteries: batteries.into_iter().map(str::to_string).collect_vec(),
        }
    })
    .parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(input
        .batteries
        .iter()
        .map(|battery| find_largest(battery, 2))
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    Ok(input
        .batteries
        .iter()
        .map(|battery| find_largest(battery, 12))
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                batteries: vec![
                    "987654321111111".to_string(),
                    "811111111111119".to_string(),
                    "234234234234278".to_string(),
                    "818181911112111".to_string(),
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 357);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 3121910778619u64);
    }
}
