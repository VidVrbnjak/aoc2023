use std::{collections::HashSet, sync::Mutex};

static mut SUMMARY: Mutex<usize> = Mutex::new(0);

pub fn p1(input: &str) {
    let patterns = parse_patterns(input);
    let summary = patterns
        .iter()
        .map(find_reflections)
        .map(|r| {
            r.iter()
                .map(|r| match r {
                    Reflection::Horizontal { top, bottom } => bottom * 100,
                    Reflection::Vertical { left, right } => *right,
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Summarizing the patterns, gives us {summary}");
}

pub fn p2(input: &str) {
    let mut patterns = parse_patterns(input);
    patterns.iter_mut().for_each(fix_smudge);

    let summary = patterns
        .iter()
        .map(find_reflections)
        .map(|r| {
            let mut has_horizontal = false;
            let mut has_vertical = false;
            r.iter()
                .map(|r| match r {
                    Reflection::Horizontal { top, bottom } => {
                        if !has_horizontal {
                            has_horizontal = true;
                            bottom * 100
                        } else {
                            bottom * 100
                        }
                    }
                    Reflection::Vertical { left, right } => {
                        if !has_vertical {
                            has_vertical = true;
                            *right
                        } else {
                            *right
                        }
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    unsafe {
        println!("Static SUMMARY {}", SUMMARY.get_mut().unwrap());
    }
}

// pub fn p2(input: &str) {
//     let mut patterns = parse_patterns(input);
//     let org_reflections = patterns.iter().map(find_reflections).collect::<Vec<_>>();
//     patterns.iter_mut().for_each(fix_smudge);
//     let smudge_reflections = patterns.iter().map(find_reflections).collect::<Vec<_>>();

//     let summary = org_reflections
//         .iter()
//         .zip(smudge_reflections)
//         .map(|(org, smudge)| {
//             smudge
//                 .clone()
//                 .iter()
//                 .filter(|r| org.contains(r))
//                 .map(|r| match r {
//                     Reflection::Horizontal { top, bottom } => bottom * 100,
//                     Reflection::Vertical { left, right } => *right,
//                 })
//                 .sum::<usize>()
//         })
//         .sum::<usize>();

//     println!("Summarizing the patterns, with fixed smudges, gives us {summary}");
// }

fn parse_patterns(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns = Vec::new();
    patterns.push(Vec::new());
    for l in input.lines() {
        if l.is_empty() {
            patterns.push(Vec::new());
            continue;
        }

        let pattern = patterns.last_mut().unwrap();
        pattern.push(l.chars().collect::<Vec<_>>());
    }

    patterns
}

fn find_reflections(pattern: &Vec<Vec<char>>) -> Vec<Reflection> {
    let mut reflections = Vec::new();
    for bottom_idx in 1..pattern.len() {
        let top_iter = pattern.iter().take(bottom_idx).rev();
        let bottom_iter = pattern.iter().skip(bottom_idx);
        let mut zipped = top_iter.zip(bottom_iter);
        if zipped.all(|(lhs, rhs)| lhs == rhs) {
            reflections.push(Reflection::Horizontal {
                top: bottom_idx - 1,
                bottom: bottom_idx,
            });
            break;
        }
    }

    for rhs_idx in 1..pattern.first().unwrap().len() {
        let lhs_iter = pattern.iter().map(|f| f.iter().take(rhs_idx).rev());
        let rhs_iter = pattern.iter().map(|f| f.iter().skip(rhs_idx));
        let mut zipped = lhs_iter.zip(rhs_iter);

        if zipped.all(|(lhs, rhs)| lhs.zip(rhs).all(|(lhs, rhs)| lhs == rhs)) {
            reflections.push(Reflection::Vertical {
                left: rhs_idx - 1,
                right: rhs_idx,
            });
            break;
        }
    }

    reflections
}

fn fix_smudge(pattern: &mut Vec<Vec<char>>) {
    let rows = pattern.len();
    let columns = pattern.first().unwrap().len();
    let org_reflection = find_reflections(pattern)
        .into_iter()
        .collect::<HashSet<_>>();
    // cidx = idx % columns
    // ridx = idx / columns

    for bottom_idx in 1..rows {
        let top_iter = pattern
            .iter()
            .enumerate()
            .take(bottom_idx)
            .rev()
            .flat_map(|(ridx, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(cidx, c)| ((ridx, cidx), c))
            });
        let bottom_iter = pattern.iter().skip(bottom_idx).flatten();
        let zipped = top_iter.zip(bottom_iter);
        let mut filtered = zipped.filter_map(
            |(((ridx, cidx), tc), bc)| if tc != bc { Some((ridx, cidx)) } else { None },
        );

        if filtered.clone().count() == 1 {
            let (ridx, cidx) = filtered.next().unwrap();
            let org_pattern = pattern.clone();

            pattern[ridx][cidx] = match pattern[ridx][cidx] {
                '.' => '#',
                '#' => '.',
                c => panic!("Unknown char {c} at ({ridx}, {cidx})"),
            };

            unsafe {
                *SUMMARY.get_mut().unwrap() += bottom_idx * 100;
            }
            return;
        }
    }

    for rhs_idx in 1..columns {
        let lhs_iter = pattern.iter().enumerate().map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .take(rhs_idx)
                .map(move |(cidx, c)| ((ridx, cidx), c))
                .rev()
        });

        let rhs_iter = pattern.iter().map(|row| row.iter().skip(rhs_idx));

        let zipped = lhs_iter.zip(rhs_iter).flat_map(|(lhs, rhs)| lhs.zip(rhs));
        let mut filtered = zipped.filter_map(
            |(((ridx, cidx), lc), rc)| if lc != rc { Some((ridx, cidx)) } else { None },
        );

        if filtered.clone().count() == 1 {
            let (ridx, cidx) = filtered.next().unwrap();
            pattern[ridx][cidx] = match pattern[ridx][cidx] {
                '.' => '#',
                '#' => '.',
                c => panic!("Unknown character {c}"),
            };

            unsafe {
                *SUMMARY.get_mut().unwrap() += rhs_idx;
            }

            return;
        }
    }

    panic!(
        "Smudge not found on pattern\r\n\r\n{}\r\n\r\nOriginal reflections\r\n{:#?}",
        pattern
            .iter()
            .map(|row| {
                let mut row_str = row.iter().collect::<String>();
                row_str.push_str("\r\n");
                row_str
            })
            .collect::<String>(),
        find_reflections(pattern)
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Reflection {
    Horizontal { top: usize, bottom: usize },
    Vertical { left: usize, right: usize },
}

fn print_pattern(pattern: &Vec<Vec<char>>) {
    println!(
        "{}",
        pattern
            .iter()
            .map(|row| {
                let mut row_str = row.iter().collect::<String>();
                row_str.push_str("\r\n");
                row_str
            })
            .collect::<String>()
    )
}
