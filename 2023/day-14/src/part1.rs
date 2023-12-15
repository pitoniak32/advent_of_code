use std::fmt::Display;

use anyhow::Result;
use util::transpose_matrix;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub enum Tile {
    O,
    R,
    S,
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::O => 'O',
            Tile::S => '.',
            Tile::R => '#',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::O,
            '.' => Tile::S,
            '#' => Tile::R,
            v => unreachable!("{} is not a vaild tile", v),
        }
    }
}

// 113456 = just right
pub fn process(input: &str) -> Result<String> {
    let platform = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<_>>>();

    let mut transposed = transpose_matrix(&platform);

    let sorted = transposed
        .iter_mut()
        .map(|row| {
            row.split_mut(|c| c == &Tile::R)
                .for_each(|section| section.sort());
            row.clone()
        })
        .collect::<Vec<_>>();

    let transposed = &transpose_matrix(&sorted);

    let len = sorted.len() - 1;
    let load = transposed.iter().enumerate().fold(0, |mut acc, (i, row)| {
        let stones = row.iter().filter(|c| c == &&Tile::O).count();
        acc += (len - i + 1) * stones;
        acc
    });

    Ok(load.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(process(input)?, "136");
        Ok(())
    }
}
