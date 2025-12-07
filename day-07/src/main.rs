use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {
    start: usize,
    rows: Vec<Vec<bool>>,
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    let mut lines = input.lines();
    let start = lines
        .next()
        .unwrap()
        .bytes()
        .enumerate()
        .find_map(|(i, c)| if c == b'S' { Some(i) } else { None })
        .unwrap();
    let rows = lines
        .map(|line| line.bytes().map(|c| c == b'^').collect())
        .collect_vec();
    Ok(("", InputData { start, rows }))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let init = (0..input.rows.len())
        .map(|i| i == input.start)
        .collect_vec();
    Ok(input
        .rows
        .iter()
        .scan(init, |state, cur| {
            let mut count = 0;
            for (i, c) in cur.iter().enumerate() {
                if *c && state[i] {
                    count += 1;
                    state[i] = false;
                    state[i - 1] = true;
                    state[i + 1] = true;
                }
            }
            Some(count)
        })
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let init = (0..input.rows.len())
        .map(|i| usize::from(i == input.start))
        .collect_vec();
    Ok(input
        .rows
        .iter()
        .fold(init, |mut state, cur| {
            for (i, c) in cur.iter().enumerate() {
                if *c {
                    state[i - 1] += state[i];
                    state[i + 1] += state[i];
                    state[i] = 0;
                }
            }
            state
        })
        .into_iter()
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
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, true, false, false, false,
                        false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, true, false, true, false, false,
                        false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, true, false, true, false, true, false,
                        false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, false, false, false, true, false, true, false, false, false, true,
                        false, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, false, false, true, false, true, false, false, false, true, false,
                        true, false, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, false, true, false, false, false, true, false, false, false, false,
                        false, true, false, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ],
                    vec![
                        false, true, false, true, false, true, false, true, false, true, false,
                        false, false, true, false
                    ],
                    vec![
                        false, false, false, false, false, false, false, false, false, false,
                        false, false, false, false, false
                    ]
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
