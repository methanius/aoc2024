use std::collections::HashSet;
use std::convert::identity;

use aoc2024::grid::Grid;
use aoc2024::position::Position;

type Day12Grid = Grid<char>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn main() {
    let text: String =
        std::fs::read_to_string("data/12.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn parse_char_grid(input: &str) -> Day12Grid {
    Grid::parse_grid(input, identity)
}

fn char_group_extender(
    grid: &Day12Grid,
    pos: &Position,
    mut acc: HashSet<Position>,
) -> HashSet<Position> {
    acc.insert(*pos);
    grid.get_direct_position_value_neighbours(pos).iter().fold(
        acc,
        |mut acc, (neighbour_pos, c)| {
            if *c
                == grid
                    .get(pos)
                    .expect("Outer function should ensure Some(_) only here")
                && !acc.contains(neighbour_pos)
            {
                acc.extend(char_group_extender(grid, neighbour_pos, acc.clone()));
            }
            acc
        },
    )
}

fn part_1_grouper(input: &str) -> Vec<HashSet<Position>> {
    let mut already_processed: HashSet<Position> = HashSet::new();
    let grid = parse_char_grid(input);
    grid.to_indexed_iterator()
        .fold(Vec::new(), |mut acc, (pos, _c)| {
            if !already_processed.contains(&pos) {
                let group = char_group_extender(&grid, &pos, HashSet::new());
                for p in &group {
                    already_processed.insert(*p);
                }
                acc.push(group);
            }
            acc
        })
}

fn find_fence_len_for_group(group: &HashSet<Position>) -> usize {
    group.iter().fold(0, |mut acc, pos| {
        acc += usize::from(
            pos.row == 0
                || !group.contains(&Position {
                    row: pos.row - 1,
                    ..*pos
                }),
        );
        acc += usize::from(
            pos.col == 0
                || !group.contains(&Position {
                    col: pos.col - 1,
                    ..*pos
                }),
        );
        acc += usize::from(!group.contains(&Position {
            row: pos.row + 1,
            ..*pos
        }));
        acc += usize::from(!group.contains(&Position {
            col: pos.col + 1,
            ..*pos
        }));
        acc
    })
}

fn part_1(input: &str) -> usize {
    part_1_grouper(input).iter().fold(0, |acc, set| {
        acc + set.len() * find_fence_len_for_group(set)
    })
}

fn step_edge(pos_dir: (Position, Direction), group: &HashSet<Position>) -> (Position, Direction) {
    let (pos, direction) = pos_dir;
    match direction {
        Direction::Up => {
            if pos.row == 0 {
                (pos, Direction::Left)
            } else {
                let above = Position::new(pos.row - 1, pos.col);
                let above_right = Position::new(pos.row - 1, pos.col + 1);
                match (group.contains(&above), group.contains(&above_right)) {
                    (false, _) => (pos, Direction::Left),
                    (true, false) => (above, Direction::Up),
                    (true, true) => (above_right, Direction::Right),
                }
            }
        }
        Direction::Left => {
            if pos.col == 0 {
                return (pos, Direction::Down);
            }
            let left = Position::new(pos.row, pos.col - 1);
            if !group.contains(&left) {
                return (pos, Direction::Down);
            }
            if pos.row == 0 {
                return (left, Direction::Left);
            }
            let left_above = Position::new(pos.row - 1, pos.col - 1);
            if group.contains(&left_above) {
                (left_above, Direction::Up)
            } else {
                (left, Direction::Left)
            }
        }
        Direction::Down => {
            let down = Position::new(pos.row + 1, pos.col);
            if !group.contains(&down) {
                return (pos, Direction::Right);
            }
            if pos.col == 0 {
                return (down, Direction::Down);
            }
            let down_left = Position::new(pos.row + 1, pos.col - 1);
            if group.contains(&down_left) {
                (down_left, Direction::Left)
            } else {
                (down, Direction::Down)
            }
        }
        Direction::Right => {
            let right = Position::new(pos.row, pos.col + 1);
            let right_down = Position::new(pos.row + 1, pos.col + 1);
            match (group.contains(&right), group.contains(&right_down)) {
                (false, _) => (pos, Direction::Up),
                (true, false) => (right, Direction::Right),
                (true, true) => (right_down, Direction::Down),
            }
        }
    }
}

fn find_number_of_sides_for_group(group: &HashSet<Position>) -> usize {
    let has_left_edge =
        |pos: &Position| pos.col == 0 || !group.contains(&Position::new(pos.row, pos.col - 1));
    let mut previous_seen: HashSet<(Position, Direction)> = HashSet::new();
    let mut num_edges = 0;
    while let Some(mut start) = group
        .iter()
        .map(|pos| (*pos, Direction::Down))
        .find(|(pos, direction)| has_left_edge(pos) && !previous_seen.contains(&(*pos, *direction)))
    {
        while !previous_seen.contains(&start) {
            previous_seen.insert(start);
            let next_pos = step_edge(start, group);
            num_edges += usize::from(start.1 != next_pos.1);
            start = next_pos;
        }
    }
    num_edges
}

fn part_2(input: &str) -> usize {
    part_1_grouper(input).iter().fold(0, |acc, set| {
        acc + set.len() * find_number_of_sides_for_group(set)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn day_12_test_char_grid_parser() {
        assert_eq!(
            parse_char_grid(INPUT),
            Grid::new(INPUT.lines().map(|l| l.chars().collect()).collect())
        );
    }

    #[test]
    fn day_12_part_1_grouper_test() {
        let grid = parse_char_grid(INPUT);
        assert_eq!(
            char_group_extender(&grid, &Position::new(0, 0), HashSet::new()).len(),
            12
        );
        assert_eq!(
            char_group_extender(&grid, &Position::new(0, 4), HashSet::new()).len(),
            4
        );
    }

    #[test]
    fn day_12_part_1() {
        let input2 = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part_1(input2), 772);
        assert_eq!(part_1(INPUT), 1930);
    }

    #[test]
    fn day_12_part_2() {
        let grid1 = "\
AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part_2(grid1), 80);
        let grid2 = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(part_1_grouper(grid2).len(), 3);
        assert_eq!(part_2(grid2), 236);
        assert_eq!(
            part_2(
                "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            ),
            368
        );
        assert_eq!(part_2(INPUT), 1206);
    }
}
