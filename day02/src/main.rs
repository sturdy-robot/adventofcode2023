use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


fn part1(input: &str) -> String {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let result = input.lines().map(|line| {
        let line = line.to_string();
        let game_id = Regex::new(r"Game (\d+)").unwrap();
        let game_id = game_id.captures(&line).unwrap().get(1).unwrap().as_str();
        let red_match = Regex::new(r"(\d+) red").unwrap();
        let green_match = Regex::new(r"(\d+) green").unwrap();
        let blue_match = Regex::new(r"(\d+) blue").unwrap();
        let red = red_match.captures_iter(&line).map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<_>>();
        let green = green_match.captures_iter(&line).map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<_>>();
        let blue = blue_match.captures_iter(&line).map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<_>>();
        let red = red.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let green = green.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let blue = blue.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for r in red.iter() {
            if *r > max_red {
                return 0;
            }
        }

        for g in green.iter() {
            if *g > max_green {
                return 0;
            }
        }

        for b in blue.iter() {
            if *b > max_blue {
                return 0;
            }
        }

        game_id.parse::<i32>().unwrap()
    }).sum::<i32>();
    result.to_string()
}

fn part2(input: &str) -> String {
    let result = input.lines().map(|line| {
        let line = line.to_string();
        let red_match = Regex::new(r"(\d+) red").unwrap();
        let green_match = Regex::new(r"(\d+) green").unwrap();
        let blue_match = Regex::new(r"(\d+) blue").unwrap();
        let red = red_match.captures_iter(&line).map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<_>>();
        let green = green_match.captures_iter(&line).map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<_>>();
        let blue = blue_match.captures_iter(&line).map(|cap| cap.get(1).unwrap().as_str()).collect::<Vec<_>>();
        let red = red.iter().map(|s| s.parse::<i32>().unwrap()).max().unwrap();
        let green = green.iter().map(|s| s.parse::<i32>().unwrap()).max().unwrap();
        let blue = blue.iter().map(|s| s.parse::<i32>().unwrap()).max().unwrap();
        red * blue * green
    }).sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), "8");
    }

    #[test]
    fn test_part1_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "2169");
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), "2286");
    }

    #[test]
    fn test_part2_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "60948");
    }
}
