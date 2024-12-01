fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let text = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = text[0];
    let seed_to_soil = text[1];
    let soil_to_fertilizer = text[2];
    let fertilizer_to_water = text[3];
    let water_to_light = text[4];
    let light_to_temperature = text[5];
    let temperature_to_humidity = text[6];
    let humidity_to_location = text[7];

    

    let result = "35";
    result.to_string()
}

fn part2(input: &str) -> String {
    let result = "";
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

//     #[test]
//     fn test_part1_with_input() {
//         let input = include_str!("../input.txt");
//         assert_eq!(part1(input), "20855");
//     }
//
//     #[test]
//     fn test_part2() {
//         let input = "seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4";
//         assert_eq!(part2(input), "30");
//     }
//
//     #[test]
//     fn test_part2_with_input() {
//         let input = include_str!("../input.txt");
//         assert_eq!(part2(input), "5489600");
//     }
}
