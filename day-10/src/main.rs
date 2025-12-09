use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser
    };

    todo!()
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<()> {
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<()> {
    Ok(())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_parser() {
        assert_parser!(parse, INPUT, InputData {});
    }

    #[test]
    fn test_part1() {
        // assert_part!(parse, part1, INPUT, ());
    }

    #[test]
    fn test_part2() {
        // assert_part!(parse, part2, INPUT, ());
    }
}
