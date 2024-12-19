use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::many1, sequence::preceded, IResult,
};
use std::collections::HashMap;

fn main() {
    let text: String =
        std::fs::read_to_string("data/11.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Stone {
    engraving: u64,
}

impl Stone {
    const fn new(engraving: u64) -> Self {
        Self { engraving }
    }

    fn blinked(&self) -> Vec<Self> {
        if self.engraving == 0 {
            vec![Self::new(1)]
        } else if self.engraving.to_string().len() % 2 == 0 {
            let number_string = self.engraving.to_string();
            vec![
                Self::new(
                    number_string[..number_string.len() / 2]
                        .parse::<u64>()
                        .expect("Hardcoded AOC pattern"),
                ),
                Self::new(
                    number_string[number_string.len() / 2..]
                        .parse::<u64>()
                        .expect("Hardcoded AOC pattern"),
                ),
            ]
        } else {
            vec![Self::new(&self.engraving * 2024)]
        }
    }
}

fn u64_parser(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_stones(input: &str) -> Vec<Stone> {
    let (_rest, stones) = many1(alt((u64_parser, preceded(tag(" "), u64_parser))))(input)
        .expect("Hardcoded string from AOC");
    stones.iter().map(|d| Stone::new(*d)).collect()
}

fn day_11_driver(input: &str, blinks: usize) -> u64 {
    let stones = parse_stones(input);
    let mut stone_map: HashMap<Stone, u64> =
        stones.into_iter().fold(HashMap::new(), |mut acc, stone| {
            *acc.entry(stone).or_default() += 1;
            acc
        });
    for _ in 0..blinks {
        stone_map = stone_map
            .into_iter()
            .flat_map(|(stone, count)| {
                stone
                    .blinked()
                    .into_iter()
                    .map(move |current_stone| (current_stone, count))
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<Stone, u64>, (stone, count)| {
                    *acc.entry(stone).or_default() += count;
                    acc
                },
            );
    }
    stone_map.values().sum()
}
fn part_1(input: &str) -> u64 {
    day_11_driver(input, 25)
}

fn part_2(input: &str) -> u64 {
    day_11_driver(input, 75)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_11_stone_blinking_part_1() {
        assert_eq!(
            [
                Stone::new(0),
                Stone::new(1),
                Stone::new(10),
                Stone::new(99),
                Stone::new(999),
            ]
            .iter()
            .flat_map(Stone::blinked)
            .collect::<Vec<Stone>>(),
            vec![
                Stone::new(1),
                Stone::new(2024),
                Stone::new(1),
                Stone::new(0),
                Stone::new(9),
                Stone::new(9),
                Stone::new(2_021_976),
            ]
        );
        assert_eq!(
            [125, 17]
                .iter()
                .map(|e| Stone::new(*e))
                .flat_map(|s| Stone::blinked(&s))
                .flat_map(|s| Stone::blinked(&s))
                .flat_map(|s| Stone::blinked(&s))
                .flat_map(|s| Stone::blinked(&s))
                .flat_map(|s| Stone::blinked(&s))
                .flat_map(|s| Stone::blinked(&s))
                .collect::<Vec<Stone>>(),
            [
                2_097_446_912,
                14168,
                4048,
                2,
                0,
                2,
                4,
                40,
                48,
                2024,
                40,
                48,
                80,
                96,
                2,
                8,
                6,
                7,
                6,
                0,
                3,
                2
            ]
            .iter()
            .map(|e| Stone::new(*e))
            .collect::<Vec<Stone>>()
        );
        let mut stones = vec![Stone::new(125), Stone::new(17)];
        for _ in 0..25 {
            stones = stones.iter().flat_map(Stone::blinked).collect();
        }
        assert_eq!(stones.len(), 55312);
    }
}
