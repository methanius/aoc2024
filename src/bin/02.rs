use nom::branch::alt;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map_res, recognize};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

fn main() {
    let text: String =
        std::fs::read_to_string("data/02.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn parse_levels(line: &str) -> IResult<&str, Vec<i64>> {
    many1(alt((
        terminated(map_res(recognize(digit1), str::parse), space1),
        map_res(recognize(digit1), str::parse),
    )))(line)
}

fn part_1_criterion(line: &[i64]) -> bool {
    (line.iter().is_sorted() || line.iter().rev().is_sorted())
        && line.windows(2).all(|a| {
            let diff = a[0].abs_diff(a[1]);
            diff <= 3 && diff != 0
        })
}

fn part_1(text: &str) -> usize {
    text.lines()
        .map(|l| {
            let (_, parser_results) = parse_levels(l).unwrap();
            parser_results
        })
        .filter(|l| part_1_criterion(l))
        .count()
}

fn part_2(text: &str) -> usize {
    let levels: Vec<Vec<i64>> = text
        .lines()
        .map(|l| {
            let (_, level_ints) = parse_levels(l).unwrap();
            level_ints
        })
        .collect();
    let easy_count: usize = levels.iter().filter(|l| part_1_criterion(l)).count();
    let one_off_count: usize = levels
        .iter()
        .filter(|l| !part_1_criterion(l))
        .filter(|l| {
            l.iter().enumerate().any(|(n, _)| {
                part_1_criterion(
                    &l.iter()
                        .enumerate()
                        .filter(|(m, _)| n != *m)
                        .map(|(_, a)| *a)
                        .collect::<Vec<i64>>(),
                )
            })
        })
        .count();
    easy_count + one_off_count
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parser_test() {
        assert_eq!(
            INPUT
                .lines()
                .map(parse_levels)
                .map(|a| a.unwrap())
                .map(|(_, a)| a)
                .collect::<Vec<Vec<i64>>>(),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }

    #[test]
    fn day_2_part_1_test() {
        assert_eq!(part_1(INPUT), 2);
    }

    #[test]
    fn day_2_part_2_test() {
        assert_eq!(part_2(INPUT), 4);
    }
}
