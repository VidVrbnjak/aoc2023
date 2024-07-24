pub fn p1(input: &str) {
    let races = parse_races(input);
    let mult = races.iter().fold(1, |acc, r| acc * possible_solutions(r));

    println!("{mult}");
}

pub fn p2(input: &str) {
    let race = parse_race_p2(input);
    let solutions = possible_solutions(&race);
    println!("{solutions}");
}

#[derive(Clone, Debug)]
struct Race {
    time: f64,
    distance: f64,
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines = input
        .lines()
        .map(|x| {
            x.split(":")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .filter_map(|x| u32::from_str_radix(x, 10).ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .next()
        .unwrap()
        .iter()
        .zip(lines.iter().skip(1).next().unwrap().iter())
        .map(|(time, distance)| Race {
            time: (*time).into(),
            distance: (*distance).into(),
        })
        .collect::<Vec<_>>()
}

fn parse_race_p2(input: &str) -> Race {
    let time = u64::from_str_radix(
        &input
            .lines()
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .replace(" ", ""),
        10,
    )
    .unwrap();
    let distance = u64::from_str_radix(
        &input
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .replace(" ", ""),
        10,
    )
    .unwrap();

    Race {
        time: time as f64,
        distance: distance as f64,
    }
}

fn roots(a: f64, b: f64, c: f64) -> (f64, Option<f64>) {
    let ac4 = a * c * 4f64;
    let a2 = a * 2f64;

    let r1 = (-b + (b.powf(2f64) - ac4).sqrt()) / a2;
    let r2 = (-b - (b.powf(2f64) - ac4).sqrt()) / a2;

    if (r1 - r2).abs() < 0.0001 {
        (r1, None)
    } else {
        (r1, Some(r2))
    }
}

fn possible_solutions(r: &Race) -> usize {
    let (mut r1, r2) = roots(-1_f64, r.time, -r.distance);
    if r2.is_none() {
        return 0;
    }

    let mut r2 = r2.unwrap();

    r1 = r1.max(0f64);
    r2 = r2.max(0f64);

    let mut solutions = if r2 <= r1 {
        r1.floor() - r2.ceil() + 1_f64
    } else {
        r2.floor() - r1.ceil() + 1_f64
    };
    if r1.floor() == r1 {
        solutions = solutions - 1_f64;
    }

    if r2.floor() == r2 {
        solutions = solutions - 1_f64;
    }

    solutions as usize
}
