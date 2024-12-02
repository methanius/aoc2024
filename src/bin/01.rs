use counter::Counter;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::{all_consuming, map_res, recognize};
use nom::sequence::separated_pair;
use nom::IResult;
use std::iter::zip;

fn part_1(text: &str) -> u64 {
    let (mut first, mut last): (Vec<u64>, Vec<u64>) = text
        .lines()
        .map(parse_tuple_ints)
        .map(|a| a.unwrap())
        .map(|(_, tup)| tup)
        .collect();
    first.sort_unstable();
    last.sort_unstable();
    zip(first, last).fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

fn part_2(text: &str) -> u64 {
    let (first, last): (Vec<u64>, Vec<u64>) = text
        .lines()
        .map(parse_tuple_ints)
        .map(|a| a.unwrap())
        .map(|(_, tup)| tup)
        .collect();
    let frequency_map: Counter<_> = last.iter().collect();
    first
        .into_iter()
        .map(|i| i * frequency_map[&i] as u64)
        .sum()
}

fn main() {
    let text: String =
        std::fs::read_to_string("data/01.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn parse_tuple_ints(text: &str) -> IResult<&str, (u64, u64)> {
    all_consuming(separated_pair(
        map_res(recognize(digit1), str::parse),
        multispace1,
        map_res(recognize(digit1), str::parse),
    ))(text)
}

#[cfg(test)]
mod test {

    use crate::parse_tuple_ints;

    use super::*;
    const INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn integer_tuple_parser() {
        assert_eq!(
            parse_tuple_ints("3040102401240 124012041024"),
            Ok(("", (3_040_102_401_240, 124_012_041_024)))
        );
        assert_eq!(
            parse_tuple_ints("402349            2040324"),
            Ok(("", (402_349, 2_040_324)))
        );
    }

    #[test]
    fn test_input_parse() {
        assert_eq!(part_1(INPUT), 11);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 31);
    }
}
