fn main() {
    // let text: String =
    //     std::fs::read_to_string("data/10.txt").expect("Couldn't read file at hard-coded path!");
    // println!("Part 1:\n{}", part_1(&text));
    // println!("Part 2:\n{}", part_2(&text));
}

#[derive(Debug, PartialEq)]
struct Grid {
    data: Vec<Vec<u64>>,
}

impl Grid {
    // fn get(&self, row: u64, col: u64) -> Option<u64> {
    //     match (row.try_into::<usize>(), col.try_into::<usize>()) {
    //         (Ok(_), Ok(_)) => todo!(),
    //         (Ok(_), Err(_))  | (Err(_), Ok(_))  | (Err(_), Err(_)) => None,
    //     }
}

fn parse_grid(value: &str) -> Grid {
    Grid {
        data: value
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        u64::from(
                            c.to_digit(10)
                                .expect("Hardcoded input should all be integers"),
                        )
                    })
                    .collect()
            })
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn day_10_test_grid_parser() {
        assert_eq!(
            parse_grid(INPUT),
            Grid {
                data: vec![
                    vec![8, 9, 0, 1, 0, 1, 2, 3,],
                    vec![7, 8, 1, 2, 1, 8, 7, 4,],
                    vec![8, 7, 4, 3, 0, 9, 6, 5,],
                    vec![9, 6, 5, 4, 9, 8, 7, 4,],
                    vec![4, 5, 6, 7, 8, 9, 0, 3,],
                    vec![3, 2, 0, 1, 9, 0, 1, 2,],
                    vec![0, 1, 3, 2, 9, 8, 0, 1,],
                    vec![1, 0, 4, 5, 6, 7, 3, 2,],
                ],
                rows: 8,
                cols: 8,
            }
        );
    }
}
