use std::{cmp::Ordering, collections::HashMap, ops::Range};

pub fn p1(input: &str) {
    println!("Skipping day 19 part 1");
    return;

    let workflows = parse_workflows(input);
    let parts = parse_parts(input);

    let mut sum = 0;
    for part in parts.iter() {
        let mut wf_res = PartCmd::GoToWorkflow("in".to_string());
        loop {
            match wf_res {
                PartCmd::Accepted => {
                    sum += part.values().sum::<u32>();
                    break;
                }
                PartCmd::Rejected => {
                    break;
                }
                PartCmd::GoToWorkflow(ref wf_name) => {
                    let wf = workflows.get(wf_name).unwrap();
                    for ((cond_range, cond_part), res) in wf {
                        if !cond_range.contains(part.get(cond_part).unwrap()) {
                            continue;
                        }

                        wf_res = res.clone();
                        break;
                    }
                }
            }
        }
    }

    println!("Result sum {sum}");
}

pub fn p2(input: &str) {
    println!("Skipping day 19 part 2");
    return;
    assert!(Parts::all().count() == 256_000_000_000_000);
    assert!(Parts::empty().count() == 0);

    let workflows = wf2(parse_workflows(input));
    let mut q = vec![(Parts::all(), Cmd2::Wf("in".to_string(), 0))];
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();
    while let Some((parts, cmd)) = q.pop() {
        match cmd {
            Cmd2::Accepted => {
                accepted.push(parts);
            }
            Cmd2::Rejected => {
                rejected.push(parts);
            }
            Cmd2::Wf(wf, step) => {
                let (filter, cmd) = workflows.get(wf.as_str()).unwrap().get(step).unwrap();
                let filtered = filter_parts(&parts, filter);
                assert_eq!(filtered.out.count() + filtered.pass.count(), parts.count());
                if filtered.pass.count() > 0 {
                    q.push((filtered.pass, cmd.clone()));
                }
                if filtered.out.count() > 0 {
                    q.push((filtered.out, Cmd2::Wf(wf, step + 1)));
                }
            }
        }
    }

    println!(
        "Pssible parts accepted {}",
        accepted.iter().map(Parts::count).sum::<usize>()
    );
    println!(
        "Pssible parts rejected {}",
        rejected.iter().map(Parts::count).sum::<usize>()
    );
}

fn parse_workflows(input: &str) -> HashMap<String, Vec<((Range<u32>, Part), PartCmd)>> {
    input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut split = l.split("{");
            let wf_name = split.next().unwrap().to_string();
            let steps = split
                .next()
                .unwrap()
                .trim_end_matches("}")
                .split(",")
                .map(|step| {
                    if step.contains(":") {
                        let condition = step.split(":").next().unwrap();
                        let part = condition.chars().next().unwrap();
                        let part = Part::try_from(part.to_string().as_str()).unwrap();
                        let number =
                            u32::from_str_radix(&condition.chars().skip(2).collect::<String>(), 10)
                                .unwrap();
                        let range = match condition.chars().skip(1).next().unwrap() {
                            '>' => number..4001,
                            '<' => 1..(number + 1),
                            s => panic!("Invalid conditinal operator {s}"),
                        };

                        let wf_result = match step.split(":").skip(1).next().unwrap() {
                            "A" => PartCmd::Accepted,
                            "R" => PartCmd::Rejected,
                            s => PartCmd::GoToWorkflow(s.to_string()),
                        };
                        ((range, part), wf_result)
                    } else {
                        (
                            (u32::MIN..u32::MAX, Part::A),
                            match step {
                                "A" => PartCmd::Accepted,
                                "R" => PartCmd::Rejected,
                                c => PartCmd::GoToWorkflow(c.to_string()),
                            },
                        )
                    }
                })
                .collect::<Vec<_>>();
            (wf_name, steps)
        })
        .collect::<HashMap<_, _>>()
}

fn parse_parts(input: &str) -> Vec<HashMap<Part, u32>> {
    let wfs = input.lines().take_while(|l| !l.is_empty()).count();
    input
        .lines()
        .skip(wfs + 1)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.trim_start_matches("{")
                .trim_end_matches("}")
                .split(",")
                .map(|pp| {
                    (
                        Part::try_from(pp.split("=").next().unwrap()).unwrap(),
                        u32::from_str_radix(pp.split("=").skip(1).next().unwrap(), 10).unwrap(),
                    )
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Part {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Part {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Part::X),
            "m" => Ok(Part::M),
            "a" => Ok(Part::A),
            "s" => Ok(Part::S),
            x => Err(format!("Invalid part {x}").to_string()),
        }
    }
}

#[derive(Debug, Clone)]
enum PartCmd {
    Accepted,
    Rejected,
    GoToWorkflow(String),
}

#[derive(Debug, Clone)]
enum Cmd2 {
    Accepted,
    Rejected,
    Wf(String, usize),
}

#[derive(Clone, Debug)]
struct Parts {
    x: [Option<()>; 4000],
    m: [Option<()>; 4000],
    a: [Option<()>; 4000],
    s: [Option<()>; 4000],
}

impl Parts {
    const fn all() -> Self {
        Parts {
            x: [Some(()); 4000],
            m: [Some(()); 4000],
            a: [Some(()); 4000],
            s: [Some(()); 4000],
        }
    }

    const fn empty() -> Self {
        Parts {
            x: [None; 4000],
            m: [None; 4000],
            a: [None; 4000],
            s: [None; 4000],
        }
    }

    fn count(&self) -> usize {
        self.x.iter().filter(|x| x.is_some()).count()
            * self.m.iter().filter(|x| x.is_some()).count()
            * self.a.iter().filter(|x| x.is_some()).count()
            * self.s.iter().filter(|x| x.is_some()).count()
    }

    fn get_mut(&mut self, p: Part) -> &mut [Option<()>; 4000] {
        match p {
            Part::X => &mut self.x,
            Part::M => &mut self.m,
            Part::A => &mut self.a,
            Part::S => &mut self.s,
        }
    }
}

#[derive(Clone, Debug)]
struct PartsFilter {
    part: Part,
    point: u32,
    op: Ordering,
}

impl PartsFilter {
    fn idx_is_good(&self, idx: usize) -> bool {
        match self.op {
            Ordering::Less => idx + 1 < self.point as usize,
            Ordering::Equal => idx + 1 == self.point as usize,
            Ordering::Greater => idx + 1 > self.point as usize,
        }
    }
}

fn filter_parts(parts: &Parts, filter: &PartsFilter) -> FilteredParts {
    let mut pass = parts.clone();
    pass.get_mut(filter.part)
        .iter_mut()
        .enumerate()
        .for_each(|(idx, opt)| {
            if !filter.idx_is_good(idx) {
                opt.take();
            }
        });

    let mut out = parts.clone();
    out.get_mut(filter.part)
        .iter_mut()
        .enumerate()
        .for_each(|(idx, opt)| {
            if filter.idx_is_good(idx) {
                opt.take();
            }
        });

    assert_eq!(pass.count() + out.count(), parts.count());

    FilteredParts { pass, out }
}

#[derive(Clone, Debug)]
struct FilteredParts {
    pass: Parts,
    out: Parts,
}

fn wf2(
    wf: HashMap<String, Vec<((Range<u32>, Part), PartCmd)>>,
) -> HashMap<String, Vec<(PartsFilter, Cmd2)>> {
    wf.into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|((r, p), cmd)| {
                        (
                            if r.start == 1 {
                                PartsFilter {
                                    part: p,
                                    point: r.len() as u32,
                                    op: Ordering::Less,
                                }
                            } else {
                                PartsFilter {
                                    part: p,
                                    point: r.start,
                                    op: Ordering::Greater,
                                }
                            },
                            match cmd {
                                PartCmd::Accepted => Cmd2::Accepted,
                                PartCmd::Rejected => Cmd2::Rejected,
                                PartCmd::GoToWorkflow(wf) => Cmd2::Wf(wf, 0),
                            },
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}
