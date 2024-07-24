use std::collections::HashMap;

pub fn p1(input: &str) {
    let mut hands = parse_hands(input);

    hands.sort_by(|x, y| match x.strength.cmp(&y.strength) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => x.cards.cmp(&y.cards),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });

    let res = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + (idx as u32 + 1) * hand.bid);

    println!("Betting winnings {res}");
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
    let bid = u32::from_str_radix(str.split(" ").skip(1).next().unwrap(), 10).unwrap();
    {
        Hand {
            cards,
            strength,
            bid,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
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

        if counts.iter().any(|x| *x.1 == 5) {
            return Strength::Five;
        }

        if counts.iter().any(|x| *x.1 == 4) {
            return Strength::Four;
        }

        if counts.iter().any(|x| *x.1 == 3) && counts.iter().any(|x| *x.1 == 2) {
            return Strength::FullHouse;
        }

        if counts.iter().any(|x| *x.1 == 3) {
            return Strength::Three;
        }

        if counts.iter().filter(|x| *x.1 == 2).count() == 2 {
            return Strength::TwoPair;
        }

        if counts.iter().any(|x| *x.1 == 2) {
            return Strength::OnePair;
        }

        return Strength::HighCard;
    }
}

struct Hand {
    cards: [Card; 5],
    strength: Strength,
    bid: u32,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
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
    Jack,
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
            "J" => Card::Jack,
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
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Ten => 10,
                Card::Jack => 11,
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
