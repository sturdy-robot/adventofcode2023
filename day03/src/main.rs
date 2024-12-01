fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let result = input.lines().collect::<Vec<&str>>();
    let result = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut numbers: Vec<(usize, String)> = Vec::new();
            let mut num = "".to_string();
            let mut p = 0;
            for (i, x) in line.chars().enumerate() {
                if x.is_ascii_digit() {
                    if num.clone() == "".to_string() {
                        p = i;
                    }
                    num.push(x);
                } else {
                    if num.clone() != "".to_string() {
                        numbers.push((p, num.clone()));
                        num = "".to_string();                        
                    }
                }

                if i == line.len() - 1 && num != "".to_string() {
                    numbers.push((p, num.clone()));
                    num = "".to_string();
                }
            }

            let symbols = line
                .chars()
                .enumerate()
                .filter(|(i, x)| !x.is_ascii_digit() && x != &'.')
                .map(|(i, x)| (i, x))
                .collect::<Vec<(usize, char)>>();

            let mut upper_symbols: Vec<(usize, char)> = Vec::new();
            let mut lower_symbols: Vec<(usize, char)> = Vec::new();
            let mut upper_line: &str = "";
            let mut lower_line: &str = "";
            if idx != 0 {
                upper_line = result[idx - 1];
                upper_symbols = upper_line
                    .chars()
                    .enumerate()
                    .filter(|(i, x)| !x.is_ascii_digit() && x != &'.')
                    .map(|(i, x)| (i, x))
                    .collect::<Vec<(usize, char)>>();
            }

            if idx != result.len() - 1 {
                lower_line = result[idx + 1];
                lower_symbols = lower_line
                    .chars()
                    .enumerate()
                    .filter(|(i, x)| !x.is_ascii_digit() && x != &'.')
                    .map(|(i, x)| (i, x))
                    .collect::<Vec<(usize, char)>>();
            }

            numbers
                .iter()
                .map(|number| {
                    let p1 = number.0 as i32;
                    let p2 = p1 + number.1.len() as i32 - 1;
                    if !symbols.is_empty() {
                        for symbol in symbols.iter() {
                            let p3 = symbol.0 as i32;
                            if p3 == (p1 - 1) || (p3 == p2 + 1 ){
                                return number.1.parse::<i32>().unwrap();
                            }
                        }
                    }

                    if !upper_symbols.is_empty() {
                        for symbol in upper_symbols.iter() {
                            let p3 = symbol.0 as i32;
                            if (p3 == p1 - 1 )|| (p3 == p2 + 1) || (p3 >= p1 && p3 <= p2) {
                                return number.1.parse::<i32>().unwrap();
                            }
                        }
                    }

                    if !lower_symbols.is_empty() {
                        for symbol in lower_symbols.iter() {
                            let p3 = symbol.0 as i32;
                            if (p3 == p1 - 1) || (p3 == p2 + 1 ) || (p3 >= p1 && p3 <= p2) {
                                return number.1.parse::<i32>().unwrap();
                            }
                        }
                    }

                    0
                })
                .sum::<i32>()
        })
        .sum::<i32>();
    result.to_string()
}

fn get_numbers(line: &str) -> Vec<(usize, String)> {
    let mut numbers: Vec<(usize, String)> = Vec::new();
    let mut num = "".to_string();
    let mut p = 0;
    for (i, x) in line.chars().enumerate() {
        if x.is_ascii_digit() {
            if num.clone() == "".to_string() {
                p = i;
            }
            num.push(x);
        } else {
            if num.clone() != "".to_string() {
                numbers.push((p, num.clone()));
                num = "".to_string();
            }
        }

        if i == line.len() - 1 && num != "".to_string() {
            numbers.push((p, num.clone()));
            num = "".to_string();
        }
    }

    return numbers;
}

fn part2(input: &str) -> String {
    let result = input.lines().collect::<Vec<&str>>();
    let result = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let symbols = line
                .chars()
                .enumerate()
                .filter(|(i, x)| x == &'*')
                .map(|(i, x)| (i, x))
                .collect::<Vec<(usize, char)>>();

            let numbers_in_line: Vec<(usize, String)> = get_numbers(&line);

            let upper_line;
            let lower_line;
            let mut upper_numbers: Vec<(usize, String)> = Vec::new();
            let mut lower_numbers: Vec<(usize, String)> = Vec::new();

            if idx != 0 {
                upper_line = result[idx - 1];
                upper_numbers = get_numbers(&upper_line);
            }

            if idx != result.len() - 1 {
                lower_line = result[idx + 1];
                lower_numbers = get_numbers(&lower_line);
            }

            symbols.iter().map(|symbol| {
                let p1 = symbol.0 as i32;

                let mut n: Vec<i32> = Vec::new();

                if !numbers_in_line.is_empty() {
                    for number in numbers_in_line.iter() {
                        let p2 = number.0 as i32;
                        let p3 = p2 + number.1.len() as i32 - 1;

                        if (p1 == p3 + 1) || (p1 == p2 - 1) {
                            n.push(number.1.parse::<i32>().unwrap());
                        }
                    }
                }

                if !upper_numbers.is_empty() {
                    for number in upper_numbers.iter() {
                        let p2 = number.0 as i32;
                        let p3 = p2 + number.1.len() as i32 - 1;

                        if (p1 == p3 + 1) || (p1 == p2 - 1) || (p1 >= p2 && p1 <= p3) {
                            n.push(number.1.parse::<i32>().unwrap());
                        }
                    }
                }

                if !lower_numbers.is_empty() {
                    for number in lower_numbers.iter() {
                        let p2 = number.0 as i32;
                        let p3 = p2 + number.1.len() as i32 - 1;

                        if (p1 == p3 + 1) || (p1 == p2 - 1) || (p1 >= p2 && p1 <= p3){
                            n.push(number.1.parse::<i32>().unwrap());
                        }
                    }
                }

                if n.len() > 1 {
                    let mut x = 1;
                    for num in n.iter() {
                        x *= num;
                    }
                    return x;
                } else {
                    return 0;
                }
            }).sum::<i32>()
        })
        .sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), "4361");
    }

    #[test]
    fn test_part1_with_input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "539433");
    }

        #[test]
        fn test_part2() {
            let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
            assert_eq!(part2(input), "467835");
        }

        #[test]
        fn test_part2_with_input() {
            let input = include_str!("../input.txt");
            assert_eq!(part2(input), "75847567");
        }
}
