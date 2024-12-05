use itertools::Itertools;
use std::cmp::{max, min};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

// Thanks to HyperNeutrino for the more efficient solution than the one I originally worked on
fn part1(input: &str) -> String {
    let mut blocks_iter = input.split("\n\n");
    let seeds_block = blocks_iter.next().unwrap();
    let blocks = blocks_iter.collect::<Vec<_>>();

    let mut seeds: Vec<i64> = seeds_block
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    for block in blocks {
        let ranges: Vec<(i64, i64, i64)> = block
            .lines()
            .skip(1)
            .map(|line| {
                let nums: Vec<i64> = line
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                (nums[0], nums[1], nums[2])
            })
            .collect::<Vec<(i64, i64, i64)>>();

        let mut new_seeds = vec![];
        for &x in &seeds {
            let mut found = false;
            for &(a, b, c) in &ranges {
                if b <= x && x < b + c {
                    new_seeds.push(x - b + a);
                    found = true;
                    break;
                }
            }
            if !found {
                new_seeds.push(x);
            }
        }
        seeds = new_seeds;
    }

    let result = seeds.iter().min_by_key(|x| *x).unwrap();
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut blocks_iter = input.split("\n\n");
    let seeds_block = blocks_iter.next().unwrap();
    let blocks = blocks_iter.collect::<Vec<_>>();

    let inputs: Vec<i64> = seeds_block
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut seeds: Vec<(i64, i64)> = vec![];
    for i in (0..inputs.len()).step_by(2) {
        seeds.push((inputs[i], inputs[i] + inputs[i + 1]));
    }

    for block in blocks {
        let mut ranges = vec![];
        for line in block.lines().skip(1) {
            let range: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            ranges.push((range[0], range[1], range[2]));
        }

        let mut new_seeds = vec![];
        while let Some((s, e)) = seeds.pop() {
            let mut found = false;
            for &(a, b, c) in &ranges {
                let os = max(s, b);
                let oe = min(e, b + c);
                if os < oe {
                    new_seeds.push((os - b + a, oe - b + a));
                    if os > s {
                        seeds.push((s, os));
                    }
                    if e > oe {
                        seeds.push((oe, e));
                    }
                    found = true;
                    break;
                }
            }

            if !found {
                new_seeds.push((s, e));
            }
        }

        seeds = new_seeds;
    }

    let result = seeds.iter().min_by_key(|&(x, _)| x).unwrap().0;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(input), "35");
    }

    #[test]
    fn test_part1_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "309796150");
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";
        assert_eq!(part2(input), "46");
    }

    #[test]
    fn test_part2_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "50716416");
    }
}
