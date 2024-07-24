use std::ops::Add;

pub fn p2(input: &str) {
    let seeds = parse_seeds(input);
    let maps = parse_maps(input);
    let min = seeds
        .iter()
        .flat_map(|nr| apply_map(maps.get(0).unwrap(), &nr))
        .flat_map(|nr| apply_map(maps.get(1).unwrap(), &nr))
        .flat_map(|nr| apply_map(maps.get(2).unwrap(), &nr))
        .flat_map(|nr| apply_map(maps.get(3).unwrap(), &nr))
        .flat_map(|nr| apply_map(maps.get(4).unwrap(), &nr))
        .flat_map(|nr| apply_map(maps.get(5).unwrap(), &nr))
        .flat_map(|nr| apply_map(maps.get(6).unwrap(), &nr))
        .min_by(|x, y| x.start.cmp(&y.start))
        .unwrap();

    println!("{min:#?}");
    println!("{}", min.start);
}

fn parse_maps(input: &str) -> Vec<Vec<Jump>> {
    let mut maps = Vec::new();
    let mut map_idx = None;
    for line in input.lines().skip(2) {
        if line.contains(":") {
            maps.push(Vec::new());
            map_idx = map_idx.and_then(|x| Some(x + 1)).or(Some(0));
        } else if maps.len() > 0 {
            let map = maps.get_mut(map_idx.unwrap()).unwrap();
            let jump_numbers = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| usize::from_str_radix(x, 10).unwrap())
                .collect::<Vec<_>>();

            if jump_numbers.len() > 0 {
                map.push(Jump {
                    sr_start: *jump_numbers.get(1).unwrap(),
                    dr_start: *jump_numbers.get(0).unwrap(),
                    len: *jump_numbers.get(2).unwrap(),
                });
            }
        }
    }

    for map in maps.iter_mut() {
        map.sort_by(|x, y| x.sr_start.cmp(&y.sr_start));
    }

    for map in maps.iter_mut() {
        let mut idx = 0;
        let mut last_sr = 0;
        let mut last_len = 0;
        while let Some(jump) = map.get(idx) {
            if last_sr + last_len < jump.sr_start {
                let sr_start = last_sr + last_len;
                let dr_start = last_sr + last_len;
                let len = jump.sr_start - sr_start;
                map.insert(
                    idx,
                    Jump {
                        sr_start,
                        dr_start,
                        len,
                    },
                );
                last_sr = sr_start;
                last_len = len;
                idx += 1;
            } else {
                last_sr = jump.sr_start;
                last_len = jump.len;
                idx += 1;
            }
        }

        let last_jump = map.last().unwrap();
        if last_jump.sr_start + last_jump.len < usize::MAX {
            map.push(Jump {
                sr_start: last_jump.sr_start + last_jump.len,
                dr_start: last_jump.sr_start + last_jump.len,
                len: usize::MAX - last_jump.sr_start - last_jump.len,
            });
        }
    }

    maps
}

fn parse_seeds(input: &str) -> Vec<NR> {
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|x| usize::from_str_radix(x, 10).ok())
        .collect::<Vec<_>>();

    numbers
        .iter()
        .step_by(2)
        .zip(numbers.iter().skip(1).step_by(2))
        .map(|(start, len)| NR {
            start: *start,
            len: *len,
        })
        .collect()
}

fn apply_map(map: &Vec<Jump>, nr: &NR) -> Vec<NR> {
    // Map must be ordered ASC
    let mut nrs = Vec::new();
    for jump in map.iter() {
        if nr.start + nr.len < jump.sr_start {
            continue;
        }

        if nr.start >= jump.sr_start + jump.len {
            continue;
        }

        let (sr_start, len) = match (
            jump.sr_start < nr.start,
            jump.sr_start + jump.len < nr.start + nr.len,
        ) {
            (true, true) => {
                // Left bound by NR
                // Right bound by JUMP
                (nr.start, jump.sr_start + jump.len - nr.start)
            }
            (true, false) => {
                // Left bound by NR
                // Right bound by NR
                (nr.start, nr.len)
            }
            (false, true) => {
                // Left bound by JUMP
                // Right bound by JUMP
                (jump.sr_start, jump.len)
            }
            (false, false) => {
                // Left bound by JUMP
                // Right bound by NR
                (jump.sr_start, nr.start + nr.len - jump.sr_start)
            }
        };

        let left_shrink = sr_start - jump.sr_start;
        nrs.push(NR {
            start: jump.dr_start + left_shrink,
            len,
        });
    }

    if nrs.len() == 0 {
        nrs.push(nr.clone());
    }

    // println!("{nr:#?} => {:#?}", nrs);
    nrs
}

#[derive(Clone, Debug)]
struct NR {
    start: usize,
    len: usize,
}

#[derive(Clone, Debug)]
struct Jump {
    sr_start: usize,
    dr_start: usize,
    len: usize,
}
