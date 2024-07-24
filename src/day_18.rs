use std::{collections::HashSet, fmt::Display, num::ParseIntError, str::FromStr};

pub fn p1(input: &str) {
    let dp = DigPlan::from_str(input).unwrap();
    let trench = dp.trench();
    let enclosed_area = trench_enclosed_area(&trench);
    let trench_area = trench.len();
    println!(
        "According to the dig plan, it could containt {} cubic meters of lava",
        enclosed_area + trench_area
    );
    println!(
        "Fast enclosed area {}",
        trench_enclosed_area_fast(&dp).abs() + (trench_area / 2) as i128 + 1
    );
}

pub fn p2(input: &str) {
    println!("Skipping day 18 p2");
    return;
    let dp = DigPlan::from_str(input).unwrap().correct();
    let enclosed_area = trench_enclosed_area_fast(&dp);
    println!(
        "According to the dig plan, it could containt {} cubic meters of lava",
        enclosed_area.abs() + (dp.trench().len() / 2) as i128 + 1
    );
}

struct Instruction {
    dir: Direction,
    len: usize,
    rgb: String,
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            str => Err(str.to_string()),
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => 'U',
            Direction::Left => 'L',
            Direction::Down => 'D',
            Direction::Right => 'R',
        }
        .to_string()
    }
}

struct DigPlan {
    instructions: Vec<Instruction>,
}

impl FromStr for DigPlan {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut split = l.split(" ");
                let dir = Direction::from_str(split.next().unwrap()).unwrap();
                let len = usize::from_str_radix(split.next().unwrap(), 10).unwrap();
                let rgb = split
                    .next()
                    .unwrap()
                    .trim_start_matches("(")
                    .trim_end_matches(")")
                    .to_string();

                Instruction { dir, len, rgb }
            })
            .collect::<Vec<_>>();

        Ok(DigPlan { instructions })
    }
}

impl Display for DigPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ins in self.instructions.iter() {
            write!(f, "{} {} ({})\r\n", ins.dir.to_string(), ins.len, ins.rgb)?;
        }

        Ok(())
    }
}

impl DigPlan {
    fn trench(&self) -> Vec<(usize, usize)> {
        let mut trench = Vec::with_capacity(self.instructions.len() + 1);
        let mut pos = (0i32, 0i32);
        for ins in self.instructions.iter() {
            for _ in 0..ins.len {
                match ins.dir {
                    Direction::Up => pos.0 -= 1,
                    Direction::Left => pos.1 -= 1,
                    Direction::Down => pos.0 += 1,
                    Direction::Right => pos.1 += 1,
                };
                trench.push(pos.clone());
            }
        }
        let min_ridx = trench.iter().map(|x| x.0).min().unwrap_or_default();
        let min_cidx = trench.iter().map(|x| x.1).min().unwrap_or_default();

        trench
            .into_iter()
            .map(|(ridx, cidx)| ((ridx - min_ridx) as usize, (cidx - min_cidx) as usize))
            .collect::<Vec<_>>()
    }

    fn correct(mut self) -> Self {
        for instruction in self.instructions.iter_mut() {
            instruction.len = usize::from_str_radix(
                &instruction.rgb.chars().skip(1).take(5).collect::<String>(),
                16,
            )
            .unwrap();
            instruction.dir = match instruction.rgb.chars().skip(6).take(1).next().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                c => panic!("Unknown direction char {c}"),
            };
        }

        self
    }
}

fn trench_enclosed_area(trench: &Vec<(usize, usize)>) -> usize {
    let max_ridx = trench.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let max_cidx = trench.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let trench = trench.iter().collect::<HashSet<_>>();
    let mut q = Vec::new();
    for ridx in 0..=max_ridx {
        if !trench.contains(&(ridx, 0)) {
            q.push((ridx, 0));
        }

        if !trench.contains(&(ridx, max_cidx)) {
            q.push((ridx, max_cidx));
        }
    }

    for cidx in 0..=max_cidx {
        if !trench.contains(&(0, cidx)) {
            q.push((0, cidx));
        }

        if !trench.contains(&(max_ridx, cidx)) {
            q.push((max_ridx, cidx));
        }
    }

    let mut outside = HashSet::new();
    while let Some(p) = q.pop() {
        if trench.contains(&p) {
            continue;
        }

        if !outside.insert(p) {
            continue;
        }

        if p.0 > 0 {
            q.push((p.0 - 1, p.1));
        }

        if p.1 > 0 {
            q.push((p.0, p.1 - 1));
        }

        if p.0 + 1 < max_ridx {
            q.push((p.0 + 1, p.1));
        }

        if p.1 + 1 < max_cidx {
            q.push((p.0, p.1 + 1));
        }
    }

    return (max_ridx + 1) * (max_cidx + 1) - trench.len() - outside.len();
}

fn trench_enclosed_area_fast(dp: &DigPlan) -> i128 {
    let mut rpos = (0_i128, 0_i128);
    let trench = dp
        .instructions
        .iter()
        .map(|ins| {
            match ins.dir {
                Direction::Up => rpos.0 -= ins.len as i128,
                Direction::Left => rpos.1 -= ins.len as i128,
                Direction::Down => rpos.0 += ins.len as i128,
                Direction::Right => rpos.1 += ins.len as i128,
            };
            rpos.clone()
            // let mut pos = (rpos.0 * 2, rpos.1 * 2);
            // match ins.dir {
            //     Direction::Down => pos.0 += 1,
            //     Direction::Right => pos.1 += 1,
            //     Direction::Up => pos.0 -= 1,
            //     Direction::Left => pos.1 -= 1,
            // };
            // pos
        })
        .collect::<Vec<_>>();

    let min_x = trench.iter().map(|(x, _)| *x).min().unwrap_or_default();
    let min_y = trench.iter().map(|(_, y)| *y).min().unwrap_or_default();
    let trench = trench
        .iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect::<Vec<_>>();

    trench
        .iter()
        .zip(trench.iter().cycle().skip(1))
        .map(|(v1, v2)| (v1.0 * v2.1) - (v2.0 * v1.1))
        .sum::<i128>()
        / 2_i128
}

enum Vertical {
    Up,
    Down,
}

enum Horizontal {
    Left,
    Right,
}

struct Dir {
    vertical: Option<(usize, Vertical)>,
    horizontal: Option<(usize, Horizontal)>,
}
