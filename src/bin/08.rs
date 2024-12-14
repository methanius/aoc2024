use std::collections::{HashMap, HashSet};
use std::iter::successors;

use itertools::Itertools;

fn main() {
    let text: String =
        std::fs::read_to_string("data/08.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    row: i64,
    col: i64,
}

impl Position {
    const fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

type AntennaMap = HashMap<char, HashSet<Position>>;
fn parse_antennas(input: &str) -> AntennaMap {
    input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .fold(
                    HashMap::new(),
                    |mut acc: AntennaMap, (column, char)| match char {
                        '.' => acc,
                        a if a.is_alphanumeric() => {
                            acc.entry(a).or_default().insert(Position::new(
                                row.try_into().unwrap(),
                                column.try_into().unwrap(),
                            ));
                            acc
                        }
                        _ => panic!(),
                    },
                )
        })
        .fold(HashMap::new(), |acc: AntennaMap, next_map| {
            next_map.into_iter().fold(acc, |mut inner_acc, (k, v)| {
                inner_acc.entry(k).or_default().extend(v);
                inner_acc
            })
        })
}

fn get_grid_size(input: &str) -> Position {
    Position::new(
        input.lines().count().try_into().unwrap(),
        input.lines().next().unwrap().len().try_into().unwrap(),
    )
}

fn possible_antinodes(
    left_antenna: &Position,
    right_antenna: &Position,
    maxes: &Position,
) -> Vec<Position> {
    let delta_pos = Position::new(
        right_antenna.row - left_antenna.row,
        right_antenna.col - left_antenna.col,
    );
    let first_pair: (i64, i64) = (
        right_antenna.row - 2 * delta_pos.row,
        right_antenna.col - 2 * delta_pos.col,
    );
    let second_pair: (i64, i64) = (
        left_antenna.row + 2 * delta_pos.row,
        left_antenna.col + 2 * delta_pos.col,
    );
    [first_pair, second_pair]
        .iter()
        .filter(|pos| pos.0 >= 0 && pos.0 < maxes.row && pos.1 >= 0 && pos.1 < maxes.row)
        .map(|(a, b)| Position::new(*a, *b))
        .collect()
}

fn day_8_engine(
    input: &str,
    antinodes_from_antenna_pairs_fun: &dyn Fn(&Position, &Position, &Position) -> Vec<Position>,
) -> usize {
    let antennas = parse_antennas(input);
    let maxes = get_grid_size(input);
    antennas
        .values()
        .flat_map(|antenna_sets| {
            antenna_sets
                .iter()
                .combinations(2)
                .flat_map(|ants| antinodes_from_antenna_pairs_fun(ants[0], ants[1], &maxes))
                .collect::<Vec<Position>>()
        })
        .fold(
            HashSet::new(),
            |mut acc: HashSet<Position>, position: Position| {
                acc.insert(position);
                acc
            },
        )
        .len()
}
fn part_1(input: &str) -> usize {
    day_8_engine(input, &possible_antinodes)
}

fn possible_antinodes_part_2(
    left_antenna: &Position,
    right_antenna: &Position,
    maxes: &Position,
) -> Vec<Position> {
    let delta = Position::new(
        right_antenna.row - left_antenna.row,
        right_antenna.col - left_antenna.col,
    );
    let filter_fun = |pos: &Position| -> bool {
        pos.row < maxes.row && pos.row >= 0 && pos.col < maxes.col && pos.col >= 0
    };
    let mut res: Vec<Position> = Vec::new();
    let left_posses: Vec<Position> = successors(Some(left_antenna.clone()), |next_pos| {
        let new_pos = Position::new(next_pos.row - delta.row, next_pos.col - delta.col);
        if filter_fun(&new_pos) {
            Some(new_pos)
        } else {
            None
        }
    })
    .collect();
    let right_posses: Vec<Position> = successors(Some(right_antenna.clone()), |next_pos| {
        let new_pos = Position::new(next_pos.row + delta.row, next_pos.col + delta.col);
        if filter_fun(&new_pos) {
            Some(new_pos)
        } else {
            None
        }
    })
    .collect();
    res.extend(left_posses);
    res.extend(right_posses);
    res
}

fn part_2(input: &str) -> usize {
    day_8_engine(input, &possible_antinodes_part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn day_8_test_parse_antennas() {
        assert_eq!(
            parse_antennas(INPUT),
            HashMap::from([
                (
                    'A',
                    HashSet::from([
                        Position::new(5, 6),
                        Position::new(8, 8),
                        Position::new(9, 9),
                    ])
                ),
                (
                    '0',
                    HashSet::from([
                        Position::new(1, 8),
                        Position::new(2, 5),
                        Position::new(3, 7),
                        Position::new(4, 4),
                    ]),
                ),
            ])
        );
    }

    #[test]
    fn day_8_antinonode_test() {
        assert_eq!(
            possible_antinodes(
                &Position::new(4, 4),
                &Position::new(6, 6),
                &Position::new(15, 15)
            ),
            vec![Position::new(2, 2), Position::new(8, 8)]
        );
        assert_eq!(
            possible_antinodes(
                &Position::new(1, 1),
                &Position::new(5, 5),
                &Position::new(10, 10)
            ),
            vec![Position::new(9, 9)]
        );
    }

    #[test]
    fn day_8_part_1() {
        assert_eq!(part_1(INPUT), 14);
    }

    #[test]
    fn day_8_part_2() {
        assert_eq!(part_2(INPUT), 34);
    }
}
