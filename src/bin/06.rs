use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

fn main() {
    let text: String =
        std::fs::read_to_string("data/06.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    const fn step(&self, max_positions: &Position) -> Option<Position> {
        match self.direction {
            Direction::Up => {
                if self.position.row == 0 {
                    None
                } else {
                    Some(Position::new(self.position.row - 1, self.position.column))
                }
            }
            Direction::Left => {
                if self.position.column == 0 {
                    None
                } else {
                    Some(Position::new(self.position.row, self.position.column - 1))
                }
            }
            Direction::Right => {
                if self.position.column == max_positions.column - 1 {
                    None
                } else {
                    Some(Position::new(self.position.row, self.position.column + 1))
                }
            }
            Direction::Down => {
                if self.position.row == max_positions.row - 1 {
                    None
                } else {
                    Some(Position::new(self.position.row + 1, self.position.column))
                }
            }
        }
    }
}

type Obstacles = HashSet<Position>;

fn parser_for_part_1(text: &str) -> (Guard, Obstacles, Position) {
    let (maybe_guard, obstacles) = text
        .lines()
        .enumerate()
        .map(|(row, l): (usize, &str)| {
            l.chars().enumerate().fold(
                (None, HashSet::new()),
                |(g, mut obstacles): (Option<Guard>, Obstacles), (column, char)| match char {
                    '.' => (g, obstacles),
                    '#' => {
                        obstacles.insert(Position { row, column });
                        (g, obstacles)
                    }
                    '^' => (
                        Some(Guard {
                            position: Position { row, column },
                            direction: Direction::Up,
                        }),
                        obstacles,
                    ),
                    _ => panic!(),
                },
            )
        })
        .fold(
            (None, HashSet::new()),
            |(guard, mut obstacles): (Option<Guard>, Obstacles),
             (maybe_guard, line_obstacles): (Option<Guard>, Obstacles)| {
                obstacles.extend(line_obstacles);
                (
                    match (guard, maybe_guard) {
                        (None, None) => None,
                        (Some(seen_guard), None) | (None, Some(seen_guard)) => Some(seen_guard),
                        (Some(_), Some(_)) => panic!(),
                    },
                    obstacles,
                )
            },
        );
    (
        maybe_guard.expect("He should definitely be there"),
        obstacles,
        Position::new(
            text.lines().count(),
            text.lines().next().unwrap().chars().count(),
        ),
    )
}

fn part_1_step_guard(guard: &Guard, obstacles: &Obstacles, grid_maxes: &Position) -> Option<Guard> {
    guard.step(grid_maxes).map(|next_pos| {
        if obstacles.contains(&next_pos) {
            Guard {
                direction: guard.direction.turn_right(),
                ..*guard
            }
        } else {
            Guard {
                position: next_pos,
                ..*guard
            }
        }
    })
}

fn part_1(text: &str) -> usize {
    let (mut guard, obstacles, max_pos) = parser_for_part_1(text);
    let mut sites_visited: HashMap<Position, HashSet<Direction>> =
        HashMap::from([(guard.position, HashSet::from([guard.direction]))]);
    while let Some(new_guard) = part_1_step_guard(&guard, &obstacles, &max_pos) {
        guard = new_guard;
        sites_visited
            .entry(guard.position)
            .or_default()
            .insert(guard.direction);
    }
    sites_visited.keys().len()
}

fn part_2(text: &str) -> usize {
    let (guard, obstacles, max_pos) = parser_for_part_1(text);
    text.lines()
        .enumerate()
        .par_bridge()
        .map(|(n, l)| {
            l.chars()
                .enumerate()
                .filter(|(m, c)| match c {
                    '.' => {
                        let mut sites_visited: HashMap<Position, HashSet<Direction>> =
                            HashMap::from([(guard.position, HashSet::from([guard.direction]))]);
                        let mut new_obstacles = obstacles.clone();
                        new_obstacles.insert(Position::new(n, *m));
                        let mut current_guard = guard;
                        while let Some(new_guard) =
                            part_1_step_guard(&current_guard, &new_obstacles, &max_pos)
                        {
                            current_guard = new_guard;
                            if sites_visited.contains_key(&current_guard.position)
                                && sites_visited
                                    .entry(current_guard.position)
                                    .or_default()
                                    .contains(&current_guard.direction)
                            {
                                return true;
                            }
                            sites_visited
                                .entry(current_guard.position)
                                .or_default()
                                .insert(current_guard.direction);
                        }
                        false
                    }
                    '#' | '^' => false,
                    _ => panic!(),
                })
                .count()
        })
        .sum()
}
#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn day_6_test_part_1_parser() {
        assert_eq!(
            parser_for_part_1(INPUT),
            (
                Guard {
                    position: Position::new(6, 4),
                    direction: Direction::Up
                },
                HashSet::from([
                    Position::new(0, 4),
                    Position::new(1, 9),
                    Position::new(3, 2),
                    Position::new(4, 7),
                    Position::new(6, 1),
                    Position::new(7, 8),
                    Position::new(8, 0),
                    Position::new(9, 6),
                ]),
                Position::new(10, 10),
            )
        );
    }

    #[test]
    fn day_6_guard() {
        let max = Position::new(2, 2);
        assert_eq!(
            Guard {
                position: Position::new(1, 1),
                direction: Direction::Up
            }
            .step(&max),
            Some(Position::new(0, 1))
        );
    }

    #[test]
    fn day_6_part_1() {
        assert_eq!(part_1(INPUT), 41);
    }

    #[test]
    fn day_6_part_2() {
        assert_eq!(part_2(INPUT), 6);
    }
}
