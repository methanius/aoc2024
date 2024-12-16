use nom::branch::alt;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::{bytes::complete::tag, IResult};
use std::collections::{HashMap, HashSet};

fn main() {
    let text: String =
        std::fs::read_to_string("data/05.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

fn separate_rule_block_and_update_block(text: &str) -> (&str, &str) {
    text.split_once("\n\n")
        .expect("Otherwise the static input format was misunderstood")
}

fn parse_digit(digit_str: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(digit_str)
}

fn rule_parser(rule_text: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_digit, tag("|"), parse_digit)(rule_text)
}

fn extract_rules(rule_block: &str) -> HashMap<u64, HashSet<u64>> {
    rule_block
        .lines()
        .map(rule_parser)
        .map(|parsed| parsed.expect("AOC hardcoded format").1)
        .fold(HashMap::new(), |mut acc, (before, after): (u64, u64)| {
            let val = acc.entry(before).or_default();
            val.insert(after);
            acc
        })
}

fn update_parser(update_line: &str) -> IResult<&str, Vec<u64>> {
    many1(alt((terminated(parse_digit, tag(",")), parse_digit)))(update_line)
}

fn extract_updates(update_block: &str) -> Vec<Vec<u64>> {
    update_block
        .lines()
        .map(update_parser)
        .map(|parsed| parsed.expect("AOC Hardcoded format").1)
        .collect()
}

fn part_1_filter(vec: &[u64], part_1_rules: &HashMap<u64, HashSet<u64>>) -> bool {
    vec.iter().enumerate().rev().any(|(n, elm)| {
        vec.iter().take(n).any(|prev_elm| {
            part_1_rules.contains_key(elm) && part_1_rules.get(elm).unwrap().contains(prev_elm)
        })
    })
}

fn part_1(input: &str) -> u64 {
    let (rule_block, update_block) = separate_rule_block_and_update_block(input);
    let rules = extract_rules(rule_block);
    let updates = extract_updates(update_block);
    updates
        .iter()
        .filter_map(|update| {
            if part_1_filter(update, &rules) {
                None
            } else {
                Some(*update.get(update.len() / 2).unwrap())
            }
        })
        .sum()
}

fn sort_vec(vec: &[u64], rules: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    for (n, key) in vec.iter().enumerate().rev() {
        for (m, prev_elm) in vec.iter().take(n).enumerate() {
            if rules.contains_key(key) && rules.get(key).unwrap().contains(prev_elm) {
                let mut new_vec: Vec<u64> = vec.to_owned();
                let moved_elm = new_vec.remove(n);
                new_vec.insert(m, moved_elm);
                return sort_vec(&new_vec, rules);
            }
        }
    }
    vec.to_vec()
}
fn part_2(input: &str) -> u64 {
    let (rule_block, update_block) = separate_rule_block_and_update_block(input);
    let rules = extract_rules(rule_block);
    let updates = extract_updates(update_block);
    updates
        .iter()
        .filter(|vec| part_1_filter(vec, &rules))
        .map(|update| sort_vec(update, &rules))
        .map(|sorted_vec| *sorted_vec.get(sorted_vec.len() / 2).unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn day_5_split_once() {
        assert_eq!(
            separate_rule_block_and_update_block(INPUT),
            (
                "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
                "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            )
        );
    }

    #[test]
    fn process_rule() {
        assert_eq!(rule_parser("12|32"), Ok(("", (12, 32))));
        assert_eq!(
            extract_rules(separate_rule_block_and_update_block(INPUT).0),
            HashMap::from([
                (61, HashSet::from([13, 53, 29])),
                (53, HashSet::from([29, 13])),
                (75, HashSet::from([61, 53, 29, 47, 13])),
                (97, HashSet::from([29, 53, 47, 75, 13, 61])),
                (29, HashSet::from([13])),
                (47, HashSet::from([13, 61, 53, 29]))
            ])
        );
    }

    #[test]
    fn process_updates() {
        assert_eq!(
            extract_updates(separate_rule_block_and_update_block(INPUT).1),
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        );
    }

    #[test]
    fn get_middle_of_vector() {
        let test: Vec<u64> = vec![75, 47, 61, 53, 29];
        assert_eq!(*test.get(test.len() / 2).unwrap(), 61u64);
    }

    #[test]
    fn day_05_test_part_1() {
        assert_eq!(part_1(INPUT), 143);
    }

    #[test]
    fn day_5_sort_test() {
        let rules = extract_rules(separate_rule_block_and_update_block(INPUT).0);
        assert_eq!(sort_vec(&[61, 13, 29], &rules), vec![61, 29, 13]);
        assert_eq!(
            sort_vec(&[75, 97, 47, 61, 54], &rules),
            vec![97, 75, 47, 61, 54]
        );
        assert_eq!(
            sort_vec(&[97, 13, 75, 29, 47], &rules),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn day_05_test_part_2() {
        assert_eq!(part_2(INPUT), 123);
    }
}
