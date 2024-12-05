use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand: Vec<char>,
    hand_type: HandType,
    bid: usize,
    rank: usize,
}

fn part1(input: &str) -> String {
    const CARDS_STRENGTH: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let mut strength_map = HashMap::new();
    for i in 0..CARDS_STRENGTH.len() {
        strength_map.insert(CARDS_STRENGTH[i], i);
    }
    let hands = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut all_hands: Vec<Hand> = hands
        .iter()
        .map(|hand| {
            let mut cards = hand[0].chars().collect::<Vec<_>>();
            let bid = hand[1].parse::<usize>().unwrap();

            let mut count_cards = HashMap::new();
            for card in cards.iter_mut() {
                *count_cards.entry(*card).or_insert(0) += 1;
            }

            let hand_type = match count_cards.values().max() {
                Some(&5) => HandType::FiveOfAKind,
                Some(&3) => {
                    if count_cards.keys().len() == 2 {
                        HandType::FullHouse
                    } else {
                        HandType::ThreeOfAKind
                    }
                }
                Some(&4) => HandType::FourOfAKind,
                Some(&2) => {
                    if count_cards.keys().len() == 3 {
                        HandType::TwoPair
                    } else {
                        HandType::OnePair
                    }
                }
                _ => HandType::HighCard,
            };

            Hand {
                hand: cards,
                hand_type,
                bid,
                rank: 0,
            }
        })
        .collect();
    all_hands.sort_by(|a, b| {
        if a.hand_type > b.hand_type {
            return Ordering::Greater;
        } else if a.hand_type < b.hand_type {
            return Ordering::Less;
        } else {
            for i in 0..a.hand.len() {
                if strength_map.get(&a.hand[i]).unwrap() < strength_map.get(&b.hand[i]).unwrap() {
                    return Ordering::Less;
                } else if strength_map.get(&a.hand[i]).unwrap()
                    > strength_map.get(&b.hand[i]).unwrap()
                {
                    return Ordering::Greater;
                } else {
                    continue;
                }
            }
            return Ordering::Equal;
        }
    });
    for (i, hand) in all_hands.iter_mut().enumerate() {
        hand.rank = i + 1;
    }
    let result = all_hands
        .iter()
        .map(|hand| hand.bid * hand.rank)
        .sum::<usize>();
    result.to_string()
}

fn part2(input: &str) -> String {
    const CARDS_STRENGTH: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    let mut strength_map = HashMap::new();
    for i in 0..CARDS_STRENGTH.len() {
        strength_map.insert(CARDS_STRENGTH[i], i);
    }
    let hands = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut all_hands: Vec<Hand> = hands
        .iter()
        .map(|hand| {
            let mut cards = hand[0].chars().collect::<Vec<_>>();
            let bid = hand[1].parse::<usize>().unwrap();

            let mut count_cards = HashMap::new();
            for card in cards.iter_mut() {
                *count_cards.entry(*card).or_insert(0) += 1;
            }

            let hand_type = match count_cards.values().max() {
                Some(&5) => HandType::FiveOfAKind,
                Some(&4) => {
                    if count_cards.get(&'J').is_some() {
                        HandType::FiveOfAKind                        
                    } else {
                        HandType::FourOfAKind
                    }
                }
                Some(&3) => {
                    if count_cards.get(&'J').is_some() {
                        let wildcard = count_cards.get(&'J').unwrap();
                        if wildcard == &3 {
                            if count_cards.keys().len() == 2 {
                                HandType::FiveOfAKind
                            } else {
                                HandType::FourOfAKind
                            }
                        } else if wildcard == &2 {
                            HandType::FiveOfAKind
                        } else {
                            HandType::FourOfAKind
                        }
                    } else {
                        if count_cards.keys().len() == 2 {
                            HandType::FullHouse
                        } else {
                            HandType::ThreeOfAKind
                        }
                    }
                }
                Some(&2) => {
                    if count_cards.get(&'J').is_some() {
                        let wildcard = count_cards.get(&'J').unwrap();
                        if wildcard == &2 {
                            if count_cards.keys().len() == 3 {
                                HandType::FourOfAKind
                            } else {
                                HandType::ThreeOfAKind
                            }
                        } else {
                            if count_cards.keys().len() == 3 {
                                HandType::FullHouse
                            } else {
                                HandType::ThreeOfAKind
                            }
                        }
                    } else {
                        if count_cards.keys().len() == 3 {
                            HandType::TwoPair
                        } else {
                            HandType::OnePair
                        }
                    }
                }
                _ => {
                    if count_cards.get(&'J').is_some() {
                        HandType::OnePair
                    } else {
                        HandType::HighCard
                    }
                },
            };

            Hand {
                hand: cards,
                hand_type,
                bid,
                rank: 0,
            }
        })
        .collect();
    all_hands.sort_by(|a, b| {
        if a.hand_type > b.hand_type {
            return Ordering::Greater;
        } else if a.hand_type < b.hand_type {
            return Ordering::Less;
        } else {
            for i in 0..a.hand.len() {
                if strength_map.get(&a.hand[i]).unwrap() < strength_map.get(&b.hand[i]).unwrap() {
                    return Ordering::Less;
                } else if strength_map.get(&a.hand[i]).unwrap()
                    > strength_map.get(&b.hand[i]).unwrap()
                {
                    return Ordering::Greater;
                } else {
                    continue;
                }
            }
            return Ordering::Equal;
        }
    });
    for (i, hand) in all_hands.iter_mut().enumerate() {
        hand.rank = i + 1;
    }
    let result = all_hands
        .iter()
        .map(|hand| hand.bid * hand.rank)
        .sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(input), "6440");
    }

    // #[test]
    // fn test_part1_with_input() {
    // let input = include_str!("../input.txt");
    // assert_eq!(part1(input), "2065338");
    // }

    #[test]
    fn test_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part2(input), "5905");
    }

    //     #[test]
    //     fn test_part2_with_input() {
    //         let input = include_str!("../input.txt");
    //         assert_eq!(part2(input), "34934171");
    //     }
}
