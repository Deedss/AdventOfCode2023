use core::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TypeOfHands {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn get_card_value(c: char, part: u8) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => {
            if 1 == part {
                11
            } else {
                1
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => todo!(),
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    bid: i64,
    type_of_hands: TypeOfHands,
}

impl Hand {
    fn new(cards: &str, bid: i64, part: u8) -> Self {
        let cards_by_values = cards
            .chars()
            .map(|c| get_card_value(c, part))
            .collect::<Vec<u8>>();

        let type_of_hands;
        if part == 1 {
            type_of_hands = get_type_of_hands(&cards_by_values);
        } else {
            type_of_hands = get_type_of_hands_part_2(&cards_by_values);
        }
        Hand {
            cards: cards_by_values.clone(),
            bid,
            type_of_hands,
        }
    }
}
fn get_type_of_hands(cards_by_values: &Vec<u8>) -> TypeOfHands {
    let mut counts = [0u8; 15];
    for c in cards_by_values.iter() {
        counts[*c as usize] += 1;
    }

    if counts.iter().any(|&count| count == 5) {
        TypeOfHands::FiveOfAKind
    } else if counts.iter().any(|&count| count == 4) {
        TypeOfHands::FourOfAKind
    } else if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
        TypeOfHands::FullHouse
    } else if counts.iter().any(|&count| count == 3) {
        TypeOfHands::ThreeOfAKind
    } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
        TypeOfHands::TwoPair
    } else if counts.iter().any(|&count| count == 2) {
        TypeOfHands::OnePair
    } else {
        TypeOfHands::HighCard
    }
}

fn get_type_of_hands_part_2(cards_by_values: &Vec<u8>) -> TypeOfHands {
    let mut counts = [0u8; 15];
    let mut jokers: u8 = 0;
    for c in cards_by_values.iter() {
        if 1 == *c {
            jokers += 1;
        }
        counts[*c as usize] += 1;
    }

    if counts.iter().any(|&count| count == 5) {
        TypeOfHands::FiveOfAKind
    } else if counts.iter().any(|&count| count + jokers == 4) {
        TypeOfHands::FourOfAKind
    } else if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
        TypeOfHands::FullHouse
    } else if jokers == 1 && counts.iter().filter(|&&count| count == 2).count() == 2 {
        TypeOfHands::FullHouse
    } else if jokers == 2
        && counts.iter().any(|&count| count == 2)
        && counts.iter().any(|&count| count == 1)
    {
        TypeOfHands::FullHouse
    } else if counts.iter().any(|count| *count + jokers == 3) {
        TypeOfHands::ThreeOfAKind
    } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
        TypeOfHands::TwoPair
    } else if jokers == 1 && counts.iter().filter(|c| **c == 2).count() == 1 {
        TypeOfHands::TwoPair
    } else if counts.iter().any(|count| *count + jokers == 2) {
        TypeOfHands::OnePair
    } else {
        TypeOfHands::HighCard
    }
}

fn get_hands(input: &str, part: u8) -> Vec<Hand> {
    input
        .lines()
        .map(|l| {
            let cards = l.split_whitespace().nth(0).unwrap();
            let bid = l.split_whitespace().nth(1).unwrap().parse::<i64>().unwrap();
            Hand::new(cards, bid, part)
        })
        .collect()
}

fn compare_by_type_and_value(lhs: &Hand, rhs: &Hand) -> Ordering {
    match lhs.type_of_hands.cmp(&rhs.type_of_hands) {
        Ordering::Equal => {
            for n in 0..5 {
                match (lhs.cards.get(n as usize), rhs.cards.get(n as usize)) {
                    (Some(l_card), Some(r_card)) => match l_card.cmp(r_card) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    },
                    _ => break, // Stop comparing if one hand has fewer than 5 cards
                }
            }
            Ordering::Equal // All cards are equal
        }
        ordering => ordering,
    }
}

fn part_1(input: &str) -> i64 {
    let mut hands = get_hands(input, 1);
    let mut total: i64 = 0;

    hands.sort_by(|a, b| compare_by_type_and_value(a, b));
    for (i, hand) in hands.iter().enumerate() {
        // println!("hand: {:?} and rank:{}", hand, i + 1);
        total += hand.bid * (i + 1) as i64;
    }

    total
}

fn part_2(input: &str) -> i64 {
    let mut hands = get_hands(input, 2);
    let mut total: i64 = 0;

    hands.sort_by(|a, b| compare_by_type_and_value(a, b));
    for (i, hand) in hands.iter().enumerate() {
        println!("hand: {:?} and rank:{}", hand, i + 1);
        total += hand.bid * (i + 1) as i64;
    }

    total
}

fn main() {
    /*     let example_1 = part_1(include_str!("example.input"));
    println!("Example part 1 : {}", example_1);
    let part_1 = part_1(include_str!("input.input"));
    println!("Input part 1 : {}", part_1) */

    let example_2 = part_2(include_str!("example.input"));
    println!("Example part 2 : {}", example_2);
    assert_eq!(example_2, 5905);
    let part_2 = part_2(include_str!("input.input"));
    println!("Input part 2 : {}", part_2);
    assert_eq!(part_2, 253907829);
    // answer : 253907829
}
