use aoc2024::grid::Grid;
use aoc2024::position::Position;
use std::collections::{HashMap, HashSet};

fn main() {
    let text: String =
        std::fs::read_to_string("data/10.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn walk_to_trail_ends<'a>(grid: &'a Grid<u64>, start: &'a Position) -> Vec<Position> {
    let mut current_iteration_positions: Vec<Position> = Vec::from([*start]);
    for target_at_step in 1..=9 {
        let mut next_iteration_positions = Vec::new();
        for position in &current_iteration_positions {
            for neighbour in position.direct_neighbours() {
                if grid.get(&neighbour) == Some(&target_at_step) {
                    next_iteration_positions.push(neighbour);
                }
            }
        }
        current_iteration_positions = next_iteration_positions;
    }
    current_iteration_positions
}

fn part_1(input: &str) -> u64 {
    let grid = Grid::parse_grid(input, |c: char| {
        u64::from(c.to_digit(10).expect("AOC hardcoded pattern"))
    });
    grid.to_indexed_iterator()
        .filter(|(_pos, height)| **height == 0)
        .fold(
            HashMap::new(),
            |mut acc: HashMap<Position, HashSet<Position>>, (pos, _height)| {
                acc.entry(pos).or_insert_with(|| {
                    let mut pos_set = HashSet::new();
                    pos_set.extend(walk_to_trail_ends(&grid, &pos));
                    pos_set
                });
                acc
            },
        )
        .values()
        .map(|heads: &HashSet<Position>| heads.len() as u64)
        .sum()
}

fn part_2(input: &str) -> u64 {
    let grid = Grid::parse_grid(input, |c: char| {
        u64::from(c.to_digit(10).expect("Hardcoded aoc pattern"))
    });
    grid.to_indexed_iterator()
        .filter(|(_pos, height)| **height == 0)
        .flat_map(|(pos, _height)| walk_to_trail_ends(&grid, &pos))
        .count() as u64
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn day_10_test_grid_parser() {
        assert_eq!(
            Grid::parse_grid(INPUT, |c| c.to_digit(10).expect("Hardcoded test")),
            Grid {
                data: vec![
                    vec![8, 9, 0, 1, 0, 1, 2, 3,],
                    vec![7, 8, 1, 2, 1, 8, 7, 4,],
                    vec![8, 7, 4, 3, 0, 9, 6, 5,],
                    vec![9, 6, 5, 4, 9, 8, 7, 4,],
                    vec![4, 5, 6, 7, 8, 9, 0, 3,],
                    vec![3, 2, 0, 1, 9, 0, 1, 2,],
                    vec![0, 1, 3, 2, 9, 8, 0, 1,],
                    vec![1, 0, 4, 5, 6, 7, 3, 2,],
                ],
            }
        );
    }

    #[test]
    fn day_10_test_walk_to_trail_end() {
        let grid = Grid::parse_grid(INPUT, |c: char| u64::from(c.to_digit(10).expect("Test")));
        let unique_start_ends = |grid: &Grid<u64>, position: &Position| {
            let mut ends = HashSet::new();
            ends.extend(walk_to_trail_ends(grid, position));
            ends
        };
        assert_eq!(unique_start_ends(&grid, &Position::new(0, 2)).len(), 5);
        assert_eq!(unique_start_ends(&grid, &Position::new(0, 4)).len(), 6);
        assert_eq!(unique_start_ends(&grid, &Position::new(2, 4)).len(), 5);
        assert_eq!(unique_start_ends(&grid, &Position::new(4, 6)).len(), 3);
        assert_eq!(unique_start_ends(&grid, &Position::new(5, 2)).len(), 1);
    }

    #[test]
    fn day_10_test_part_1() {
        assert_eq!(part_1(INPUT), 36);
    }

    #[test]
    fn day_10_part_2() {
        assert_eq!(part_2(INPUT), 81);
    }
}
