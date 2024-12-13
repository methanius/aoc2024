use rayon::prelude::*;

use itertools::{repeat_n, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

fn main() {
    let text: String =
        std::fs::read_to_string("data/07.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    // println!("Part 2:\n{}", part_2(&text));
}

fn u64_parser(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parser(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(u64_parser, tag(":"), many1(preceded(tag(" "), u64_parser)))(input)
}

#[derive(Clone)]
enum Ops {
    Mul,
    Add,
}

fn part_1_single_line(line: &str) -> Option<u64> {
    let (_rest, (target, nums)): (&str, (u64, Vec<u64>)) =
        parser(line).expect("AOC hardcoded format");
    if repeat_n([Ops::Mul, Ops::Add], nums.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            nums.iter()
                .skip(1)
                .enumerate()
                .fold(*nums.first().unwrap(), |acc, (n, elm)| {
                    let current_op = ops.get(n).expect("These lengths must match");
                    match current_op {
                        Ops::Mul => acc * elm,
                        Ops::Add => acc + elm,
                    }
                })
                == target
        })
    {
        Some(target)
    } else {
        None
    }
}

fn part_1(input: &str) -> u64 {
    input.par_lines().filter_map(part_1_single_line).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn day_7_test_line_for_part_1() {
        assert_eq!(part_1_single_line(INPUT.lines().next().unwrap()), Some(190));
    }

    #[test]
    fn day_7_part_1() {
        assert_eq!(part_1(INPUT), 3749);
    }
}
