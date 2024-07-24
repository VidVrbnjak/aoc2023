pub fn p1(input: &str) {
    let cards = input.lines().map(parse_card).collect::<Vec<_>>();
    let card_points = cards.iter().map(|card| {
        card.picked_winners()
            .fold(0u32, |acc, _| u32::max(1u32, acc * 2))
    });

    println!("Total card points {}", card_points.sum::<u32>());
}

pub fn p2(input: &str) {
    let mut cards = input
        .lines()
        .map(parse_card)
        .map(|c| (1_usize, c.picked_winners().count() as u32))
        .collect::<Vec<_>>();

    for idx in 0..cards.len() {
        let (number_of_cards, picked_winners) = cards[idx];
        for offsett in 0..picked_winners {
            cards[idx + 1 + offsett as usize].0 += number_of_cards;
        }
    }

    println!(
        "We have a total of {} scratch cards",
        cards.iter().map(|(cnt, _)| cnt).sum::<usize>()
    );
}

struct Card {
    nr: u32,
    winning_numbers: Vec<u32>,
    picked_numbers: Vec<u32>,
}

fn parse_card(input: &str) -> Card {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let mut s = input.split(":");
    let nr = u32::from_str_radix(
        &s.next()
            .unwrap()
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .collect::<String>(),
        10,
    )
    .unwrap();

    let mut s = s.next().unwrap().split("|");
    let winning_numbers = s
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|x| u32::from_str_radix(x, 10).ok())
        .collect::<Vec<_>>();
    let actual_numbers = s
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|x| u32::from_str_radix(x, 10).ok())
        .collect::<Vec<_>>();

    Card {
        nr,
        winning_numbers,
        picked_numbers: actual_numbers,
    }
}

impl Card {
    fn picked_winners(&self) -> impl Iterator<Item = u32> {
        let mut picked_winners = Vec::new();
        for picked_number in self.picked_numbers.iter() {
            if self.winning_numbers.contains(picked_number) {
                picked_winners.push(*picked_number);
            }
        }

        picked_winners.into_iter()
    }
}
