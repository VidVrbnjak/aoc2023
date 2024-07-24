use std::collections::HashMap;

pub fn p1(input: &str) {
    let (instructions, map) = parse_input(input);
    let mut steps = 0;
    let mut pos = "AAA";
    let mut instructions = instructions.chars().cycle();
    while pos != "ZZZ" {
        let node = map.get(pos).unwrap();
        pos = match instructions.next().unwrap() {
            'R' => &node.1,
            'L' => &node.0,
            c => panic!("Unkwnown instruction {c}"),
        };
        steps += 1;
    }

    println!("It took {steps} steps to go from AAA to ZZZ");
}

pub fn p2(input: &str) {
    let (instructions, map) = parse_input(input);
    let mut ctxs = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|x| x.to_string())
        .map(|x| (x, 0))
        .collect::<Vec<_>>();

    for ctx in ctxs.iter_mut() {
        let steps = &mut ctx.1;
        let pos = &mut ctx.0;
        let mut instructions = instructions.chars().cycle();
        while !pos.ends_with("Z") {
            let node = map.get(pos).unwrap();
            *steps += 1;
            *pos = match instructions.next().unwrap() {
                'L' => node.0.clone(),
                'R' => node.1.clone(),
                c => panic!("Unknown instruction {c}"),
            };
        }
    }

    let steps = ctxs.iter().fold(1, |acc, (_, steps)| lcm(acc, *steps));
    println!("The ghost map took {steps} to solve");
}

fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next().unwrap();

    (
        instructions.to_string(),
        lines
            .map(|l| {
                let mut s = l.split("=");
                let node = s.next().unwrap().trim();
                let mut lr = s
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|s| s.replace("(", "").replace(")", ""));
                (
                    node.to_string(),
                    (
                        lr.next().unwrap().trim().to_string(),
                        lr.next().unwrap().trim().to_string(),
                    ),
                )
            })
            .collect::<HashMap<String, (String, String)>>(),
    )
}
