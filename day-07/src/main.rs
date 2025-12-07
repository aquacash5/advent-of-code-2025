use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    start: usize,
    rows: Vec<BTreeSet<usize>>,
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    let mut lines = input.lines();
    let start = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
        .find_map(|(i, &c)| if c == b'S' { Some(i) } else { None })
        .unwrap();
    let rows = lines
        .map(|line| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(i, &c)| if c == b'^' { Some(i) } else { None })
                .collect()
        })
        .collect_vec();
    Ok(("", InputData { start, rows }))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input
        .rows
        .iter()
        .scan(BTreeSet::from([input.start]), |state, cur| {
            let mut count = 0;
            for i in state.clone() {
                if cur.contains(&i) {
                    count += 1;
                    state.remove(&i);
                    state.insert(i - 1);
                    state.insert(i + 1);
                }
            }
            Some(count)
        })
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .rows
        .iter()
        .fold(BTreeMap::from([(input.start, 1usize)]), |mut state, cur| {
            let keys = state.keys().copied().collect_vec();
            for i in keys {
                if cur.contains(&i) {
                    let v = state.remove(&i).unwrap();
                    state.entry(i - 1).and_modify(|c| *c += v).or_insert(v);
                    state.entry(i + 1).and_modify(|c| *c += v).or_insert(v);
                }
            }
            state
        })
        .values()
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                start: 7,
                rows: vec![
                    BTreeSet::from([]),
                    BTreeSet::from([7]),
                    BTreeSet::from([]),
                    BTreeSet::from([6, 8]),
                    BTreeSet::from([]),
                    BTreeSet::from([5, 7, 9]),
                    BTreeSet::from([]),
                    BTreeSet::from([4, 6, 10]),
                    BTreeSet::from([]),
                    BTreeSet::from([3, 5, 9, 11]),
                    BTreeSet::from([]),
                    BTreeSet::from([2, 6, 12]),
                    BTreeSet::from([]),
                    BTreeSet::from([1, 3, 5, 7, 9, 13]),
                    BTreeSet::from([]),
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 21);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 40);
    }
}
