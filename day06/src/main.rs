fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut text = input.split("\n");

    let time_iter = text.next().unwrap();
    let distance_iter = text.next().unwrap();

    let mut time = time_iter
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut distance = distance_iter
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut total_ways_to_win = vec![];
    for (t, d) in time.iter().zip(distance.iter()) {
        let mut possible_wins = vec![];
        for i in 0..*t {
            let total_distance = (t - i) * i;
            if total_distance > *d {
                possible_wins.push(total_distance);
            }
        }
        total_ways_to_win.push(possible_wins.len());
    }

    let mut result: usize = 1;
    for i in total_ways_to_win.iter() {
        let n = *i;
        result = result.wrapping_mul(n);
    }
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut text = input.lines();

    let time = text
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<i64>();

    let distance = text
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<i64>();

    let mut result = 0;
    for (t, d) in time.iter().zip(distance.iter()) {
        for i in 0..*t {
            let total_distance = (t - i) * i;
            if total_distance > *d {
                result += 1;
            }
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(input), "288");
    }

    #[test]
    fn test_part1_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "2065338");
    }

    #[test]
    fn test_part2() {
        let input = "Time:      71530
Distance:  940200";
        assert_eq!(part2(input), "71503");
    }

    #[test]
    fn test_part2_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "34934171");
    }
}
