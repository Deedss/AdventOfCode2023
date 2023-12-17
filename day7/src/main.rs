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
}

impl Hand {
    fn new(cards: &str, bid: i64) -> Self {
        let cards_by_values = cards
            .chars()
            .map(|c| get_card_value(c))
            .collect::<Vec<u8>>();
        Hand {
            cards: cards_by_values.clone(),
            bid,
            type_of_hands: get_type_of_hands(&cards_by_values),
        }
    }
}
fn get_type_of_hands(cards_by_values: &Vec<u8>) -> TypeOfHands {
    let mut counts = [0; 15];
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
    let mut hands = get_hands(input);
    let mut total: i64 = 0;

    hands.sort_by(|a, b| compare_by_type_and_value(a, b));
    for (i, hand) in hands.iter().enumerate() {
        println!("hand: {:?} and rank:{}", hand, i + 1);
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
