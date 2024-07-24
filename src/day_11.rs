use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

pub fn p1(input: &str) {
    let mut map = parse_map(input);
    // println!("ORG PARSED MAP");
    // print(&map);

    dupe_empty_rows(&mut map);
    dupe_empty_columns(&mut map);
    // println!("\r\nDuped map");
    // print(&map);

    let rows = map.len();
    let columns = map.first().unwrap().len();

    let mut min_dx = usize::MAX;

    let mut galaxies = HashSet::new();
    for row in 0..rows {
        for col in 0..columns {
            if map.get(row).unwrap().get(col).unwrap().is_some() {
                galaxies.insert(POS { row, col });
            }
        }
    }

    let mut galaxy_iter = map
        .iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .map(move |(cidx, opt)| ((ridx, cidx), opt))
        })
        .filter(|(_, opt)| opt.is_some())
        .map(|(idx, _)| idx);

    let mut distance_sum = 0;
    let mut pairs = 0;
    while let Some(g1) = galaxy_iter.next() {
        let mut g2_iter = galaxy_iter.clone();
        while let Some(g2) = g2_iter.next() {
            distance_sum += distance(g1, g2);
            pairs += 1;
        }
    }

    println!("Total pairs {pairs}");
    println!("The distance between all galaxy pairs is {distance_sum}");
}

pub fn p2(input: &str) {
    let map = parse_map(input);
    let rows = map.len();
    let columns = map.first().unwrap().len();
    let mut map = map
        .iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .map(move |(cidx, opt)| ((ridx, cidx), opt))
        })
        .filter(|(_, opt)| opt.is_some())
        .map(|(idx, _)| (idx, idx))
        .collect::<HashMap<_, _>>();

    for ridx in 0..rows {
        if map.iter().filter(|((row, _), _)| *row == ridx).count() == 0 {
            map.iter_mut()
                .filter(|((row, _), _)| *row > ridx)
                .for_each(|(_, v)| v.0 += 999_999);
        }
    }

    for cidx in 0..columns {
        if map.iter().filter(|((_, col), _)| *col == cidx).count() == 0 {
            map.iter_mut()
                .filter(|((_, col), _)| *col > cidx)
                .for_each(|(_, (_, col))| *col += 999_999);
        }
    }

    let map = map.iter().map(|(old, new)| new).collect::<Vec<_>>();
    let mut galaxy_iter = map.iter();
    let mut distance_sum = 0;
    let mut pairs = 0;
    while let Some(g1) = galaxy_iter.next() {
        let mut g2_iter = galaxy_iter.clone();
        while let Some(g2) = g2_iter.next() {
            pairs += 1;
            distance_sum += distance(**g1, **g2);
        }
    }
    println!("Extra expansion pairs {pairs}");
    println!("Extra expansion distance sum {distance_sum}");
}

// 357134917863
// Too high
// 357134560737
// Too Low
// 82000210

#[derive(Clone, Debug, PartialEq, Eq)]
struct POS {
    row: usize,
    col: usize,
}

impl Hash for POS {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.max(self.col).hash(state);
        self.row.min(self.col).hash(state);
    }
}

fn print(map: &Vec<Vec<Option<Galaxy>>>) {
    map.iter().for_each(|v| {
        let l = v
            .iter()
            .map(|opt| if opt.is_some() { 'G' } else { '.' })
            .collect::<String>();
        println!("{l}");
    });
}

fn parse_map(input: &str) -> Vec<Vec<Option<Galaxy>>> {
    let mut map = Vec::new();
    for line in input.lines() {
        map.push(Vec::new());
        let sub_map = map.last_mut().unwrap();
        for c in line.chars() {
            match c {
                '.' => sub_map.push(None),
                '#' => sub_map.push(Some(Galaxy)),
                c => panic!("Unmapped character {c} {}", c.escape_debug()),
            }
        }
    }

    map
}

fn dupe_empty_rows(map: &mut Vec<Vec<Option<Galaxy>>>) {
    let mut line_idx = 0;
    while line_idx < map.len() {
        let line_map = map.get(line_idx).unwrap();
        if line_map.iter().all(|x| x.is_none()) {
            map.insert(line_idx, line_map.clone());
            line_idx += 2;
        } else {
            line_idx += 1;
        }
    }
}

fn dupe_empty_columns(map: &mut Vec<Vec<Option<Galaxy>>>) {
    let mut col_idx = 0;
    {
        while col_idx < map.first().unwrap().len() {
            let mut is_empty = true;
            for row_idx in 0..map.len() {
                if map.get(row_idx).unwrap().get(col_idx).unwrap().is_some() {
                    is_empty = false;
                    break;
                }
            }

            if !is_empty {
                col_idx += 1;
            } else {
                map.iter_mut().for_each(|r| r.insert(col_idx, None));
                col_idx += 2;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Galaxy;

fn pos_distance(p1: &POS, p2: &POS) -> usize {
    p1.row.abs_diff(p2.row) + p1.col.abs_diff(p2.col)
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let distance = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
    // println!(
    //     "Distance between ({},{}) and ({},{}) is {}",
    //     p1.0, p1.1, p2.0, p2.1, distance
    // );
    distance
}
