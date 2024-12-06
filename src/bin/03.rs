use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map;
use nom::combinator::{map_res, rest};
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;

fn main() {
    let text: String =
        std::fs::read_to_string("data/03.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn mul_parser(line: &str) -> IResult<&str, (u64, u64)> {
    delimited(
        tag(r"mul("),
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u64>()),
            tag(","),
            map_res(digit1, |s: &str| s.parse::<u64>()),
        ),
        tag(r")"),
    )(line)
}
fn parse_single_with_front_mul(line: &str) -> IResult<&str, (u64, u64)> {
    many_till(anychar, mul_parser)(line).map(|(rem, (_chars, tup))| (rem, tup))
}

fn parse_many_muls(line: &str) -> IResult<&str, Vec<(u64, u64)>> {
    terminated(many0(parse_single_with_front_mul), rest)(line)
}

fn part_1(text: &str) -> u64 {
    text.lines()
        .map(parse_many_muls)
        .map(|res| {
            let (_rem, v) = res.unwrap();
            v.iter().map(|(a, b)| a * b).sum::<u64>()
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Expression {
    Mul(u64, u64),
    Cond(bool),
}

fn cond_parser(text: &str) -> IResult<&str, Expression> {
    map(alt((tag(r"don't()"), tag(r"do()"))), |s: &str| {
        Expression::Cond(matches!(s, r"do()"))
    })(text)
}

fn mul_expression_parser(text: &str) -> IResult<&str, Expression> {
    map(mul_parser, |(a, b)| Expression::Mul(a, b))(text)
}

fn parse_many_expressions(text: &str) -> IResult<&str, Vec<Expression>> {
    let expression_parser = alt((cond_parser, mul_expression_parser));
    let remove_lead_till_expression = map(
        many_till(anychar, expression_parser),
        |(_chars, expression)| expression,
    );
    terminated(many0(remove_lead_till_expression), rest)(text)
}

fn part_2(text: &str) -> u64 {
    let (_rest, v) = parse_many_expressions(text).unwrap();
    v.iter()
        .fold((true, 0), |(b, e): (bool, u64), item: &Expression| match item {
            Expression::Cond(t) => (*t, e),
            Expression::Mul(x, y) => {
                if b {
                    (b, e + x * y)
                } else {
                    (b, e)
                }
            }
        }).1
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn get_tuples() {
        assert_eq!(
            parse_many_muls(INPUT),
            Ok(("", vec![(2, 4), (5, 5), (11, 8), (8, 5)]))
        );
    }

    #[test]
    fn mul_parser_test() {
        assert_eq!(mul_parser("mul(23,24)"), Ok(("", (23u64, 24u64))));
    }

    #[test]
    fn preceded_mul_parser_test() {
        assert_eq!(
            parse_single_with_front_mul("1234c0291c4i019muc12904mnulc12940mul(43,43)"),
            Ok(("", (43, 43)))
        );
    }

    #[test]
    fn day_3_part_1_test() {
        assert_eq!(part_1(INPUT), 161);
    }

    #[test]
    fn day_3_test_do_parser() {
        assert_eq!(cond_parser(r"do()"), Ok(("", Expression::Cond(true))));
        assert_eq!(cond_parser(r"don't()"), Ok(("", Expression::Cond(false))));
    }

    #[test]
    fn day_3_mul_expression_parser() {
        assert_eq!(
            mul_expression_parser("mul(321,302)"),
            Ok(("", Expression::Mul(321, 302)))
        );
    }

    #[test]
    fn day_3_part_2() {
        assert_eq!(part_2(INPUT), 48);
    }
}
