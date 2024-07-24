use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn p1(input: &str) {
    let map = parse_pipes(input);
    let start_pos = starting_pos(input);
    println!("Starting position is {start_pos:#?}");
    let lp = find_loop(&start_pos, map);
    println!("The furthest you can get away is {}", lp.len() / 2);
}

pub fn p2(input: &str) {
    let full_map = parse_pipes(input);
    let start_pos = starting_pos(input);
    let fence = find_loop(&start_pos, full_map.clone());
    let mut fence_map = fence
        .iter()
        .skip(1)
        .map(|pos| (pos.clone(), full_map.get(pos).unwrap().clone()))
        .collect::<HashMap<_, _>>();
    let next_pos = fence[1];
    let prev_pos = fence[fence.len() - 1];
    let start_pipe = match (next_pos.0.cmp(&prev_pos.1), next_pos.1.cmp(&prev_pos.1)) {
        (Ordering::Less, Ordering::Less) => Pipe::SW,
        (Ordering::Less, Ordering::Equal) => Pipe::Vertical,
        (Ordering::Less, Ordering::Greater) => Pipe::SE,
        (Ordering::Equal, Ordering::Less) => Pipe::Horizontal,
        (Ordering::Equal, Ordering::Equal) => unreachable!(),
        (Ordering::Equal, Ordering::Greater) => Pipe::Horizontal,
        (Ordering::Greater, Ordering::Less) => Pipe::NW,
        (Ordering::Greater, Ordering::Equal) => Pipe::Vertical,
        (Ordering::Greater, Ordering::Greater) => Pipe::NE,
    };
    fence_map.insert(start_pos, start_pipe);

    let mut expanded_fence_map = HashMap::new();
    for ((row, col), pipe) in fence_map.iter() {
        let new_row = row * 2 + 1;
        let new_col = col * 2 + 1;
        expanded_fence_map.insert((new_row, new_col), pipe.clone());
        match pipe {
            Pipe::Vertical => {
                expanded_fence_map.insert((new_row - 1, new_col), Pipe::Vertical);
                expanded_fence_map.insert((new_row + 1, new_col), Pipe::Vertical);
            }
            Pipe::Horizontal => {
                expanded_fence_map.insert((new_row, new_col - 1), Pipe::Horizontal);
                expanded_fence_map.insert((new_row, new_col + 1), Pipe::Horizontal);
            }
            Pipe::NE => {
                expanded_fence_map.insert((new_row - 1, new_col), Pipe::Vertical);
                expanded_fence_map.insert((new_row, new_col + 1), Pipe::Horizontal);
            }
            Pipe::NW => {
                expanded_fence_map.insert((new_row - 1, new_col), Pipe::Vertical);
                expanded_fence_map.insert((new_row, new_col - 1), Pipe::Horizontal);
            }
            Pipe::SE => {
                expanded_fence_map.insert((new_row + 1, new_col), Pipe::Vertical);
                expanded_fence_map.insert((new_row, new_col + 1), Pipe::Vertical);
            }
            Pipe::SW => {
                expanded_fence_map.insert((new_row + 1, new_col), Pipe::Vertical);
                expanded_fence_map.insert((new_row, new_col - 1), Pipe::Horizontal);
            }
        }
    }
    let columns = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != '\r' && *c != '\n')
        .count();
    let rows = input.lines().count();

    let expanded_columns = columns * 2 + 1;
    let expanded_rows = rows * 2 + 1;

    let mut next_pos = BTreeSet::new();
    let mut outside = HashSet::new();

    for row in 0..expanded_rows {
        if !expanded_fence_map.contains_key(&(row, 0)) {
            next_pos.insert((row, 0));
        }

        if !expanded_fence_map.contains_key(&(row, expanded_columns - 1)) {
            next_pos.insert((row, expanded_columns - 1));
        }
    }

    for col in 0..expanded_columns {
        if !expanded_fence_map.contains_key(&(0, col)) {
            next_pos.insert((0, col));
        }

        if !expanded_fence_map.contains_key(&(expanded_rows - 1, col)) {
            next_pos.insert((expanded_rows - 1, col));
        }
    }

    while next_pos.len() > 0 {
        let pos = next_pos.pop_last().unwrap();
        outside.insert(pos);
        if pos.0 > 0
            && !expanded_fence_map.contains_key(&(pos.0 - 1, pos.1))
            && !outside.contains(&(pos.0 - 1, pos.1))
        {
            next_pos.insert((pos.0 - 1, pos.1));
        }
        if pos.0 + 1 < expanded_rows
            && !expanded_fence_map.contains_key(&(pos.0 + 1, pos.1))
            && !outside.contains(&(pos.0 + 1, pos.1))
        {
            next_pos.insert((pos.0 + 1, pos.1));
        }
        if pos.1 > 0
            && !expanded_fence_map.contains_key(&(pos.0, pos.1 - 1))
            && !outside.contains(&(pos.0, pos.1 - 1))
        {
            next_pos.insert((pos.0, pos.1 - 1));
        }
        if pos.1 + 1 < expanded_columns
            && !expanded_fence_map.contains_key(&(pos.0, pos.1 + 1))
            && !outside.contains(&(pos.0, pos.1 + 1))
        {
            next_pos.insert((pos.0, pos.1 + 1));
        }
    }
    println!("Total area {}", rows * columns);
    println!("Fence squares {}", fence_map.len());
    println!(
        "Outside squares {}",
        outside
            .iter()
            .filter(|x| x.0 % 2 == 1 && x.1 % 2 == 1)
            .count()
    );
    let area = rows * columns
        - fence_map.len()
        - outside
            .iter()
            .filter(|x| x.0 % 2 == 1 && x.1 % 2 == 1)
            .count();

    println!("Fence encloses {area} squares");
}

fn find_loop(
    start_pos: &(usize, usize),
    map: HashMap<(usize, usize), Pipe>,
) -> Vec<(usize, usize)> {
    let mut loop_start = vec![
        (start_pos.0 + 1, start_pos.1),
        (start_pos.0, start_pos.1 + 1),
    ];

    if start_pos.0 > 0 {
        loop_start.push((start_pos.0 - 1, start_pos.1));
    }

    if start_pos.1 > 0 {
        loop_start.push((start_pos.0, start_pos.1 - 1));
    }

    for mut pos in loop_start {
        // print!("Searching loop: ");
        // print!("({}, {})=>({}, {})", start_pos.0, start_pos.1, pos.0, pos.1);
        let mut prev_pos = start_pos.clone();
        let mut l = vec![start_pos.clone()];
        'search: loop {
            if prev_pos.0.abs_diff(pos.0) + prev_pos.1.abs_diff(pos.1) != 1 {
                // panic!(
                //     "Previous position ({}, {}) and current position ({}, {}) are not adjecent",
                //     prev_pos.0, prev_pos.1, pos.0, pos.1,
                // );
            }

            if &pos == start_pos {
                return l;
            }

            match traverse(prev_pos, pos, &map) {
                Some(next_pos) => {
                    // print!("=>({}, {})", next_pos.0, next_pos.1);
                    l.push(pos);
                    prev_pos = pos;
                    pos = next_pos;
                }
                None => {
                    // println!("=>END");
                    break 'search;
                }
            }
        }
    }

    panic!("No loop found")
}

#[test]
fn travers_se_from_s() {
    let pos = (1, 0);
    let pipe_pos = (0, 0);
    let mut map = HashMap::new();
    map.insert((0, 0), Pipe::SE);
    assert_eq!(traverse(pos, pipe_pos, &map), Some((0, 1)));
}

fn traverse(
    from_pos: (usize, usize),
    pipe_pos: (usize, usize),
    map: &HashMap<(usize, usize), Pipe>,
) -> Option<(usize, usize)> {
    let pipe = map.get(&pipe_pos);
    if pipe.is_none() {
        return None;
    }
    let pipe = pipe.unwrap();
    match pipe {
        Pipe::Vertical => {
            if from_pos.0 + 1 == pipe_pos.0 && from_pos.1 == pipe_pos.1 {
                Some((pipe_pos.0 + 1, pipe_pos.1))
            } else if pipe_pos.0 != 0 && from_pos.0 == pipe_pos.0 + 1 && from_pos.1 == pipe_pos.1 {
                Some((pipe_pos.0 - 1, pipe_pos.1))
            } else {
                None
            }
        }
        Pipe::Horizontal => {
            if from_pos.0 == pipe_pos.0 && from_pos.1 + 1 == pipe_pos.1 {
                Some((pipe_pos.0, pipe_pos.1 + 1))
            } else if pipe_pos.1 != 0 && from_pos.0 == pipe_pos.0 && from_pos.1 == pipe_pos.1 + 1 {
                Some((pipe_pos.0, pipe_pos.1 - 1))
            } else {
                None
            }
        }
        Pipe::NE => {
            if from_pos.0 + 1 == pipe_pos.0 && from_pos.1 == pipe_pos.1 {
                Some((pipe_pos.0, pipe_pos.1 + 1))
            } else if pipe_pos.0 != 0 && from_pos.0 == pipe_pos.0 && from_pos.1 == pipe_pos.1 + 1 {
                Some((pipe_pos.0 - 1, pipe_pos.1))
            } else {
                None
            }
        }
        Pipe::NW => {
            if pipe_pos.1 != 0 && from_pos.0 + 1 == pipe_pos.0 && from_pos.1 == pipe_pos.1 {
                Some((pipe_pos.0, pipe_pos.1 - 1))
            } else if pipe_pos.0 != 0 && from_pos.0 == pipe_pos.0 && from_pos.1 + 1 == pipe_pos.1 {
                Some((pipe_pos.0 - 1, pipe_pos.1))
            } else {
                None
            }
        }
        Pipe::SE => {
            if from_pos.0 == pipe_pos.0 + 1 && from_pos.1 == pipe_pos.1 {
                Some((pipe_pos.0, pipe_pos.1 + 1))
            } else if from_pos.0 == pipe_pos.0 && from_pos.1 == pipe_pos.1 + 1 {
                Some((pipe_pos.0 + 1, pipe_pos.1))
            } else {
                None
            }
        }
        Pipe::SW => {
            if pipe_pos.0 != 0 && from_pos.0 == pipe_pos.0 + 1 && from_pos.1 == pipe_pos.1 {
                Some((pipe_pos.0, pipe_pos.1 - 1))
            } else if from_pos.0 == pipe_pos.0 && from_pos.1 + 1 == pipe_pos.1 {
                Some((pipe_pos.0 + 1, pipe_pos.1))
            } else {
                None
            }
        }
    }
}

fn starting_pos(input: &str) -> (usize, usize) {
    let cols = input.lines().next().unwrap().chars().count();
    let (idx, _) = input
        .chars()
        .filter(|x| *x != '\r' && *x != '\n')
        .enumerate()
        .find(|(_, c)| *c == 'S')
        .unwrap();

    ((idx + 1) / cols, idx % cols)
}

fn parse_pipes(input: &str) -> HashMap<(usize, usize), Pipe> {
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            match Pipe::try_from(c) {
                Ok(p) => {
                    map.insert((row, col), p);
                }
                Err(_) => {}
            };
        })
    });
    map
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SE,
    SW,
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::SE),
            _ => Err(()),
        }
    }
}
