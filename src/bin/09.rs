use nom::bytes::complete::take;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

use std::{collections::VecDeque, iter::repeat_n, ops::Div};

fn main() {
    let text: String =
        std::fs::read_to_string("data/09.txt").expect("Couldn't read file at hard-coded path!");
    println!("Part 1:\n{}", part_1(&text));
    println!("Part 2:\n{}", part_2(&text));
}

#[derive(Debug, PartialEq)]
enum BlockType {
    File { index: u64 },
    Empty,
}

impl BlockType {
    /// Returns `true` if the block type is [`File`].
    ///
    /// [`File`]: BlockType::File
    #[must_use]
    const fn is_file(&self) -> bool {
        matches!(self, Self::File { .. })
    }

    /// Returns `true` if the block type is [`Empty`].
    ///
    /// [`Empty`]: BlockType::Empty
    #[must_use]
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
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

fn try_move_file_to_space(disc: VecDeque<Block>) -> VecDeque<Block> {
    for (n, file) in disc
        .iter()
        .enumerate()
        .rev()
        .filter(|(_n, b)| b.variant.is_file())
    {
        if let Some((m, _empty_block)) = disc
            .iter()
            .enumerate()
            .find(|(m, empty)| *m < n && empty.variant.is_empty() && empty.count >= file.count)
        {
            let mut new_disc = disc;
            let file_block: Block = new_disc.remove(n).expect("We just found this file!");
            new_disc.insert(
                n,
                Block {
                    variant: BlockType::Empty,
                    count: file_block.count,
                },
            );
            new_disc.insert(m, file_block);
            new_disc
                .get_mut(m + 1)
                .expect("We just pushed an element to this location by an insert!")
                .count -= new_disc.get(m).expect("We just pushed this value!").count;
            return try_move_file_to_space(new_disc);
        }
    }
    disc
}

#[derive(Debug, Clone, PartialEq)]
enum Part2Index {
    Index(u64),
    Empty,
}

fn part_2_flatten_disc_for_hash(disc: &VecDeque<Block>) -> Vec<Part2Index> {
    disc.iter()
        .fold(Vec::new(), |mut acc, block| match block.variant {
            BlockType::File { index } => {
                acc.extend(repeat_n(
                    Part2Index::Index(index),
                    block
                        .count
                        .try_into()
                        .expect("We are scanning a small compressed file."),
                ));
                acc
            }
            BlockType::Empty => {
                acc.extend(repeat_n(
                    Part2Index::Empty,
                    block
                        .count
                        .try_into()
                        .expect("We are scanning a small compressed file."),
                ));
                acc
            }
        })
}

fn part_2(input: &str) -> u64 {
    let disc = first_parse_part_1(input);
    let moved_disc = try_move_file_to_space(disc);
    part_2_flatten_disc_for_hash(&moved_disc)
        .iter()
        .enumerate()
        .filter_map(|(n, part_2_idx)| match part_2_idx {
            Part2Index::Index(id) => Some(*id * (n as u64)),
            Part2Index::Empty => None,
        })
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

    #[test]
    fn day_9_part_2_move_files_test() {
        let disc = try_move_file_to_space(first_parse_part_1(INPUT));
        let compact = part_2_flatten_disc_for_hash(&disc);
        assert_eq!(
            compact,
            vec![
                Part2Index::Index(0),
                Part2Index::Index(0),
                Part2Index::Index(9),
                Part2Index::Index(9),
                Part2Index::Index(2),
                Part2Index::Index(1),
                Part2Index::Index(1),
                Part2Index::Index(1),
                Part2Index::Index(7),
                Part2Index::Index(7),
                Part2Index::Index(7),
                Part2Index::Empty,
                Part2Index::Index(4),
                Part2Index::Index(4),
                Part2Index::Empty,
                Part2Index::Index(3),
                Part2Index::Index(3),
                Part2Index::Index(3),
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Index(5),
                Part2Index::Index(5),
                Part2Index::Index(5),
                Part2Index::Index(5),
                Part2Index::Empty,
                Part2Index::Index(6),
                Part2Index::Index(6),
                Part2Index::Index(6),
                Part2Index::Index(6),
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Empty,
                Part2Index::Index(8),
                Part2Index::Index(8),
                Part2Index::Index(8),
                Part2Index::Index(8),
                Part2Index::Empty,
                Part2Index::Empty,
            ]
        );
    }

    #[test]
    fn day_9_part_2() {
        assert_eq!(part_2(INPUT), 2858);
    }
}
