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

    /// .
    ///
    /// # Panics
    ///
    /// Panics if usize cannot be cast from input u64s
    #[must_use]
    pub fn get(&self, Position { row, col }: &Position) -> Option<&T> {
        self.data
            .get(usize::try_from(*row).expect("The Advent of Code grid is not that big!"))
            .map(|c| {
                c.get(usize::try_from(*col).expect("The AoC puzzle grid is not that large!"))
            })?
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if usize cannot be cast from input u64s
    pub fn set(&mut self, Position { row, col }: &Position, new_val: T) {
        *self
            .data
            .get_mut(
                usize::try_from(*row)
                    .expect("Usize to u64 should be fine for problems of regular AOC size"),
            )
            .expect("AOC grids are usually small")
            .get_mut(
                usize::try_from(*col)
                    .expect("Usize from u64 is fine for small problems, like AOC usually are"),
            )
            .expect("AOC grids are usually small") = new_val;
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

    /// Returns the shape of this [`Grid<T>`].
    ///
    /// # Panics
    ///
    /// Panics if usize can not be converted to u64 on this platform.
    #[must_use]
    pub fn shape(&self) -> Position {
        Position::new(
            self.data
                .len()
                .try_into()
                .expect("The AOC grids are small!"),
            self.data
                .first()
                .expect("First grid vector should not be empty!")
                .len()
                .try_into()
                .expect("The AOC grids are small!"),
        )
    }
}
