use std::{fs, cmp::Ordering};

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    initial: String,
    strength: i32,
    value: i32
}

#[derive(Debug)]
struct Card {
    value: char,
    strength: i32,
    count: i32
}

fn get_card_value (card: char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 0,
        _ => card.to_string().parse::<i32>().unwrap(),
    }
}

fn create_hand_struct (hand: Vec<&str>) -> Hand {
    let mut cards: Vec<Card> = vec![];

    for card in hand[0].chars() {
        match cards.iter().position(|c| c.value == card) {
            Some(index) => cards[index].count += 1,
            None => {
                let val = get_card_value(card);
                cards.push(Card {
                    value: card,
                    strength: val,
                    count: 1
                });
            }
        }
    }

    cards.sort_by(|a, b| {
        (b.count, b.strength).cmp(&(a.count, a.strength))
    });

    let strength = cards.iter().map(|e| e.strength * e.count).sum::<i32>();

    Hand {
        cards,
        strength,
        initial: hand[0].to_string(),
        value: hand[1].parse::<i32>().unwrap()
    }
}

fn sort_strength (a: &Hand, b: &Hand) -> Ordering {
    let mut a_high: i32 = 0;
    let mut b_high: i32 = 0;

    for (a_card, b_card) in a.initial.chars().zip(b.initial.chars()) {
        let a_val = get_card_value(a_card);
        let b_val = get_card_value(b_card);

        if a_val == b_val {
            continue;
        } else {
            a_high = a_val;
            b_high = b_val;
            break;
        }
    }

    a_high.cmp(&b_high)
}

fn calculate_hand_ranks (hands: Vec<Hand>) -> i32 {
    let mut high_card: Vec<Hand> = vec![];
    let mut one_pair: Vec<Hand>= vec![];
    let mut two_pair: Vec<Hand>= vec![];
    let mut three_of_a_kind: Vec<Hand> = vec![];
    let mut full_house: Vec<Hand> = vec![];
    let mut four_of_a_kind: Vec<Hand> = vec![];
    let mut five_of_a_kind: Vec<Hand> = vec![];

    // classifier here
    for hand in hands {
        let mut pairs = 0;
        let mut three = 0;
        let mut four = 0;
        let mut five = 0;
        let mut joker = 0;

        for card in hand.cards.iter() {
            if card.value == 'J' {
                joker = 1 * card.count;
            } else if card.count == 2 {
                pairs += 1;
            } else if card.count == 3 {
                three = 1;
            } else if card.count == 4 {
                four = 1;
            } else if card.count == 5 {
                five = 1;
            }
        }

        if pairs == 1 && three == 1 {
            if joker == 1 {
                four_of_a_kind.push(hand);
            } else if joker == 2 {
                five_of_a_kind.push(hand);
            } else {
                full_house.push(hand);
            }
        } else if three == 1 {
            if joker == 1 {
                four_of_a_kind.push(hand);
            } else if joker == 2 {
                five_of_a_kind.push(hand);
            } else {
                three_of_a_kind.push(hand);
            }
        } else if four == 1 {
            if joker == 1 {
                five_of_a_kind.push(hand);
            } else {
                four_of_a_kind.push(hand);
            }
        } else if five == 1 {
            five_of_a_kind.push(hand);
        } else if pairs == 2 {
            if joker == 1 {
                full_house.push(hand);
            } else {
                two_pair.push(hand);
            }
        } else if pairs == 1 {
            if joker == 1 {
                three_of_a_kind.push(hand);
            } else if joker == 2 {
                four_of_a_kind.push(hand);
            } else if joker == 3 {
                five_of_a_kind.push(hand);
            } else {
                one_pair.push(hand);
            }
        } else {
            high_card.push(hand)
        }

    }

    // rank each list
    //
    // concat everything
    high_card.sort_by(|a, b| sort_strength(a, b));
    one_pair.sort_by(|a, b| sort_strength(a, b));
    two_pair.sort_by(|a, b| sort_strength(a, b));
    three_of_a_kind.sort_by(|a, b| sort_strength(a, b));
    full_house.sort_by(|a, b| sort_strength(a, b));
    four_of_a_kind.sort_by(|a, b| sort_strength(a, b));
    five_of_a_kind.sort_by(|a, b| sort_strength(a, b));

    print!("High Card: {:#?}\n", high_card);
    print!("One Pair: {:#?}\n", one_pair);
    print!("Two Pair: {:#?}\n", two_pair);
    print!("Three of a Kind: {:#?}\n", three_of_a_kind);
    print!("Full House: {:#?}\n", full_house);
    print!("Four of a Kind: {:#?}\n", four_of_a_kind);
    print!("Five of a Kind: {:#?}\n", five_of_a_kind);
    
    let mut total = 0;
    let mut rank = 1;

    for hand in high_card {
        total += hand.value * rank;
        rank += 1;
    }

    for hand in one_pair {
        total += hand.value * rank;
        rank += 1;
    }

    for hand in two_pair {
        total += hand.value * rank;
        rank += 1;
    }

    for hand in three_of_a_kind {
        total += hand.value * rank;
        rank += 1;
    }

    for hand in full_house {
        total += hand.value * rank;
        rank += 1;
    }

    for hand in four_of_a_kind {
        total += hand.value * rank;
        rank += 1;
    }

    for hand in five_of_a_kind {
        total += hand.value * rank;
        rank += 1;
    }


    total
}

fn part2 (str: String) -> i32 {
    let mut hands: Vec<Hand> = vec![];
    for line in str.lines() {
        let hand: Vec<&str> = line.split(' ').collect();
        hands.push(create_hand_struct(hand));
    }
    calculate_hand_ranks(hands)
}

// fn part1 (str: String) -> i32 {
//     let mut hands: Vec<Hand> = vec![];
//     for line in str.lines() {
//         let hand: Vec<&str> = line.split(' ').collect();
//         hands.push(create_hand_struct(hand));
//     }
//     calculate_hand_ranks(hands)
// }

// 250049853 - lower

fn main() {
    let mut total = 0;
    if let Ok(lines) = fs::read_to_string("./input") {
        total = part2(lines);
    }

    println!("{}", total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(TEST_DATA.to_string()), 6440)
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_DATA.to_string()), 5905)
    }
}
