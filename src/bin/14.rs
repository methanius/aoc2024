use std::collections::HashMap;
use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::{preceded, separated_pair},
    IResult,
};
use std::{io, iter::successors};

fn main() {
    let text: String =
        std::fs::read_to_string("data/14.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    // println!("Part 2:\n{}", part_2(&text));
    part_2(&text);
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        str::parse(s)
    })(input)?;

    Ok((i, number))
}

fn parse_robot_line(line: &str) -> Robot {
    let (_rest, ((pos_x, pos_y), (vel_x, vel_y))) = separated_pair(
        separated_pair(preceded(tag("p="), parse_i64), tag(","), parse_i64),
        tag(" v="),
        separated_pair(parse_i64, tag(","), parse_i64),
    )(line)
    .expect("AOC hardcoded pattern");
    Robot {
        x: pos_x,
        y: pos_y,
        v_x: vel_x,
        v_y: vel_y,
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input.lines().map(parse_robot_line).collect()
}

fn step_robots(robots: &[Robot], grid_size: &(i64, i64)) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| Robot {
            x: (robot.x + robot.v_x).rem_euclid(grid_size.0),
            y: (robot.y + robot.v_y).rem_euclid(grid_size.1),
            ..*robot
        })
        .collect()
}

fn part_1_driver(input: &str, grid_size: &(i64, i64)) -> i64 {
    let mut robots = parse_robots(input);
    for _ in 0..100 {
        robots = step_robots(&robots, grid_size);
    }
    quadrant_count_robots(&robots, grid_size).iter().product()
}

fn quadrant_count_robots(robots: &[Robot], grid_size: &(i64, i64)) -> Vec<i64> {
    let demarcator = (grid_size.0 / 2, grid_size.1 / 2);
    robots
        .iter()
        .fold(vec![0, 0, 0, 0], |mut acc: Vec<i64>, robot| {
            if robot.x > demarcator.0 && robot.y < demarcator.1 {
                acc[0] += 1;
            } else if robot.x < demarcator.0 && robot.y < demarcator.1 {
                acc[1] += 1;
            } else if robot.x < demarcator.0 && robot.y > demarcator.1 {
                acc[2] += 1;
            } else if robot.x > demarcator.0 && robot.y > demarcator.1 {
                acc[3] += 1;
            };
            acc
        })
}
fn part_1(input: &str) -> i64 {
    part_1_driver(input, &(101, 103))
}

fn part_2(input: &str) {
    let robots = parse_robots(input);
    for (n, non_overlapping_robots) in successors(Some(robots), |robots: &Vec<Robot>| {
        Some(step_robots(robots, &(101, 103)))
    })
    .enumerate()
    .filter(|(_n, robots)| {
        let mut robot_positions: HashSet<(i64, i64)> = HashSet::new();
        robots
            .iter()
            .all(|robot| robot_positions.insert((robot.x, robot.y)))
    }) {
        let robot_positions_by_line = non_overlapping_robots.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<i64, HashSet<i64>>, robot| {
                acc.entry(robot.y).or_default().insert(robot.x);
                acc
            },
        );
        for y in 0..103 {
            let mut line: Vec<char> = vec!['.'; 101];
            if let Some(positions) = robot_positions_by_line.get(&y) {
                for pos in positions {
                    line[usize::try_from(*pos).unwrap()] = '#';
                }
            }
            let printable: String = line.iter().copied().collect();
            println!("{printable:?}");
        }
        println!("Found at {n:?}");
        println!("Write y and press enter to stop iterating.");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect(" What ARE you doing.");
        if &buffer[..] == "y\n" {
            return;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn day_14_test_part_1_driver() {
        let grid_size = (11, 7);
        let mut robots = parse_robots(INPUT);
        for _ in 0..100 {
            robots = step_robots(&robots, &grid_size);
        }
        assert_eq!(quadrant_count_robots(&robots, &grid_size), vec![3, 1, 4, 1]);
        assert_eq!(part_1_driver(INPUT, &grid_size), 12);
    }
}
