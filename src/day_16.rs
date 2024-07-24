use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn p1(input: &str) {
    let contraption = parse_contraption(input);
    let mem = run_beam(
        Beam {
            pos: (0, 0),
            dir: Direction::East,
        },
        &contraption,
    );

    println!("The beam energized {} tiles", mem.len())
}

pub fn p2(input: &str) {
    println!("Skipping day 16 part 2");
    return;
    let contraption = parse_contraption(input);
    let mut best = (
        Beam {
            pos: (0, 0),
            dir: Direction::South,
        },
        0,
    );
    for ridx in 0..contraption.rows {
        let start = Beam {
            pos: (ridx, 0),
            dir: Direction::East,
        };
        let mem = run_beam(start.clone(), &contraption);
        if best.1 < mem.len() {
            best = (start, mem.len());
        }

        let start = Beam {
            pos: (ridx, contraption.cols - 1),
            dir: Direction::West,
        };
        let mem = run_beam(start.clone(), &contraption);
        if best.1 < mem.len() {
            best = (start, mem.len());
        }
    }

    for cidx in 0..contraption.cols {
        let start = Beam {
            pos: (0, cidx),
            dir: Direction::South,
        };
        let mem = run_beam(start.clone(), &contraption);
        if best.1 < mem.len() {
            best = (start, mem.len());
        }

        let start = Beam {
            pos: (contraption.rows - 1, cidx),
            dir: Direction::North,
        };
        let mem = run_beam(start.clone(), &contraption);
        if best.1 < mem.len() {
            best = (start, mem.len());
        }
    }

    println!(
        "Best starting beam is {:#?} with {} energized tiles",
        best.0, best.1
    );
}

fn run_beam(start: Beam, contraption: &Contraption) -> HashMap<POS, HashSet<Direction>> {
    let mut mem = HashMap::new();
    let mut beams = vec![start];

    while let Some(beam) = beams.pop() {
        if beam.pos.0 >= contraption.rows || beam.pos.1 >= contraption.cols {
            // Beam is outside the contraption
            continue;
        }

        if !mem
            .entry(beam.pos.clone())
            .or_insert(HashSet::new())
            .insert(beam.dir.clone())
        {
            // Loop
            continue;
        }

        match contraption.mirrors.get(&beam.pos) {
            Some(m) => match (m, beam.dir) {
                (Mirror::M0, Direction::North) => {
                    if beam.pos.1 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 - 1),
                            dir: Direction::West,
                        });
                    }

                    beams.push(Beam {
                        pos: (beam.pos.0, beam.pos.1 + 1),
                        dir: Direction::East,
                    });
                }
                (Mirror::M0, Direction::West) => {
                    if beam.pos.1 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 - 1),
                            dir: Direction::West,
                        });
                    }
                }
                (Mirror::M0, Direction::South) => {
                    if beam.pos.1 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 - 1),
                            dir: Direction::West,
                        });
                    }

                    beams.push(Beam {
                        pos: (beam.pos.0, beam.pos.1 + 1),
                        dir: Direction::East,
                    });
                }
                (Mirror::M0, Direction::East) => {
                    beams.push(Beam {
                        pos: (beam.pos.0, beam.pos.1 + 1),
                        dir: Direction::East,
                    });
                }
                (Mirror::M45, Direction::North) => {
                    beams.push(Beam {
                        pos: (beam.pos.0, beam.pos.1 + 1),
                        dir: Direction::East,
                    });
                }
                (Mirror::M45, Direction::West) => {
                    beams.push(Beam {
                        pos: (beam.pos.0 + 1, beam.pos.1),
                        dir: Direction::South,
                    });
                }
                (Mirror::M45, Direction::South) => {
                    if beam.pos.1 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 - 1),
                            dir: Direction::West,
                        });
                    }
                }
                (Mirror::M45, Direction::East) => {
                    if beam.pos.0 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0 - 1, beam.pos.1),
                            dir: Direction::North,
                        });
                    }
                }
                (Mirror::M90, Direction::North) => {
                    if beam.pos.0 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0 - 1, beam.pos.1),
                            dir: Direction::North,
                        });
                    }
                }
                (Mirror::M90, Direction::West) => {
                    if beam.pos.0 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0 - 1, beam.pos.1),
                            dir: Direction::North,
                        });
                    }

                    beams.push(Beam {
                        pos: (beam.pos.0 + 1, beam.pos.1),
                        dir: Direction::South,
                    });
                }
                (Mirror::M90, Direction::South) => {
                    beams.push(Beam {
                        pos: (beam.pos.0 + 1, beam.pos.1),
                        dir: Direction::South,
                    });
                }
                (Mirror::M90, Direction::East) => {
                    if beam.pos.0 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0 - 1, beam.pos.1),
                            dir: Direction::North,
                        });
                    }

                    beams.push(Beam {
                        pos: (beam.pos.0 + 1, beam.pos.1),
                        dir: Direction::South,
                    });
                }
                (Mirror::M135, Direction::North) => {
                    if beam.pos.1 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 - 1),
                            dir: Direction::West,
                        });
                    }
                }
                (Mirror::M135, Direction::West) => {
                    if beam.pos.0 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0 - 1, beam.pos.1),
                            dir: Direction::North,
                        });
                    }
                }
                (Mirror::M135, Direction::South) => {
                    beams.push(Beam {
                        pos: (beam.pos.0, beam.pos.1 + 1),
                        dir: Direction::East,
                    });
                }
                (Mirror::M135, Direction::East) => beams.push(Beam {
                    pos: (beam.pos.0 + 1, beam.pos.1),
                    dir: Direction::South,
                }),
            },
            None => match beam.dir {
                Direction::North => {
                    if beam.pos.0 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0 - 1, beam.pos.1),
                            dir: Direction::North,
                        });
                    }
                }
                Direction::West => {
                    if beam.pos.1 > 0 {
                        beams.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 - 1),
                            dir: Direction::West,
                        });
                    }
                }
                Direction::South => {
                    beams.push(Beam {
                        pos: (beam.pos.0 + 1, beam.pos.1),
                        dir: Direction::South,
                    });
                }
                Direction::East => {
                    beams.push(Beam {
                        pos: (beam.pos.0, beam.pos.1 + 1),
                        dir: Direction::East,
                    });
                }
            },
        }
    }

    mem
}

fn print_energized(contraption: &Contraption, mem: &HashMap<POS, HashSet<Direction>>) {
    for ridx in 0..contraption.rows {
        for cidx in 0..contraption.cols {
            match mem.get(&(ridx, cidx)) {
                Some(_) => print!("#"),
                None => print!("."),
            };
        }
        print!("\r\n")
    }
    print!("\r\n");
}

type POS = (usize, usize);

enum Mirror {
    M0,
    M45,
    M90,
    M135,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

struct Contraption {
    rows: usize,
    cols: usize,
    mirrors: HashMap<POS, Mirror>,
}

#[derive(Debug, Clone)]
struct Beam {
    pos: POS,
    dir: Direction,
}

fn parse_contraption(input: &str) -> Contraption {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let mirrors = input
        .lines()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(cidx, c)| match c {
                    '.' => None,
                    '|' => Some(((ridx, cidx), Mirror::M90)),
                    '-' => Some(((ridx, cidx), Mirror::M0)),
                    '/' => Some(((ridx, cidx), Mirror::M45)),
                    '\\' => Some(((ridx, cidx), Mirror::M135)),
                    c => panic!("Unknown contraption part {c}"),
                })
        })
        .collect::<HashMap<POS, Mirror>>();

    Contraption {
        rows,
        cols,
        mirrors,
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ridx in 0..self.rows {
            for cidx in 0..self.cols {
                let c = match self.mirrors.get(&(ridx, cidx)) {
                    Some(m) => match m {
                        Mirror::M0 => '-',
                        Mirror::M45 => '/',
                        Mirror::M90 => '|',
                        Mirror::M135 => '\\',
                    },
                    None => '.',
                };
                write!(f, "{c}")?;
            }
            write!(f, "\r\n")?;
        }

        Ok(())
    }
}
