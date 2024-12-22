use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let text: String =
        std::fs::read_to_string("data/13.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn digit_parser(digit_str: &str) -> IResult<&str, f64> {
    map_res(digit1, str::parse)(digit_str)
}

fn parse_line(line: &str) -> (f64, f64) {
    let (_rest, nums) = separated_pair(
        preceded(
            alt((tag("Button A: X+"), tag("Button B: X+"), tag("Prize: X="))),
            digit_parser,
        ),
        alt((tag(", Y+"), tag(", Y="))),
        digit_parser,
    )(line)
    .expect("Hardcoded AOC pattern");
    nums
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn part_1_single_block(block: &str) -> Option<u64> {
    let nums: Vec<(f64, f64)> = block.lines().map(parse_line).collect();
    // Matrix inversion by hand
    let a = nums[0].0;
    let b = nums[1].0;
    let c = nums[0].1;
    let d = nums[1].1;
    let target_x = nums[2].0;
    let target_y = nums[2].1;
    let inverse_determinant = 1.0 / a.mul_add(d, -(b * c));
    let n_x = inverse_determinant * d.mul_add(target_x, -(b * target_y));
    let n_y = inverse_determinant * (-c).mul_add(target_x, a * target_y);
    let int_discrimate = |n: f64| (n - n.round()).abs() < 0.00001;
    if int_discrimate(n_x) && int_discrimate(n_y) {
        Some(3 * (n_x.round() as u64) + (n_y.round() as u64))
    } else {
        None
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn part_2_single_block(block: &str) -> Option<u64> {
    let nums: Vec<(f64, f64)> = block.lines().map(parse_line).collect();
    // Matrix inversion by hand
    let a = nums[0].0 as i128;
    let b = nums[1].0 as i128;
    let c = nums[0].1 as i128;
    let d = nums[1].1 as i128;
    let target_x = nums[2].0 as i128 + 10_000_000_000_000;
    let target_y = nums[2].1 as i128 + 10_000_000_000_000;
    let determinant = a * d - b * c;
    let n_a = (d * target_x - b * target_y) / determinant;
    let n_b = (-c * target_x + a * target_y) / determinant;
    if n_a >= 0 && n_b >= 0 && n_a * a + n_b * b == target_x && n_a * c + n_b * d == target_y {
        Some(3 * (n_a as u64) + (n_b as u64))
    } else {
        // println!("Loss.");
        None
    }
}
fn part_1(input: &str) -> u64 {
    input.split("\n\n").filter_map(part_1_single_block).sum()
}

fn part_2(input: &str) -> u64 {
    input.split("\n\n").filter_map(part_2_single_block).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn day_13_test_part_1() {
        assert_eq!(part_1(INPUT), 480);
    }
}
