use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Light {
    On,
    Off,
}

impl Light {
    fn toggle(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LightPanel {
    lights: Vec<Light>,
}

impl LightPanel {
    fn off(&self) -> Self {
        let mut lights = Vec::with_capacity(self.lights.len());
        for _ in 0..self.lights.len() {
            lights.push(Light::Off);
        }

        LightPanel { lights }
    }

    fn toggle(&self, buttons: &[usize]) -> Self {
        let mut new_indicator = self.clone();
        for &button in buttons {
            new_indicator.lights[button].toggle();
        }
        new_indicator
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BatteryBank {
    batteries: Vec<isize>,
}

impl BatteryBank {
    fn toggle(&self, buttons: &[usize]) -> Self {
        let mut new_bank = self.clone();
        for &button in buttons {
            new_bank.batteries[button] -= 1;
        }
        new_bank
    }

    fn empty(&self) -> BatteryBank {
        BatteryBank {
            batteries: (0..self.batteries.len()).map(|_| 0).collect_vec(),
        }
    }

    fn is_depleted(&self) -> bool {
        self.batteries.iter().any(|b| *b < 0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: LightPanel,
    buttons: Vec<Vec<usize>>,
    batteries: BatteryBank,
}

impl Machine {
    fn start_sequence(&self) -> usize {
        let mut set: BTreeSet<LightPanel> = BTreeSet::new();
        set.insert(self.lights.off());

        let mut count = 0usize;

        loop {
            count += 1;
            set = set
                .into_iter()
                .flat_map(|bank| {
                    self.buttons
                        .iter()
                        .map(|button| bank.toggle(button.as_slice()))
                        .collect_vec()
                })
                .collect();
            if set.contains(&self.lights) {
                return count;
            }
        }
        // let mut map: BTreeMap<LightPanel, Vec<&[usize]>> = BTreeMap::new();
        // map.insert(self.lights.off(), vec![]);

        // loop {
        //     let mut tries: Vec<(LightPanel, Vec<&[usize]>)> =
        //         Vec::with_capacity(self.buttons.len() * map.len());
        //     for button in self.buttons.iter().map(|b| b.as_slice()) {
        //         for (panel, presses) in map.iter() {
        //             let mut presses = presses.clone();
        //             presses.push(button);
        //             let panel = panel.toggle(button);
        //             if panel == self.lights {
        //                 return presses;
        //             }
        //             tries.push((panel, presses));
        //         }
        //     }
        //     for (panel, presses) in tries {
        //         map.entry(panel).or_insert(presses);
        //     }
        // }
    }

    fn set_joltage(&self) -> usize {
        let mut set: HashSet<BatteryBank> = HashSet::new();
        set.insert(self.batteries.clone());

        let empty = self.batteries.empty();

        let mut count = 0usize;

        loop {
            count += 1;
            dbg!(count);
            set.extend(
                set.par_iter()
                    .flat_map(|bank| {
                        self.buttons
                            .iter()
                            .map(|button| bank.toggle(button.as_slice()))
                            .filter(|bank| !bank.is_depleted() && !set.contains(bank))
                            .collect_vec()
                    })
                    .fold(HashSet::new, |mut acc, cur| {
                        acc.insert(cur);
                        acc
                    })
                    .reduce(HashSet::new, |mut acc, cur| {
                        acc.extend(cur);
                        acc
                    }),
            );
            if set.contains(&empty) {
                return count;
            }
        }
        // dbg!(self);
        // fn inner<'a>(bank: BatteryBank, buttons: &Vec<&'a [usize]>) -> Option<Vec<&'a [usize]>> {
        //     if bank.is_depleted() {
        //         None
        //     } else if bank.is_empty() {
        //         Some(vec![])
        //     } else {
        //         for button in buttons {
        //             if let Some(mut presses) = inner(bank.toggle(button), buttons) {
        //                 presses.push(button);
        //                 return Some(presses);
        //             }
        //         }
        //         None
        //     }
        // }

        // inner(
        //     self.batteries.clone(),
        //     &self
        //         .buttons
        //         .iter()
        //         .map(Vec::as_slice)
        //         .sorted_by_key(|v| v.len())
        //         .rev()
        //         .collect_vec(),
        // )
        // .unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    machines: Vec<Machine>,
}

fn parse(input: &str) -> ParseResult<'_, InputData> {
    use nom::{
        Parser,
        branch::alt,
        bytes::complete::tag,
        character::complete::{isize, line_ending, space1, usize},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::delimited,
    };

    let light = alt((map(tag("."), |_| Light::Off), map(tag("#"), |_| Light::On)));
    let lights = map(delimited(tag("["), many1(light), tag("]")), |lights| {
        LightPanel { lights }
    });

    let button = delimited(tag("("), separated_list1(tag(","), usize), tag(")"));
    let buttons = separated_list1(space1, button);

    let batteries = map(
        delimited(tag("{"), separated_list1(tag(","), isize), tag("}")),
        |batteries| BatteryBank { batteries },
    );

    let machine = map(
        (lights, space1, buttons, space1, batteries),
        |(lights, _, buttons, _, batteries)| Machine {
            lights,
            buttons,
            batteries,
        },
    );
    map(separated_list1(line_ending, machine), |machines| {
        InputData { machines }
    })
    .parse(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input.machines.iter().map(Machine::start_sequence).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .machines
        .iter()
        .enumerate()
        .map(|(i, m)| {
            println!("{i}");
            m
        })
        .map(Machine::set_joltage)
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                machines: vec![
                    Machine {
                        lights: LightPanel {
                            lights: vec![Light::Off, Light::On, Light::On, Light::Off]
                        },
                        buttons: vec![
                            vec![3],
                            vec![1, 3],
                            vec![2],
                            vec![2, 3],
                            vec![0, 2],
                            vec![0, 1]
                        ],
                        batteries: BatteryBank {
                            batteries: vec![3, 5, 4, 7]
                        }
                    },
                    Machine {
                        lights: LightPanel {
                            lights: vec![Light::Off, Light::Off, Light::Off, Light::On, Light::Off]
                        },
                        buttons: vec![
                            vec![0, 2, 3, 4],
                            vec![2, 3],
                            vec![0, 4],
                            vec![0, 1, 2],
                            vec![1, 2, 3, 4]
                        ],
                        batteries: BatteryBank {
                            batteries: vec![7, 5, 12, 7, 2]
                        }
                    },
                    Machine {
                        lights: LightPanel {
                            lights: vec![
                                Light::Off,
                                Light::On,
                                Light::On,
                                Light::On,
                                Light::Off,
                                Light::On
                            ]
                        },
                        buttons: vec![
                            vec![0, 1, 2, 3, 4],
                            vec![0, 3, 4],
                            vec![0, 1, 2, 4, 5],
                            vec![1, 2]
                        ],
                        batteries: BatteryBank {
                            batteries: vec![10, 11, 11, 5, 10, 5]
                        }
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 7);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 33);
    }
}
