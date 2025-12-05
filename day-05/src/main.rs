use std::ops::RangeInclusive;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    fresh_ids: Vec<RangeInclusive<u64>>,
    produce: Vec<u64>,
}

fn merge_range(
    a: RangeInclusive<u64>,
    b: RangeInclusive<u64>,
) -> (RangeInclusive<u64>, Option<RangeInclusive<u64>>) {
    match (
        a.contains(b.start()),
        a.contains(b.end()),
        a.end().abs_diff(*b.start()) == 1,
        b.end().abs_diff(*a.start()) == 1,
    ) {
        (true, true, _, _) => (a, None),
        (true, false, _, _) | (_, _, true, _) => (*a.start()..=*b.end(), None),
        (false, true, _, _) | (_, _, _, true) => (*b.start()..=*a.end(), None),
        _ => (a, Some(b)),
    }
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        bytes::complete::tag,
        character::complete::{line_ending, u64},
        combinator::map,
        multi::{count, separated_list1},
        sequence::separated_pair,
    };

    let ids = map(separated_pair(u64, tag("-"), u64), |(s, e)| s..=e);
    let fresh_ids = map(separated_list1(line_ending, ids), |fresh_ids| {
        let len = fresh_ids.len();
        fresh_ids.into_iter().sorted_by_key(|r| *r.start()).fold(
            Vec::with_capacity(len),
            |mut acc, cur| {
                if acc.is_empty() {
                    acc.push(cur);
                } else {
                    let last = acc.last_mut().unwrap();
                    let (merged, maybe) = merge_range(last.clone(), cur);
                    *last = merged;
                    if let Some(last) = maybe {
                        acc.push(last);
                    }
                }
                acc
            },
        )
    });
    let produce = separated_list1(line_ending, u64);
    let ims = separated_pair(fresh_ids, count(line_ending, 2), produce);

    map(ims, |(fresh_ids, produce)| InputData { fresh_ids, produce }).parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .produce
        .iter()
        .filter(|p| input.fresh_ids.iter().any(|ids| ids.contains(p)))
        .count())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input.fresh_ids.iter().cloned().map(Iterator::count).sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                fresh_ids: vec![3..=5, 10..=20],
                produce: vec![1, 5, 8, 11, 17, 32]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 3);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 14);
    }
}
