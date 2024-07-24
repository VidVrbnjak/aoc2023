use std::collections::HashMap;

pub fn p1(input: &str) {
    let schematic = Schematic::from_str(input);

    let sum = schematic
        .numbers
        .iter()
        .filter(|(part_pos, part_number)| {
            neighbours(part_pos, part_number)
                .any(|neighbour| schematic.symbols.contains_key(&neighbour))
        })
        .map(|(_, number)| u32::from_str_radix(&number, 10).unwrap())
        .sum::<u32>();

    println!("The sum of all part numbers adjacent to symbols is {sum}");
}

pub fn p2(input: &str) {
    let schematic = Schematic::from_str(input);

    let part_numbers = schematic
        .numbers
        .iter()
        .filter(|(part_pos, part_number)| {
            neighbours(&part_pos, part_number)
                .any(|neighbour| schematic.symbols.contains_key(&neighbour))
        })
        .map(|(part_pos, part_number)| (part_pos.clone(), part_number.clone()))
        .collect::<HashMap<Pos, String>>();

    let gears = schematic
        .symbols
        .iter()
        .filter(|(_, c)| **c == '*')
        .filter_map(|(gear_pos, _)| {
            let gear_numbers = part_numbers
                .iter()
                .filter(|(part_pos, part_number)| {
                    neighbours(part_pos, part_number)
                        .any(|neighbour| neighbour == (*gear_pos).clone())
                })
                .map(|(_, part_number)| part_number.clone())
                .collect::<Vec<_>>();

            if gear_numbers.len() == 2 {
                Some(gear_numbers)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let gear_ratios = gears.iter().map(|part_numbers| {
        part_numbers
            .iter()
            .map(|f| u32::from_str_radix(&f, 10).unwrap())
            .fold(1, |acc, x| acc * x)
    });

    println!("Sum of gear ratios is {}", gear_ratios.sum::<u32>());
}

#[derive(Default, Debug)]
struct Schematic {
    numbers: HashMap<Pos, String>,
    symbols: HashMap<Pos, char>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos {
    row: usize,
    col: usize,
}

impl Schematic {
    fn from_str(input: &str) -> Schematic {
        let mut schematic = Schematic::default();
        for (row, line) in input.lines().enumerate() {
            let mut part_pos: Option<Pos> = None;
            for (col, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    let part_pos = part_pos.get_or_insert(Pos { row, col });
                    schematic
                        .numbers
                        .entry(part_pos.clone())
                        .or_insert(String::new())
                        .push(c);
                } else {
                    part_pos = None;
                    // Check for symbol
                    if c != '.' {
                        schematic.symbols.insert(Pos { row, col }, c);
                    }
                }
            }
        }

        schematic
    }
}

fn neighbours(pos: &Pos, str: &str) -> Neighburs {
    let top_left = Pos {
        row: if pos.row == 0 { 0 } else { pos.row - 1 },
        col: if pos.col == 0 { 0 } else { pos.col - 1 },
    };

    Neighburs {
        top_left: top_left.clone(),
        bottom_right: Pos {
            row: pos.row + 1,
            col: pos.col + str.chars().count(),
        },
        next: Some(top_left.clone()),
    }
}

#[test]
fn all_neighbours() {
    let pos = Pos { row: 1, col: 1 };
    let mut neighbours = neighbours(&pos, "12");
    assert_eq!(neighbours.next(), Some(Pos { row: 0, col: 0 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 0, col: 1 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 0, col: 2 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 0, col: 3 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 1, col: 0 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 1, col: 1 })); // THIS IS A FAIL BUT W/E
    assert_eq!(neighbours.next(), Some(Pos { row: 1, col: 2 })); // THIS IS A FAIL BUT W/E
    assert_eq!(neighbours.next(), Some(Pos { row: 1, col: 3 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 2, col: 0 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 2, col: 1 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 2, col: 2 }));
    assert_eq!(neighbours.next(), Some(Pos { row: 2, col: 3 }));
    assert_eq!(neighbours.next(), None);
}

struct Neighburs {
    top_left: Pos,
    bottom_right: Pos,
    next: Option<Pos>,
}

impl Iterator for Neighburs {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_none() {
            return None;
        }

        let next = self.next.clone().unwrap();
        if next.col == self.bottom_right.col {
            if next.row == self.bottom_right.row {
                self.next = None;
            } else {
                self.next = Some(Pos {
                    row: next.row + 1,
                    col: self.top_left.col,
                });
            }
        } else {
            self.next = Some(Pos {
                row: next.row,
                col: next.col + 1,
            });
        }

        Some(next)
    }
}
