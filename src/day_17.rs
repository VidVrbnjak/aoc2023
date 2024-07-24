use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    str::FromStr,
};

pub fn p1(input: &str) {
    println!("Skipping day 17 part 1");
    return;
    let map = Map::from_str(input).unwrap();
    let steps = find_shortest_path(&map, (0, 0), (map.rows - 1, map.cols - 1), 3, 1);
    println!(
        "Best case senario we take {} heat damage",
        steps
            .iter()
            .map(|position| map.cost(&position.pos).unwrap())
            .sum::<usize>()
    );
}
pub fn p2(input: &str) {
    println!("Skipping day 17 part 2");
    return;
    let map = Map::from_str(input).unwrap();
    let steps = find_shortest_path(&map, (0, 0), (map.rows - 1, map.cols - 1), 10, 4);
    println!(
        "Best case senario we take {} heat damage",
        steps
            .iter()
            .map(|position| map.cost(&position.pos).unwrap())
            .sum::<usize>()
    );
}

fn find_shortest_path(
    map: &Map,
    start: (usize, usize),
    end: (usize, usize),
    sd_max_steps: usize,
    sd_min_steps: usize,
) -> Vec<Position> {
    let mut frontier = BinaryHeap::new();
    let mut steps_to_reach: HashMap<Position, Option<Position>> = HashMap::new();
    steps_to_reach.insert(
        Position {
            pos: start,
            dir: Direction::South,
            sd_steps: 0,
        },
        None,
    );
    steps_to_reach.insert(
        Position {
            pos: start,
            dir: Direction::East,
            sd_steps: 0,
        },
        None,
    );

    frontier.push(FrontierPos {
        pos: Position {
            pos: start,
            dir: Direction::South,
            sd_steps: 0,
        },
        prio: Reverse(0),
    });
    frontier.push(FrontierPos {
        pos: Position {
            pos: start,
            dir: Direction::East,
            sd_steps: 0,
        },
        prio: Reverse(0),
    });

    let mut costs_to_reach = HashMap::new();
    costs_to_reach.insert(
        Position {
            pos: start,
            dir: Direction::South,
            sd_steps: 0,
        },
        0,
    );
    costs_to_reach.insert(
        Position {
            pos: start,
            dir: Direction::East,
            sd_steps: 0,
        },
        0,
    );

    while let Some(ctx) = frontier.pop() {
        let mut ctx = ctx.pos;
        if ctx.pos == end {
            let mut steps = Vec::new();
            while let Some(prev_ctx) = &steps_to_reach[&ctx] {
                steps.push(ctx.clone());
                ctx = prev_ctx.clone()
            }

            let dir = steps.first().unwrap().dir;
            if steps.iter().take_while(|step| step.dir == dir).count() < sd_min_steps {
                continue;
            }

            return steps;
        }

        for nctx in ctx.neighbours() {
            if nctx.dir.is_opposite(ctx.dir) {
                continue;
            }

            if nctx.sd_steps > sd_max_steps {
                continue;
            }

            if nctx.dir != ctx.dir && ctx.sd_steps < sd_min_steps {
                continue;
            }

            if !map.contains(&nctx.pos) {
                continue;
            }

            let ncost = costs_to_reach[&ctx] + map.cost(&nctx.pos).unwrap_or_default();
            if costs_to_reach.get(&nctx).is_some_and(|c| *c <= ncost) {
                continue;
            }

            let iaheu = heuristic(&nctx.pos, &end);
            costs_to_reach.insert(nctx.clone(), ncost);
            steps_to_reach.insert(nctx.clone(), Some(ctx.clone()));
            frontier.push(FrontierPos {
                pos: nctx.clone(),
                prio: Reverse(ncost + iaheu),
            });
        }
    }

    panic!("Failed to find a path");
}

fn heuristic(lhs: &(usize, usize), rhs: &(usize, usize)) -> usize {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    pos: (usize, usize),
    dir: Direction,
    sd_steps: usize,
}

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        let mut neighbours = Vec::with_capacity(4);
        if self.pos.0 > 0 {
            neighbours.push(Position {
                pos: (self.pos.0 - 1, self.pos.1),
                dir: Direction::North,
                sd_steps: if self.dir == Direction::North {
                    self.sd_steps + 1
                } else {
                    1
                },
            });
        }

        if self.pos.1 > 0 {
            neighbours.push(Position {
                pos: (self.pos.0, self.pos.1 - 1),
                dir: Direction::West,
                sd_steps: if self.dir == Direction::West {
                    self.sd_steps + 1
                } else {
                    1
                },
            });
        }

        neighbours.push(Position {
            pos: (self.pos.0 + 1, self.pos.1),
            dir: Direction::South,
            sd_steps: if self.dir == Direction::South {
                self.sd_steps + 1
            } else {
                1
            },
        });

        neighbours.push(Position {
            pos: (self.pos.0, self.pos.1 + 1),
            dir: Direction::East,
            sd_steps: if self.dir == Direction::East {
                self.sd_steps + 1
            } else {
                1
            },
        });

        neighbours
    }
}

#[derive(Clone, Debug)]
struct Map {
    rows: usize,
    cols: usize,
    costs: HashMap<(usize, usize), usize>,
}

impl Map {
    const fn area(&self) -> usize {
        self.rows * self.cols
    }

    const fn contains(&self, pos: &(usize, usize)) -> bool {
        pos.0 < self.rows && pos.1 < self.cols
    }

    fn cost(&self, pos: &(usize, usize)) -> Option<usize> {
        self.costs.get(pos).map(|x| *x)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ridx in 0..self.rows {
            for cidx in 0..self.cols {
                write!(f, "{}", self.costs[&(ridx, cidx)])?;
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let costs = s
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(ridx, l)| {
                l.chars().enumerate().map(move |(cidx, c)| {
                    (
                        (ridx, cidx),
                        usize::from_str_radix(&c.to_string(), 10).unwrap(),
                    )
                })
            })
            .collect::<HashMap<(usize, usize), usize>>();
        let rows = s.lines().count();
        let cols = s
            .lines()
            .next()
            .map(|l| l.chars().count())
            .unwrap_or_default();

        Ok(Map { rows, cols, costs })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn is_opposite(self, othr: Direction) -> bool {
        match (self, othr) {
            (Direction::North, Direction::South) => true,
            (Direction::South, Direction::North) => true,
            (Direction::West, Direction::East) => true,
            (Direction::East, Direction::West) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FrontierPos {
    pos: Position,
    prio: std::cmp::Reverse<usize>,
}

impl PartialOrd for FrontierPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FrontierPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prio.cmp(&other.prio)
    }
}

fn print_steps_on_map(map: &Map, steps: &Vec<Position>) {
    for ridx in 0..map.rows {
        for cidx in 0..map.cols {
            let step = steps.iter().find(|pos| pos.pos == (ridx, cidx));
            let symbol = match step {
                Some(pos) => match pos.dir {
                    Direction::North => '^',
                    Direction::West => '<',
                    Direction::South => 'v',
                    Direction::East => '>',
                },
                None => map
                    .cost(&(ridx, cidx))
                    .unwrap()
                    .to_string()
                    .chars()
                    .next()
                    .unwrap(),
            };
            print!("{symbol}");
        }
        print!("\r\n");
    }
}
