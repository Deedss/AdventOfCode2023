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

fn get_card_value(c: char) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 11,
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
    value_type_of_cards: i64,
}

impl Hand {
    fn new(cards: &str, bid: i64) -> Self {
        let mut cards_by_values = cards
            .chars()
            .map(|c| get_card_value(c))
            .collect::<Vec<u8>>();
        cards_by_values.sort_by(|a, b| b.cmp(a));
        let (value_type_of_cards, type_of_hands) = get_type_of_hands(&cards_by_values);
        Hand {
            cards: cards_by_values,
            bid,
            type_of_hands,
            value_type_of_cards,
        }
    }
}

fn get_indices(arr: &[i32; 15], count: u8) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, &val) in arr.iter().enumerate() {
        if val == count as i32 {
            indices.push(i);
        }
    }
    indices
}

fn get_type_of_hands(cards_by_values: &Vec<u8>) -> (i64, TypeOfHands) {
    let mut counts = [0; 15];
    let mut sum = 0;
    for c in cards_by_values.iter() {
        counts[*c as usize] += 1;
    }

    if counts.iter().any(|&count| count == 5) {
        let indices = get_indices(&counts, 5);
        sum = indices[0] as i64 * 5;
        (sum, TypeOfHands::FiveOfAKind)
    } else if counts.iter().any(|&count| count == 4) {
        let indices = get_indices(&counts, 4);
        sum = indices[0] as i64 * 4;
        (sum, TypeOfHands::FourOfAKind)
    } else if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
        let mut indices = get_indices(&counts, 3);
        sum = indices[0] as i64 * 3;
        indices = get_indices(&counts, 2);
        sum += indices[0] as i64 * 2;
        (sum, TypeOfHands::FullHouse)
    } else if counts.iter().any(|&count| count == 3) {
        let indices = get_indices(&counts, 3);
        sum = indices[0] as i64 * 3;
        (sum, TypeOfHands::ThreeOfAKind)
    } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
        let indices = get_indices(&counts, 2);
        sum = indices[0] as i64 * 2;
        (sum, TypeOfHands::TwoPair)
    } else if counts.iter().any(|&count| count == 2) {
        let indices = get_indices(&counts, 2);
        sum = indices[0] as i64 * 2;
        (sum, TypeOfHands::OnePair)
    } else {
        // For HighCard, return the sum of card values
        sum = cards_by_values.iter().map(|&c| c as i64).sum();
        (sum, TypeOfHands::HighCard)
    }
}

fn get_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|l| {
            let cards = l.split_whitespace().nth(0).unwrap();
            let bid = l.split_whitespace().nth(1).unwrap().parse::<i64>().unwrap();
            Hand::new(cards, bid)
        })
        .collect()
}

fn compare_by_type_and_value(lhs: &Hand, rhs: &Hand) -> Ordering {
    match lhs.type_of_hands.cmp(&rhs.type_of_hands) {
        Ordering::Equal => match lhs.value_type_of_cards.cmp(&rhs.value_type_of_cards) {
            Ordering::Equal => match lhs.cards.cmp(&rhs.cards) {
                Ordering::Equal => panic!("eq"),
                ordering => return ordering,
            },
            ordering => return ordering,
        },
        ordering => ordering,
    }
}

fn part_1(input: &str) -> i64 {
    let mut hands = get_hands(input);
    let mut total: i64 = 0;

    hands.sort_by(|a, b| compare_by_type_and_value(a, b));
    hands.iter().for_each(|hand| println!("{:?}", hand));
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i + 1) as i64;
    }

    total
}

fn part_2(input: &str) -> i64 {
    0
}

fn main() {
    let example_1 = part_1(include_str!("example.txt"));
    println!("Example part 1 : {}", example_1);
    let part_1 = part_1(include_str!("input.txt"));
    println!("Input part 1 : {}", part_1);

    /*     let example_2 = part_2(include_str!("example.txt"));
    println!("Example part 2 : {}", example_2);
    let part_2 = part_2(include_str!("input.txt"));
    println!("Input part 2 : {}", part_2); */
}
