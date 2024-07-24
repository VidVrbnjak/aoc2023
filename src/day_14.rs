use std::collections::HashMap;

pub fn p1(input: &str) {
    let mut rocks = parse_input(input);
    tilt_north(&mut rocks);
    println!(
        "The weight on the north beam is {}",
        north_beam_load(&rocks)
    );
}

pub fn p2(input: &str) {
    let mut rocks = parse_input(input);
    let mut mem = HashMap::new();

    let mut cycles = 0;
    while cycles < 1_000_000_000 {
        if mem.contains_key(&rocks) {
            println!("Cycles start repeating after {cycles}");
            break;
        }

        let org = rocks.clone();
        tilt_cycle(&mut rocks);
        mem.insert(org, rocks.clone());
        cycles += 1;
    }

    let mut cycle = Vec::new();
    cycle.push(rocks.clone());
    loop {
        let n = mem[cycle.last().unwrap()].clone();
        if n == cycle[0] {
            break;
        }
        cycle.push(n);
    }
    // println!("Cycle length {}", cycle.len());

    let final_state = cycle[(1_000_000_000 - cycles) % cycle.len()].clone();
    // println!("Final state\r\n{}", rocks_to_str(&final_state));
    println!(
        "The weight on the north beam after 1 000 000 000 cycles is {}",
        north_beam_load(&final_state)
    );
}

fn north_beam_load(rocks: &Vec<Vec<Option<Rock>>>) -> usize {
    let rows = rocks.len();
    rocks
        .iter()
        .enumerate()
        .map(|(ridx, row)| {
            row.iter()
                .filter(|r| r.is_some_and(|r| r == Rock::Rounded))
                .map(|_| rows - ridx)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn tilt_cycle(rocks: &mut Vec<Vec<Option<Rock>>>) {
    let round_rocks = count_round_rocks(&rocks);
    // println!("Tilt cycle start:\r\n{}", rocks_to_str(&rocks));

    tilt_north(rocks);
    // println!("North tilted:\r\n{}", rocks_to_str(&rocks));
    if round_rocks != count_round_rocks(&rocks) {
        panic!(
            "North tilt round rocks {round_rocks} => {}",
            count_round_rocks(&rocks)
        );
    }

    tilt_west(rocks);
    // println!("West tilted:\r\n{}", rocks_to_str(&rocks));
    if round_rocks != count_round_rocks(&rocks) {
        panic!(
            "West tilt round rocks {round_rocks} => {}",
            count_round_rocks(&rocks)
        );
    }

    tilt_south(rocks);
    // println!("South tilted:\r\n{}", rocks_to_str(&rocks));
    if round_rocks != count_round_rocks(&rocks) {
        panic!(
            "South tilt round rocks {round_rocks} => {}",
            count_round_rocks(&rocks)
        );
    }

    tilt_east(rocks);
    // println!("East tilted:\r\n{}", rocks_to_str(&rocks));
    if round_rocks != count_round_rocks(&rocks) {
        panic!(
            "East tilt round rocks {round_rocks} => {}",
            count_round_rocks(&rocks)
        );
    }
}

fn rocks_to_str(rocks: &Vec<Vec<Option<Rock>>>) -> String {
    let mut s = String::new();
    rocks.iter().for_each(|row| {
        row.iter().for_each(|r| match r {
            Some(Rock::Square) => s.push('#'),
            Some(Rock::Rounded) => s.push('O'),
            None => s.push('.'),
        });

        s.push_str("\r\n");
    });

    s
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Rounded,
    Square,
}

fn parse_input(input: &str) -> Vec<Vec<Option<Rock>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Some(Rock::Rounded),
                    '#' => Some(Rock::Square),
                    '.' => None,
                    c => panic!("Unknown char {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn tilt_north(a: &mut Vec<Vec<Option<Rock>>>) {
    let rows = a.len();
    let cols = a.first().map(|v| v.len()).unwrap_or(0);
    for ridx in 0..rows {
        for cidx in 0..cols {
            if Some(Rock::Rounded) != a[ridx][cidx] {
                continue;
            }

            let mut empty_ridx: Option<usize> = None;
            for sridx in (0..ridx).rev() {
                match a[sridx][cidx] {
                    Some(Rock::Square) => {
                        if let Some(empty_ridx) = empty_ridx {
                            a[empty_ridx][cidx] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                        }
                        break;
                    }
                    Some(Rock::Rounded) => {
                        if let Some(empty_ridx) = empty_ridx {
                            if sridx == 0 {
                                a[empty_ridx][cidx] = Some(Rock::Rounded);
                                a[ridx][cidx] = None;
                                break;
                            }
                        }
                    }
                    None => {
                        if sridx == 0 {
                            a[0][cidx] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                            break;
                        } else {
                            empty_ridx = Some(sridx)
                        }
                    }
                }
            }
        }
    }
}

fn tilt_west(a: &mut Vec<Vec<Option<Rock>>>) {
    let columns = a.first().map(|a| a.len()).unwrap_or(0);
    let rows = a.len();

    for cidx in 0..columns {
        for ridx in 0..rows {
            if Some(Rock::Rounded) != a[ridx][cidx] {
                continue;
            }

            let mut empty_cidx = None;
            for scidx in (0..cidx).rev() {
                match a[ridx][scidx] {
                    Some(Rock::Square) => {
                        if let Some(empty_cidx) = empty_cidx {
                            a[ridx][empty_cidx] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                        }
                        break;
                    }
                    Some(Rock::Rounded) => {
                        if let Some(empty_cidx) = empty_cidx {
                            if scidx == 0 {
                                a[ridx][empty_cidx] = Some(Rock::Rounded);
                                a[ridx][cidx] = None;
                                break;
                            }
                        }
                    }
                    None => {
                        if scidx == 0 {
                            a[ridx][0] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                            break;
                        } else {
                            empty_cidx = Some(scidx);
                        }
                    }
                }
            }
        }
    }
}

fn tilt_south(a: &mut Vec<Vec<Option<Rock>>>) {
    let rows = a.len();
    let cols = a.first().map(|v| v.len()).unwrap_or_default();

    for ridx in (0..rows).rev() {
        for cidx in 0..cols {
            if Some(Rock::Rounded) != a[ridx][cidx] {
                continue;
            }

            let mut empty_ridx: Option<usize> = None;
            for sridx in ridx..rows {
                match a[sridx][cidx] {
                    Some(Rock::Square) => {
                        if let Some(empty_ridx) = empty_ridx {
                            a[empty_ridx][cidx] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                        }
                        break;
                    }
                    Some(Rock::Rounded) => {
                        if let Some(empty_ridx) = empty_ridx {
                            if sridx + 1 == rows {
                                a[empty_ridx][cidx] = Some(Rock::Rounded);
                                a[ridx][cidx] = None;
                                break;
                            }
                        }
                    }
                    None => {
                        if sridx + 1 == rows {
                            a[rows - 1][cidx] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                        } else {
                            empty_ridx = Some(sridx);
                        }
                    }
                }
            }
        }
    }
}

fn tilt_east(a: &mut Vec<Vec<Option<Rock>>>) {
    let rows = a.len();
    let cols = a.first().map(|v| v.len()).unwrap_or_default();

    for cidx in (0..cols).rev() {
        for ridx in 0..rows {
            if Some(Rock::Rounded) != a[ridx][cidx] {
                continue;
            }

            let mut empty_cidx: Option<usize> = None;
            for scidx in cidx..cols {
                match a[ridx][scidx] {
                    Some(Rock::Square) => {
                        if let Some(empty_cidx) = empty_cidx {
                            a[ridx][empty_cidx] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                        }
                        break;
                    }
                    Some(Rock::Rounded) => {
                        if let Some(empty_cidx) = empty_cidx {
                            if scidx + 1 == cols {
                                a[ridx][empty_cidx] = Some(Rock::Rounded);
                                a[ridx][cidx] = None;
                                break;
                            }
                        }
                    }
                    None => {
                        if scidx + 1 == cols {
                            a[ridx][cols - 1] = Some(Rock::Rounded);
                            a[ridx][cidx] = None;
                            break;
                        } else {
                            empty_cidx = Some(scidx);
                        }
                    }
                }
            }
        }
    }
}

fn count_round_rocks(a: &Vec<Vec<Option<Rock>>>) -> usize {
    a.iter()
        .map(|row| row.iter().filter(|r| **r == Some(Rock::Rounded)).count())
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn north_tilt() {
        let mut rocks = parse_input(INPUT);
        tilt_north(&mut rocks);

        let expected = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        assert_eq!(rocks, parse_input(expected))
    }

    #[test]
    fn west_tilt() {
        let mut rocks = parse_input(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....",
        );
        tilt_west(&mut rocks);

        assert_eq!(
            rocks,
            parse_input(
                "OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#...."
            )
        )
    }

    #[test]
    fn south_tilt() {
        let mut rocks = parse_input(
            "OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....",
        );
        tilt_south(&mut rocks);

        assert_eq!(
            rocks,
            parse_input(
                ".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#...."
            )
        )
    }

    #[test]
    fn east_tilt() {
        let mut rocks = parse_input(
            ".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....",
        );
        tilt_east(&mut rocks);

        assert_eq!(
            rocks,
            parse_input(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            ),
            "Actual:\r\n{}\r\nExpected\r\n.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
            rocks_to_str(&rocks)
        );
    }
}
