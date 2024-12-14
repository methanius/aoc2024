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
    println!("Part 2:\n{}", part_2(&text));
}

fn u64_parser(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parser(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(u64_parser, tag(":"), many1(preceded(tag(" "), u64_parser)))(input)
}

trait BinOp {
    fn apply_op(&self, lhs: u64, rhs: u64) -> u64;
}

#[derive(PartialEq, Eq, Hash)]
struct Mul;

impl BinOp for Mul {
    fn apply_op(&self, lhs: u64, rhs: u64) -> u64 {
        lhs * rhs
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Add;

impl BinOp for Add {
    fn apply_op(&self, lhs: u64, rhs: u64) -> u64 {
        lhs + rhs
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Append;

impl BinOp for Append {
    fn apply_op(&self, lhs: u64, rhs: u64) -> u64 {
        lhs * 10u64.pow(rhs.to_string().len().try_into().unwrap()) + rhs
    }
}

fn part_1_single_line(line: &str, ops_set: &[&dyn BinOp]) -> Option<u64> {
    let (_rest, (target, nums)): (&str, (u64, Vec<u64>)) =
        parser(line).expect("AOC hardcoded format");
    if repeat_n(ops_set.iter(), nums.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            nums.iter()
                .skip(1)
                .enumerate()
                .fold(*nums.first().unwrap(), |acc, (n, elm)| {
                    let current_op = ops.get(n).expect("These lengths must match");
                    current_op.apply_op(acc, *elm)
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
    let part_1_binops: Vec<&dyn BinOp> = vec![&Add, &Mul];
    input
        .lines()
        .filter_map(|l| part_1_single_line(l, &part_1_binops))
        .sum()
}

fn part_2(input: &str) -> u64 {
    let part_2_binops: Vec<&dyn BinOp> = vec![&Add, &Mul, &Append];
    input
        .lines()
        .filter_map(|l| part_1_single_line(l, &part_2_binops))
        .sum()
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
        let set: Vec<&dyn BinOp> = vec![&Add, &Mul];
        assert_eq!(
            part_1_single_line(INPUT.lines().next().unwrap(), &set),
            Some(190)
        );
    }

    #[test]
    fn day_7_part_1() {
        assert_eq!(part_1(INPUT), 3749);
    }

    #[test]
    fn day_7_test_append_apply() {
        assert_eq!(Append.apply_op(120, 34), 12034);
        assert_eq!(Append.apply_op(12, 34), 1234);
        assert_eq!(Append.apply_op(12, 340), 12340);
    }
}
