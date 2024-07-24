use std::str::FromStr;

pub fn p1(input: &str) {
    let hash_sum = input.split(",").map(hash).sum::<usize>();
    println!("Hash sum: {hash_sum}");
}

const INIT: Vec<Lens> = Vec::new();

pub fn p2(input: &str) {
    let mut bxs = [INIT; 256];
    for instruction in input.split(",").map(parse_instruction) {
        match instruction {
            Instruction::Remove(label) => {
                let bx = &mut bxs[hash(&label)];
                let idx = bx.iter().enumerate().find_map(|(idx, lens)| {
                    if lens.label == label {
                        Some(idx)
                    } else {
                        None
                    }
                });
                if let Some(idx) = idx {
                    bx.remove(idx);
                }
            }
            Instruction::Upsert(label, focal_length) => {
                let bx = &mut bxs[hash(&label)];

                let idx = bx.iter().enumerate().find_map(|(idx, lens)| {
                    if lens.label == label {
                        Some(idx)
                    } else {
                        None
                    }
                });

                let lens = Lens {
                    label,
                    focal_length,
                };

                match idx {
                    Some(idx) => bx[idx] = lens,
                    None => bx.push(lens),
                }
            }
        }
    }

    let focusing_power = bxs
        .iter()
        .enumerate()
        .map(|(bxidx, bx)| {
            bx.iter()
                .enumerate()
                .map(|(lidx, lens)| (bxidx + 1) * (lidx + 1) * (lens.focal_length as usize))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("The focusing power of the lens configuration is {focusing_power}");
}

#[derive(Clone)]
struct Lens {
    focal_length: u8,
    label: String,
}

enum Instruction {
    Remove(String),
    Upsert(String, u8),
}

fn parse_instruction(str: &str) -> Instruction {
    let label = str
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect::<String>();

    let mut op_char_iter = str.chars().skip_while(|c| c.is_ascii_alphabetic());
    match op_char_iter.next().unwrap() {
        '-' => Instruction::Remove(label),
        '=' => Instruction::Upsert(
            label.clone(),
            u8::from_str_radix(&op_char_iter.next().unwrap().to_string(), 10).unwrap(),
        ),
        c => panic!("Unknown op char {c}"),
    }
}

fn hash(input: &str) -> usize {
    let mut hash = 0;
    for c in input.chars() {
        if c.is_ascii_control() {
            continue;
        }
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }
    hash as usize
}

#[test]
fn hasher_should_work() {
    assert_eq!(hash("HASH"), 52);
}
