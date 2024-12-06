use diagonal::{diagonal_pos_neg, diagonal_pos_pos, straight_x, straight_y};
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{map, rest};
use nom::multi::{many0, many_till};
use nom::sequence::terminated;
use nom::IResult;

fn main() {
    let text: String =
        std::fs::read_to_string("data/04.txt").expect("Couldn't read file at hard-coded path!");
    // println!("Part 1:\n{}", part_1(&text));
    // println!("Part 2:\n{}", part_2(&text));
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
    terminated(many_xmases_from_junk,rest)(text)
}

fn part_2(text:&str) -> usize {
    let mat: Vec<Vec<char>> = text.lines().map(|s| s.chars().collect()).collect();
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
        assert_eq!(many_xmases_from_junk("SADFCASXMASASDFASXASDFXAXMASSDFOFASXMAS"), Ok(("", vec!["XMAS", "XMAS", "XMAS"])));
    }

    #[test]
    fn day_4_multi_junk_xmas_end_junk() {
        assert_eq!(many_xmases_with_trail("SADFCASXMASASDFASXASDFXAXMASSDFOFASXMAS4124S"), Ok(("", vec!["XMAS", "XMAS", "XMAS"])));
        assert_eq!(many_xmases_with_trail("ASCFKOACWOTQ#=CKQ="), Ok(("", vec![])));
    }
}
