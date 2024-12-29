use aoc2024::grid::Grid;
use aoc2024::position::Position;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::successors;
type Day15Grid = Grid<GridValue>;

fn main() {
    let text: String =
        std::fs::read_to_string("data/15.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
}

struct Robot {
    position: Position,
}

#[derive(Debug)]
enum RobotInstruction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum GridValue {
    Wall,
    Box,
    Empty,
    Robot,
}

impl GridValue {
    /// Returns `true` if the grid value is [`Robot`].
    ///
    /// [`Robot`]: GridValue::Robot
    #[must_use]
    const fn is_robot(self) -> bool {
        matches!(self, Self::Robot)
    }

    /// Returns `true` if the grid value is [`Box`].
    ///
    /// [`Box`]: GridValue::Box
    #[must_use]
    const fn is_box(self) -> bool {
        matches!(self, Self::Box)
    }
}

fn part_1_parser(input: &str) -> (Robot, Day15Grid, VecDeque<RobotInstruction>) {
    let (grid_block, instructions_block) = input
        .split_once("\n\n")
        .expect("Otherwise I did a copy paste error");
    let grid = parse_grid(grid_block);
    let robot = Robot {
        position: grid
            .to_indexed_iterator()
            .find_map(|(pos, val)| if val.is_robot() { Some(pos) } else { None })
            .expect("There should be a single robot!"),
    };
    let instructions = instructions_block
        .chars()
        .filter_map(|c| match c {
            '^' => Some(RobotInstruction::Up),
            '>' => Some(RobotInstruction::Right),
            '<' => Some(RobotInstruction::Left),
            'v' => Some(RobotInstruction::Down),
            '\n' => None,
            _ => panic!(),
        })
        .collect();
    (robot, grid, instructions)
}

fn parse_grid(grid_block: &str) -> Grid<GridValue> {
    Grid::parse_grid(grid_block, |c| match c {
        '#' => GridValue::Wall,
        '.' => GridValue::Empty,
        '@' => GridValue::Robot,
        'O' => GridValue::Box,
        _ => panic!(),
    })
}

fn execute_robot_instruction(
    robot: &mut Robot,
    grid: &mut Day15Grid,
    instruction: &RobotInstruction,
) {
    // println!("{instruction:?}");
    let step_fun = match instruction {
        RobotInstruction::Up => |pos: &Position| -> Option<Position> {
            match pos.row {
                0 => None,
                _ => Some(Position::new(pos.row - 1, pos.col)),
            }
        },
        RobotInstruction::Left => |pos: &Position| -> Option<Position> {
            match pos.col {
                0 => None,
                _ => Some(Position::new(pos.row, pos.col - 1)),
            }
        },
        RobotInstruction::Down => {
            |pos: &Position| -> Option<Position> { Some(Position::new(pos.row + 1, pos.col)) }
        }
        RobotInstruction::Right => {
            |pos: &Position| -> Option<Position> { Some(Position::new(pos.row, pos.col + 1)) }
        }
    };
    let (positions_in_front, mut values_in_front) = successors(Some(robot.position), &step_fun)
        .map(|pos| {
            (
                pos,
                grid.get(&pos)
                    .expect("We shouldn't be able to leave a walled grid"),
            )
        })
        .take_while_inclusive(|(_pos, val)| val.is_robot() || val.is_box())
        .fold(
            (VecDeque::new(), VecDeque::new()),
            |(mut positions, mut values): (VecDeque<Position>, VecDeque<GridValue>), (pos, val)| {
                positions.push_back(pos);
                values.push_back(*val);
                (positions, values)
            },
        );
    if let Some(end) = values_in_front.back() {
        match end {
            GridValue::Wall => (),
            GridValue::Empty => {
                values_in_front.pop_back();
                values_in_front.push_front(GridValue::Empty);
                // println!("{grid:?}");
                robot.position = step_fun(&robot.position).expect("Logic should keep us inside");
                positions_in_front
                    .iter()
                    .zip(values_in_front)
                    .for_each(|(pos, val)| (*grid).set(pos, val));
            }
            GridValue::Box | GridValue::Robot => panic!("This should never happen!"),
        }
    } else {
        panic!("No end on values_in_front vector?! That should never happen.")
    }
}

fn part_1(input: &str) -> u64 {
    let (mut robot, mut grid, instructions) = part_1_parser(input);
    for instruction in instructions {
        execute_robot_instruction(&mut robot, &mut grid, &instruction);
    }
    grid.to_indexed_iterator()
        .filter_map(|(pos, val)| if val.is_box() { Some(pos) } else { None })
        .fold(0, |acc, pos| acc + 100 * pos.row + pos.col)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn day_15_test_parsing_and_moves() {
        let (mut robot, mut grid, instructions) = part_1_parser(INPUT);
        for instruction in instructions {
            execute_robot_instruction(&mut robot, &mut grid, &instruction);
        }
        assert_eq!(
            grid,
            parse_grid(
                "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########"
            )
        );
    }
}
