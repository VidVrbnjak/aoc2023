use std::{collections::HashMap, fmt::Display, io::Write};

// Already tested
// 251809391
// 252113175
// 252686427
// 252284834
// 252091782
// 252091782
// 250946742
// 251824095

pub fn p2(input: &str) {
    let mut hands = parse_hands(input);
    let mut fs = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open("output7.txt")
        .unwrap();
    for hand in hands.iter() {
        writeln!(
            &mut fs,
            "{:?} {:#?} {}",
            hand.cards, hand.strength, hand.bid
        )
        .unwrap();
    }
    hands.sort_by(|x, y| match x.strength.cmp(&y.strength) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => x.cards.cmp(&y.cards),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });

    let res = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + ((idx as u64 + 1) * hand.bid));

    println!("Jokers total winnings {res}");
}

fn parse_hands(str: &str) -> Vec<Hand> {
    str.lines().map(parse_hand).collect()
}

fn parse_hand(str: &str) -> Hand {
    let cards = str.split(" ").next().unwrap().chars().collect::<Vec<_>>();
    let cards = [
        Card::from_str(&cards[0].to_string()),
        Card::from_str(&cards[1].to_string()),
        Card::from_str(&cards[2].to_string()),
        Card::from_str(&cards[3].to_string()),
        Card::from_str(&cards[4].to_string()),
    ];
    let strength = Strength::from(&cards);
    let bid = u64::from_str_radix(str.split(" ").skip(1).next().unwrap(), 10).unwrap();

    let hand = Hand {
        cards,
        strength,
        bid,
    };

    hand
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug)]
enum Strength {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for Strength {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn value(str: &Strength) -> usize {
            match str {
                Strength::Five => 6,
                Strength::Four => 5,
                Strength::FullHouse => 4,
                Strength::Three => 3,
                Strength::TwoPair => 2,
                Strength::OnePair => 1,
                Strength::HighCard => 0,
            }
        }

        value(self).cmp(&value(other))
    }
}

impl From<&[Card; 5]> for Strength {
    fn from(value: &[Card; 5]) -> Self {
        let mut counts = HashMap::new();
        for card in value {
            counts
                .entry(card)
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        }

        let jokers = counts.remove_entry(&Card::Joker).map(|x| x.1).unwrap_or(0);
        if jokers == 5 || jokers == 4 {
            return Strength::Five;
        }

        if counts.iter().any(|x| x.1 + jokers == 5) {
            return Strength::Five;
        }

        if counts.iter().any(|x| x.1 + jokers == 4) {
            return Strength::Four;
        }

        for t in counts.iter() {
            if t.1 + jokers < 3 {
                continue;
            }

            let remaining_jokers = jokers - 3 + t.1;
            if counts
                .iter()
                .filter(|x| x.0 != t.0)
                .any(|x| x.1 + remaining_jokers == 2)
            {
                return Strength::FullHouse;
            }
        }

        // if counts.iter().any(|x| x.1 + jokers == 3) && counts.iter().any(|x| *x.1 == 2) {
        //     return Strength::FullHouse;
        // }

        // if counts.iter().any(|x| *x.1 == 3) && counts.iter().any(|x| x.1 + jokers == 2) {
        //     return Strength::FullHouse;
        // }

        if counts.iter().any(|x| x.1 + jokers == 3) {
            return Strength::Three;
        }

        for t in counts.iter() {
            if t.1 + jokers < 2 {
                continue;
            }

            let remaining_jokers = jokers - 2 + t.1;
            if counts
                .iter()
                .filter(|x| x.0 != t.0)
                .any(|x| x.1 + remaining_jokers == 2)
            {
                return Strength::TwoPair;
            }
        }

        // if counts.iter().any(|x| x.1 + jokers == 2) && counts.iter().all(|x| *x.1 == 2) {
        //     return Strength::TwoPair;
        // }

        if counts.iter().any(|x| x.1 + jokers == 2) {
            return Strength::OnePair;
        }

        return Strength::HighCard;
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    strength: Strength,
    bid: u64,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_str(str: &str) -> Card {
        match str {
            "2" => Card::Two,
            "3" => Card::Three,
            "4" => Card::Four,
            "5" => Card::Five,
            "6" => Card::Six,
            "7" => Card::Seven,
            "8" => Card::Eight,
            "9" => Card::Nine,
            "T" => Card::Ten,
            "J" => Card::Joker,
            "Q" => Card::Queen,
            "K" => Card::King,
            "A" => Card::Ace,
            _ => panic!("Cannot parse card from \"{str}\""),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn value(c: &Card) -> u8 {
            match c {
                Card::Joker => 1,
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Ten => 10,
                Card::Queen => 12,
                Card::King => 13,
                Card::Ace => 14,
            }
        }

        value(self).cmp(&value(&other))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Display for Card {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Ten => "T",
            Card::Joker => "J",
            Card::Queen => "Q",
            Card::King => "K",
            Card::Ace => "A",
        };

        write!(&mut f, "{}", c)
    }
}
