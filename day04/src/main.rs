use std::collections::BTreeMap;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let overlapping_numbers = get_overlapping_numbers(&line);
            if overlapping_numbers.is_empty() {
                0
            } else {
                let mut sm = 0;
                for (idx, _) in overlapping_numbers.iter().enumerate() {
                    if idx == 0 {
                        sm += 1;
                    } else {
                        sm *= 2;
                    }
                }

                sm
            }
        })
        .sum::<u32>();
    result.to_string()
}

fn get_overlapping_numbers(line: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let rp = Regex::new(r"Card(\s+)(\d+):").unwrap();
    let line = rp.replace(&line, "").trim().to_string();
    let (winning_numbers_text, my_numbers_text) = line.split_once('|').unwrap();
    let winning_numbers = re
        .captures_iter(&winning_numbers_text)
        .map(|c| c[1].parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let my_numbers = re
        .captures_iter(&my_numbers_text)
        .map(|c| c[1].parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    my_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .copied()
        .collect::<Vec<u32>>()
}

fn part2(input: &str) -> String {
    let c = input.lines().collect::<Vec<&str>>();
    let c = c
        .iter()
        .enumerate()
        .map(|(card_number, line)| {
            let overlapping_numbers = get_overlapping_numbers(&line);
            (card_number + 1, overlapping_numbers.len())
        })
        .collect::<Vec<(usize, usize)>>();

    let card_numbers = (1..=c.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();

    let result = c
        .iter()
        .fold(card_numbers, |mut acc, (index, card_score)| {
            for i in *index + 1..(*index + 1 + *card_score) {
                let add_value = *acc.get(&index).unwrap();
                acc.entry(i).and_modify(|value| {
                    *value += add_value;
                });
            }

            acc
        })
        .values()
        .sum::<u32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), "13");
    }

    #[test]
    fn test_part1_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "20855");
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), "30");
    }

    #[test]
    fn test_part2_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "5489600");
    }
}
