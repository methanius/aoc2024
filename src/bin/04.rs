use std::iter::zip;

use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{map, rest};
use nom::multi::{many0, many_till};
use nom::sequence::terminated;
use nom::IResult;

fn main() {
    let text: String =
        std::fs::read_to_string("data/04.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn tag_xmas(text: &str) -> IResult<&str, &str> {
    tag("XMAS")(text)
}

fn junk_tag_xmas(text: &str) -> IResult<&str, &str> {
    map(many_till(anychar, tag_xmas), |(_junk, xmas)| xmas)(text)
}

fn many_xmases_from_junk(text: &str) -> IResult<&str, Vec<&str>> {
    many0(junk_tag_xmas)(text)
}

fn final_xmas_parser(text: &str) -> IResult<&str, Vec<&str>> {
    terminated(many_xmases_from_junk, rest)(text)
}

#[derive(Debug)]
struct Puzzle<'a> {
    data: &'a str,
}

impl<'a> Puzzle<'a> {
    const fn new(data: &'a str) -> Self {
        Self { data }
    }
    fn num_rows(&self) -> usize {
        self.data.lines().count()
    }
    fn num_cols(&self) -> usize {
        self.data.lines().next().unwrap().len()
    }
    fn lines(&self) -> Vec<String> {
        self.data.lines().map(|c| c.chars().collect()).collect()
    }
    fn cols(&self) -> Vec<String> {
        let acc: Vec<String> = vec![];
        self.data
            .lines()
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .fold(acc, |mut acc, (n, _)| {
                acc.push({
                    self.data
                        .lines()
                        .map(|l| l.chars().nth(n).unwrap())
                        .collect::<String>()
                });
                acc
            })
    }
    fn diags(&self) -> Vec<String> {
        let n_rows = self.num_rows();
        let n_cols = self.num_cols();
        let n_one_direction_diagonals = n_rows + n_cols - 1;
        let acc: Vec<String> = vec![];
        let mut up_right_diags = acc.clone();
        let mut down_right_diags = acc;
        let lines = self.lines();
        for nd in 0..=n_one_direction_diagonals {
            let coord_range = 0..nd;
            up_right_diags.push({
                let mut temp_up_right = String::new();
                for (r, c) in zip(coord_range.clone().rev(), coord_range.clone()) {
                    if r < n_rows && c < n_cols {
                        temp_up_right.push(lines[r].chars().nth(c).unwrap());
                    };
                }
                temp_up_right
            });
            down_right_diags.push({
                let mut temp_down_right = String::new();
                for (r, c) in zip(coord_range.clone().rev(), coord_range.clone()) {
                    if r < n_rows && c < n_cols {
                        temp_down_right.push(lines[r].chars().rev().nth(c).unwrap());
                    };
                }
                temp_down_right
            });
        }
        up_right_diags.append(&mut down_right_diags);
        up_right_diags
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect()
    }
}

fn count_xmases_and_sesamx(vs: &[String]) -> usize {
    vs.iter()
        .map(|s| {
            let (_, xmases) = final_xmas_parser(s).unwrap();
            let reverse_s = s.chars().rev().collect::<String>();
            let (_, sesamx) = final_xmas_parser(&reverse_s).unwrap();
            xmases.len() + sesamx.len()
        })
        .sum()
}

fn part_1(text: &str) -> usize {
    let p = Puzzle::new(text);
    count_xmases_and_sesamx(&p.cols())
        + count_xmases_and_sesamx(&p.lines())
        + count_xmases_and_sesamx(&p.diags())
}

#[derive(Debug, PartialEq)]
struct Block {
    data: (String, String, String),
}

impl Block {
    const fn new(data: (String, String, String)) -> Self {
        Self { data }
    }
    fn count_mas(&self) -> usize {
        if self.data.1.chars().nth(1).unwrap() == 'A' {
            match (
                self.data.0.chars().next().unwrap(),
                self.data.2.chars().next().unwrap(),
                self.data.0.chars().nth(2).unwrap(),
                self.data.2.chars().nth(2).unwrap(),
            ) {
                ('M', 'M', 'S', 'S')
                | ('M', 'S', 'M', 'S')
                | ('S', 'S', 'M', 'M')
                | ('S', 'M', 'S', 'M') => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
}

fn extract_3_3_blocks(text: &Puzzle) -> Vec<Block> {
    let mut res: Vec<Block> = vec![];
    let lines = text.lines();
    for r in 0..(lines.len() - 2) {
        for c in 0..(text.num_cols() - 2) {
            res.push(Block::new((
                lines[r][c..(c + 3)].to_string(),
                lines[r + 1][c..(c + 3)].to_string(),
                lines[r + 2][c..(c + 3)].to_string(),
            )));
        }
    }
    res
}

fn part_2(text: &str) -> usize {
    extract_3_3_blocks(&Puzzle::new(text))
        .iter()
        .map(Block::count_mas)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn day_4_test_recognize_xmas() {
        assert_eq!(tag_xmas("XMAS"), Ok(("", "XMAS")));
    }
    #[test]
    fn day_4_test_junk_before_xmas() {
        assert_eq!(junk_tag_xmas("ADFCASJFCJAOEWFCXMAS"), Ok(("", "XMAS")));
    }

    #[test]
    fn day_4_multi_junk_xmas() {
        assert_eq!(
            many_xmases_from_junk("SADFCASXMASASDFASXASDFXAXMASSDFOFASXMAS"),
            Ok(("", vec!["XMAS", "XMAS", "XMAS"]))
        );
    }

    #[test]
    fn day_4_multi_junk_xmas_end_junk() {
        assert_eq!(
            final_xmas_parser("SADFCASXMASASDFASXASDFXAXMASSDFOFASXMAS4124S"),
            Ok(("", vec!["XMAS", "XMAS", "XMAS"]))
        );
        assert_eq!(final_xmas_parser("ASCFKOACWOTQ#=CKQ="), Ok(("", vec![])));
    }

    #[test]
    fn day_4_puzzle_item_test() {
        let test: &str = "\
ABC
DEF
GHJ";
        let p = Puzzle::new(test);
        assert_eq!(
            p.lines(),
            vec![
                "ABC".chars().collect::<String>(),
                "DEF".chars().collect::<String>(),
                "GHJ".chars().collect::<String>(),
            ]
        );
        assert_eq!(
            p.cols(),
            vec![
                "ADG".chars().collect::<String>(),
                "BEH".chars().collect::<String>(),
                "CFJ".chars().collect::<String>()
            ]
        );
        assert_eq!(
            p.diags(),
            vec![
                "A".chars().collect::<String>(),
                "DB".chars().collect::<String>(),
                "GEC".chars().collect::<String>(),
                "HF".chars().collect::<String>(),
                "J".chars().collect::<String>(),
                "C".chars().collect::<String>(),
                "FB".chars().collect::<String>(),
                "JEA".chars().collect::<String>(),
                "HD".chars().collect::<String>(),
                "G".chars().collect::<String>(),
            ]
        );
        assert_eq!(
            extract_3_3_blocks(&p),
            vec![Block {
                data: ("ABC".to_owned(), "DEF".to_string(), "GHJ".to_string())
            }]
        );
    }

    #[test]
    fn day_4_part_1_test() {
        assert_eq!(part_1(INPUT), 18);
    }
}
