use std::ops::RangeInclusive;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    ranges: Vec<RangeInclusive<u64>>,
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser, bytes::complete::tag, character::complete::u64, combinator::map,
        multi::separated_list1, sequence::separated_pair,
    };

    let range = map(separated_pair(u64, tag("-"), u64), |(s, e)| s..=e);
    let ranges = separated_list1(tag(","), range);
    map(ranges, |ranges| InputData { ranges }).parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(input
        .ranges
        .iter()
        .flat_map(|r| {
            r.clone().filter(|i| {
                let s = i.to_string();
                let (s, e) = s.split_at(0usize.midpoint(s.len()));
                s == e
            })
        })
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    Ok(input
        .ranges
        .iter()
        .flat_map(|r| {
            r.clone().filter(|i| {
                let s = i.to_string();
                (1..s.len()).any(|pos| s.as_bytes().chunks(pos).all_equal())
            })
        })
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,\
    222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
    565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                ranges: vec![
                    11..=22,
                    95..=115,
                    998..=1012,
                    1188511880..=1188511890,
                    222220..=222224,
                    1698522..=1698528,
                    446443..=446449,
                    38593856..=38593862,
                    565653..=565659,
                    824824821..=824824827,
                    2121212118..=2121212124
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 4174379265u64);
    }
}
