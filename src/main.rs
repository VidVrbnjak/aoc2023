use std::collections::HashMap;

fn main() {
    q1_p1_trebuchet(include_str!("input1.txt"));
    q1_p2_trebuchet(include_str!("input1.txt"));
    q2_p1_snow_island(include_str!("input2.txt"));
    q2_p2_snow_island(include_str!("input2.txt"));
    aoc2023::day_3::p1(include_str!("input3.txt"));
    aoc2023::day_3::p2(include_str!("input3.txt"));
    aoc2023::day_4::p1(include_str!("input4.txt"));
    aoc2023::day_4::p2(include_str!("input4.txt"));
    aoc2023::day_5::p1(include_str!("input5.txt"));
    aoc2023::day_5_p2::p2(include_str!("input5.txt"));
    aoc2023::day_6::p1(include_str!("input6.txt"));
    aoc2023::day_6::p2(include_str!("input6.txt"));
    aoc2023::day_7::p1(include_str!("input7.txt"));
    aoc2023::day_7_p2::p2(include_str!("input7.txt"));
    aoc2023::day_8::p1(include_str!("input8.txt"));
    aoc2023::day_8::p2(include_str!("input8.txt"));
    aoc2023::day_9::p1(include_str!("input9.txt"));
    aoc2023::day_9::p2(include_str!("input9.txt"));
    aoc2023::day_10::p1(include_str!("input10.txt"));
    aoc2023::day_10::p2(include_str!("input10.txt"));
    aoc2023::day_11::p1(include_str!("input11.txt"));
    aoc2023::day_11::p2(include_str!("input11.txt"));
    aoc2023::day_12::p1(include_str!("input12.txt"));
    aoc2023::day_12::p2(include_str!("input12.txt"));
    aoc2023::day_13::p1(include_str!("input13.txt"));
    aoc2023::day_13::p2(include_str!("input13.txt"));
    aoc2023::day_14::p1(include_str!("input14.txt"));
    aoc2023::day_14::p2(include_str!("input14.txt"));
    aoc2023::day_15::p1(include_str!("input15.txt"));
    aoc2023::day_15::p2(include_str!("input15.txt"));
    aoc2023::day_16::p1(include_str!("input16.txt"));
    aoc2023::day_16::p2(include_str!("input16.txt"));
    aoc2023::day_17::p1(include_str!("input17.txt"));
    aoc2023::day_17::p2(include_str!("input17.txt"));
    aoc2023::day_18::p1(include_str!("input18.txt"));
    aoc2023::day_18::p2(include_str!("input18.txt"));
    aoc2023::day_19::p1(include_str!("input19.txt"));
    aoc2023::day_19::p2(include_str!("input19.txt"));
    aoc2023::day_20::p1(include_str!("input20.txt"));
    aoc2023::day_20::p2(include_str!("input20.txt"));
}

fn q1_p1_trebuchet(input: &str) {
    let sum = input
        .lines()
        .map(|s| {
            let digits = s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
            let first_digit = *digits.first().unwrap_or(&0);
            let last_digit = *digits.last().unwrap_or(&0);
            first_digit * 10 + last_digit
        })
        .sum::<u32>();

    println!("Trebuche calibration value is {sum}");
}

fn q1_p2_trebuchet(input: &str) {
    const WORD_MAP: [(&str, u32); 18] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;

    for line in lines.iter() {
        let mut ls = 0;
        'ld: for ldx in 0..line.len() {
            for p in WORD_MAP {
                if line[ldx..].starts_with(p.0) {
                    ls += 10 * p.1;
                    break 'ld;
                }
            }
        }

        'rd: for rdx in (0..line.len()).rev() {
            for p in WORD_MAP {
                if line[0..(rdx + 1)].ends_with(p.0) {
                    ls += p.1;
                    break 'rd;
                }
            }
        }

        sum += ls;
    }

    println!("Actual calibration value is {sum}");
}

fn q2_p1_snow_island(input: &str) {
    #[derive(Debug)]
    struct Game {
        id: u32,
        max_ballz: HashMap<String, u32>,
    }

    fn parse_game(input: &str) -> Game {
        let mut s = input.split(":");

        let id = u32::from_str_radix(&s.next().unwrap()[5..], 10).unwrap();

        let mut max_ballz = HashMap::new();
        for games in s {
            for game in games.split(";") {
                for ball in game.split(",") {
                    let count = u32::from_str_radix(
                        &ball
                            .trim()
                            .chars()
                            .take_while(|c| c.is_numeric())
                            .collect::<String>(),
                        10,
                    )
                    .unwrap();
                    let color = ball
                        .trim()
                        .chars()
                        .skip_while(|c| c.is_numeric())
                        .collect::<String>()
                        .trim()
                        .to_string();

                    max_ballz
                        .entry(color)
                        .and_modify(|e: &mut u32| {
                            *e = u32::max(*e, count);
                        })
                        .or_insert(count);
                }
            }
        }

        Game { id, max_ballz }
    }

    const TOTALS: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    fn game_is_possible(game: &Game) -> bool {
        for total in TOTALS {
            if game.max_ballz.get(total.0).is_some_and(|x| *x > total.1) {
                return false;
            }
        }

        return true;
    }

    let id_sum = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(parse_game)
        .filter(game_is_possible)
        .map(|x| x.id)
        .sum::<u32>();

    println!("Sum of ids of possible games is {id_sum}");
}

fn q2_p2_snow_island(input: &str) {
    #[derive(Debug)]
    struct Game {
        id: u32,
        max_ballz: HashMap<String, u32>,
    }

    fn parse_game(input: &str) -> Game {
        let mut s = input.split(":");

        let id = u32::from_str_radix(&s.next().unwrap()[5..], 10).unwrap();

        let mut max_ballz = HashMap::new();
        for games in s {
            for game in games.split(";") {
                for ball in game.split(",") {
                    let count = u32::from_str_radix(
                        &ball
                            .trim()
                            .chars()
                            .take_while(|c| c.is_numeric())
                            .collect::<String>(),
                        10,
                    )
                    .unwrap();
                    let color = ball
                        .trim()
                        .chars()
                        .skip_while(|c| c.is_numeric())
                        .collect::<String>()
                        .trim()
                        .to_string();

                    max_ballz
                        .entry(color)
                        .and_modify(|e: &mut u32| {
                            *e = u32::max(*e, count);
                        })
                        .or_insert(count);
                }
            }
        }

        Game { id, max_ballz }
    }

    let power = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(parse_game)
        .map(|g| g.max_ballz.iter().fold(1, |acc, (_, val)| acc * val))
        .sum::<u32>();

    println!("Sum of power of games is {power}");
}
