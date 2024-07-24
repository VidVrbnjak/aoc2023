use std::{collections::HashMap, fmt::Display, usize::MAX};

const LOGGING: bool = false;

pub fn p1(input: &str) {
    let lines = parse_lines(input);
    let mut mem = HashMap::new();
    let arragments_sum = lines
        .iter()
        .map(|l| solve_from_start(l.clone(), &mut mem))
        .sum::<usize>();

    println!("Total possible arragments: {arragments_sum}");
}

pub fn p2(input: &str) {
    if true {
        println!("SKIPPING DAY 12 Part 2");
        return;
    }
    let lines = parse_lines_p2(input);
    let mut mem = HashMap::new();
    let arragments_sum = lines
        .iter()
        .map(|l| solve_from_start(l.clone(), &mut mem))
        .sum::<usize>();

    println!("Total unfolded possible arragments: {arragments_sum}");
}

fn solve_from_start(l: Line, mem: &mut HashMap<Line, usize>) -> usize {
    if let Some(solution) = mem.get(&l) {
        // We already know the solution
        return *solution;
    }

    if l.d.len() == 0 && l.constraints.len() == 0 {
        return 1;
    }

    if l.d.len() == 0 && l.constraints.len() > 0 {
        return 0;
    }

    let first = l.d[0];
    let solutions = match first {
        Spring::Operational => solve_from_start(
            Line {
                d: l.d[1..].to_vec(),
                constraints: l.constraints.clone(),
            },
            mem,
        ),
        Spring::Damaged => {
            let constraint = l.constraints.first();
            if constraint.is_none() {
                if LOGGING {
                    println!("{l} is unsolvable, because it oes not have any constraints left");
                }
                mem.insert(l.clone(), 0);
                return 0;
            }
            let constraint = *constraint.unwrap();

            let max_len =
                l.d.iter()
                    .enumerate()
                    .find_map(|(idx, s)| {
                        if *s == Spring::Operational {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(l.d.len());

            if max_len < constraint {
                if LOGGING {
                    println!("{l} is unsolvable, because it cannot fit the constraint");
                }
                mem.insert(l.clone(), 0);
                return 0;
            }

            if l.constraints.len() > 1 {
                if l.d.get(constraint).is_some_and(|s| *s == Spring::Damaged) {
                    return 0;
                } else {
                    solve_from_start(
                        Line {
                            d: l.d[(constraint + 1).min(l.d.len())..].to_vec(),
                            constraints: l.constraints[1..].to_vec(),
                        },
                        mem,
                    )
                }
            } else {
                solve_from_start(
                    Line {
                        d: l.d[constraint..].to_vec(),
                        constraints: l.constraints[1..].to_vec(),
                    },
                    mem,
                )
            }
        }
        Spring::Unknown => {
            let mut damaged = l.clone();
            damaged.d[0] = Spring::Damaged;

            let mut operational = l.clone();
            operational.d[0] = Spring::Operational;

            solve_from_start(damaged, mem) + solve_from_start(operational, mem)
        }
    };

    mem.insert(l.clone(), solutions);
    solutions
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Line {
    d: Vec<Spring>,
    constraints: Vec<usize>,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.d.iter() {
            write!(f, "{s}")?;
        }

        for c in self.constraints.iter() {
            write!(f, " {c} ")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Spring::Operational => ".",
                Spring::Damaged => "#",
                Spring::Unknown => "?",
            }
        )
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let d = l
                .split(" ")
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Spring::Damaged,
                    '.' => Spring::Operational,
                    '?' => Spring::Unknown,
                    c => panic!("Unknown symbol  {c}"),
                })
                .collect::<Vec<_>>();

            let constraints = l
                .split(" ")
                .skip(1)
                .next()
                .unwrap()
                .split(",")
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>();

            Line { d, constraints }
        })
        .collect::<Vec<_>>()
}

fn parse_lines_p2(input: &str) -> Vec<Line> {
    let mut lines = parse_lines(input);
    lines.iter_mut().for_each(|l| {
        let d = l.d.clone();
        l.d.push(Spring::Unknown);
        l.d.append(&mut d.clone());
        l.d.push(Spring::Unknown);
        l.d.append(&mut d.clone());
        l.d.push(Spring::Unknown);
        l.d.append(&mut d.clone());
        l.d.push(Spring::Unknown);
        l.d.append(&mut d.clone());
        let c = l.constraints.clone();
        l.constraints.append(&mut c.clone());
        l.constraints.append(&mut c.clone());
        l.constraints.append(&mut c.clone());
        l.constraints.append(&mut c.clone());
    });

    lines
}
