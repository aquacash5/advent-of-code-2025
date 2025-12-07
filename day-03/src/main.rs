use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    batteries: Vec<String>,
}

fn find_largest(s: &str, i: usize) -> u64 {
    (1..=i)
        .scan(s, |rem, r| {
            let end = rem.len() - (i - r);
            let (i, biggest) = rem[..end]
                .chars()
                .enumerate()
                .reduce(|acc, cur| if cur.1 > acc.1 { cur } else { acc })
                .map(|(i, c)| (i + 1, c))
                .expect("No max value");
            *rem = &rem[i..];
            Some(biggest)
        })
        .collect::<String>()
        .parse()
        .expect("All numbers")
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
