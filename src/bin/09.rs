use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

use std::{collections::VecDeque, ops::Div};

fn main() {
    let text: String =
        std::fs::read_to_string("data/09.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    // println!("Part 2:\n{}", part_2(&text));
}

#[derive(Debug, PartialEq)]
enum BlockType {
    File { index: u64 },
    Empty,
}

#[derive(Debug, PartialEq)]
struct Block {
    variant: BlockType,
    count: u64,
}

fn u64_parser(c: &str) -> IResult<&str, u64> {
    map_res(take(1u64), str::parse::<u64>)(c)
}

fn first_parse_part_1(input: &str) -> VecDeque<Block> {
    many1(u64_parser)(input)
        .expect("Hardcoded input")
        .1
        .iter()
        .enumerate()
        .fold(VecDeque::new(), |mut acc, (n, digit)| {
            if n % 2 == 0 {
                acc.push_back(Block {
                    variant: BlockType::File {
                        index: n
                            .div(2)
                            .try_into()
                            .expect("Hard expect small numbers that fit"),
                    },
                    count: *digit,
                });
            } else {
                acc.push_back(Block {
                    variant: BlockType::Empty,
                    count: *digit,
                });
            }
            acc
        })
}

fn compact_scattered_disk(mut blocks: VecDeque<Block>) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    if blocks.back().unwrap().variant == BlockType::Empty {
        blocks.pop_back();
    };
    blocks.retain(|b| b.count > 0);
    while !blocks.is_empty() {
        match blocks
            .front_mut()
            .expect("We are inside a non-empty deque while loop!")
            .variant
        {
            BlockType::File { index } => {
                res.push(index);
            }
            BlockType::Empty => {
                match blocks.back() {
                    Some(block) => {
                        if block.variant == BlockType::Empty {
                            blocks.pop_back();
                        }
                        match blocks.back_mut() {
                            Some(Block { count, variant }) => {
                                match variant {
                                    BlockType::File { index } => res.push(*index),
                                    BlockType::Empty => panic!(),
                                };
                                if *count == 1 {
                                    blocks.pop_back();
                                } else {
                                    *count -= 1;
                                }
                            }
                            None => return res,
                        }
                    }
                    None => return res,
                };
            }
        }
        match blocks.get_mut(0) {
            Some(Block { count: 1, .. }) => {
                blocks.pop_front();
            }
            Some(Block { count, .. }) => *count -= 1,
            None => return res,
        };
    }
    res
}

fn part_1(input: &str) -> u64 {
    compact_scattered_disk(first_parse_part_1(input))
        .iter()
        .enumerate()
        .map(|(n, e): (usize, &u64)| (n as u64) * e)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
2333133121414131402";

    #[test]
    fn test_parse_part_1() {
        assert_eq!(
            first_parse_part_1(INPUT),
            vec![
                Block {
                    variant: BlockType::File { index: 0 },
                    count: 2
                },
                Block {
                    variant: BlockType::Empty,
                    count: 3
                },
                Block {
                    variant: BlockType::File { index: 1 },
                    count: 3
                },
                Block {
                    variant: BlockType::Empty,
                    count: 3
                },
                Block {
                    variant: BlockType::File { index: 2 },
                    count: 1
                },
                Block {
                    variant: BlockType::Empty,
                    count: 3
                },
                Block {
                    variant: BlockType::File { index: 3 },
                    count: 3
                },
                Block {
                    variant: BlockType::Empty,
                    count: 1
                },
                Block {
                    variant: BlockType::File { index: 4 },
                    count: 2
                },
                Block {
                    variant: BlockType::Empty,
                    count: 1
                },
                Block {
                    variant: BlockType::File { index: 5 },
                    count: 4
                },
                Block {
                    variant: BlockType::Empty,
                    count: 1
                },
                Block {
                    variant: BlockType::File { index: 6 },
                    count: 4
                },
                Block {
                    variant: BlockType::Empty,
                    count: 1
                },
                Block {
                    variant: BlockType::File { index: 7 },
                    count: 3
                },
                Block {
                    variant: BlockType::Empty,
                    count: 1
                },
                Block {
                    variant: BlockType::File { index: 8 },
                    count: 4
                },
                Block {
                    variant: BlockType::Empty,
                    count: 0
                },
                Block {
                    variant: BlockType::File { index: 9 },
                    count: 2
                },
            ]
        );
    }

    #[test]
    fn day_9_test_compression() {
        assert_eq!(
            compact_scattered_disk(first_parse_part_1(INPUT)),
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
    }

    #[test]
    fn day_9_part_1() {
        assert_eq!(part_1(INPUT), 1928);
    }
}
