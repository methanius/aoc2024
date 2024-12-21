use crate::position::Position;
#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T: Eq> Grid<T> {
    #[must_use]
    pub const fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get(&self, position: &Position) -> Option<&T> {
        self.data
            .get(usize::try_from(position.row).expect("The Advent of Code grid is not that big!"))
            .map(|c| {
                c.get(
                    usize::try_from(position.col).expect("The AoC puzzle grid is not that large!"),
                )
            })?
    }

    pub fn to_indexed_iterator(&self) -> impl Iterator<Item = (Position, &T)> {
        self.data.iter().enumerate().flat_map(|(row, row_vector)| {
            row_vector
                .iter()
                .enumerate()
                .map(move |(col, digit)| (Position::new(row as u64, col as u64), digit))
        })
    }
    pub fn parse_grid<U>(value: &str, str_caster: U) -> Self
    where
        U: Fn(char) -> T,
    {
        Self {
            data: value
                .lines()
                .map(|l| l.chars().map(&str_caster).collect())
                .collect(),
        }
    }

    #[must_use]
    pub fn get_direct_position_value_neighbours(&self, pos: &Position) -> Vec<(Position, &T)> {
        pos.direct_neighbours()
            .into_iter()
            .filter_map(|pos| self.get(&pos).map(|value| (pos, value)))
            .collect()
    }
}
