use std::fmt::Display;

use anyhow::Result;
use util::{transpose_matrix};

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

pub fn process(input: &str) -> Result<String> {
    let cycles = 1_000_000_000;

    let load = run_cycle(input, cycles);

    Ok(load)
}

pub fn run_cycle(input: &str, cycles: i32) -> String {
    let mut platform = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<_>>>();

    // println!("pre---");
    // print_matrix(&platform);
    // println!("---");
    //
    let mut previous_cycle = platform.clone();

    for i in 0..cycles {
        platform = transpose_matrix(&platform);
        platform = platform
            .iter_mut()
            .map(|row| {
                row.split_mut(|c| c == &Tile::R)
                    .for_each(|section| section.sort());
                row.clone()
            })
            .collect::<Vec<_>>();

        platform = transpose_matrix(&platform);
        // println!("post-north---");
        // print_matrix(&platform);
        // println!("---");

        platform = platform
            .iter_mut()
            .map(|row| {
                row.split_mut(|c| c == &Tile::R)
                    .for_each(|section| section.sort());
                row.clone()
            })
            .collect::<Vec<_>>();

        // println!("post-west---");
        // print_matrix(&platform);
        // println!("---");

        platform = transpose_matrix(&platform);

        platform = platform
            .iter_mut()
            .map(|row| {
                row.split_mut(|c| c == &Tile::R)
                    .for_each(|section| section.sort_by(|a, b| b.cmp(a)));
                row.clone()
            })
            .collect::<Vec<_>>();

        platform = transpose_matrix(&platform);
        // println!("post-south---");
        // print_matrix(&platform);
        // println!("---");

        platform = platform
            .iter_mut()
            .map(|row| {
                row.split_mut(|c| c == &Tile::R)
                    .for_each(|section| section.sort_by(|a, b| b.cmp(a)));
                row.clone()
            })
            .collect::<Vec<_>>();

        if previous_cycle == platform {
            break;
        } else {
            previous_cycle = platform.clone();
        }

        // println!("post-east---");
        // print_matrix(&platform);
        // println!("---");
        if i % 1_000_000 == 0 {
            println!("cycle: {i}");
        }
    }

    // let len = transposed.len() - 1;
    // let load = transposed.iter().enumerate().fold(0, |mut acc, (i, row)| {
    //     let stones = row.iter().filter(|c| c == &&Tile::O).count();
    //     acc += (len - i + 1) * stones;
    //     acc
    // });
    platform
        .iter()
        .map(|row| row.iter().map(|c| c.to_char()).collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        1
    )]
    #[case(
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        2
    )]
    #[case(
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        3
    )]
    fn test_run_cycle(#[case] expected: &str, #[case] cycles: i32) -> Result<()> {
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

        let result = run_cycle(input, cycles);
        println!("---------");
        println!("input:\n{}\n---", input);
        println!("expected:\n{}\n---", expected);
        println!("result:\n{}\n---", result);
        assert_eq!(result, expected);
        Ok(())
    }

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
        assert_eq!(process(input)?, "64");
        Ok(())
    }
}
