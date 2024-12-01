fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


fn part1(input: &str) -> String {
    let result = input.lines().map(|line| {
        let digits = line.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        let number = digits.chars().nth(0).unwrap().to_string() + &digits.chars().last().unwrap().to_string();
        number.parse::<i32>().unwrap()
    }).sum::<i32>();
    result.to_string()
}

fn part2(input: &str) -> String {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let result = input.lines().map(|line| {
        let mut line = line.to_string();
        for (i, digit) in digits.iter().enumerate() {
            line = line.replace(digit, format!("{}{}{}", digit, i + 1, digit).as_str())
        }
        let d = line.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        let number = d.chars().nth(0).unwrap().to_string() + &d.chars().last().unwrap().to_string();
        number.parse::<i32>().unwrap()
    }).sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), "142");
    }

    #[test]
    fn test_part1_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "54561");
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(input), "281");
    }

    #[test]
    fn test_part2_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "54076");
    }
}
