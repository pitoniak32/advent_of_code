use std::{collections::HashMap, fmt::Display};

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

// 118786 = too high
// 118780 = too high
// 118747 = just right
pub fn process(input: &str) -> Result<String> {
    let cycles = 1_000_000_000;

    let load = run_cycle(input, cycles);

    Ok(load.0)
}

pub fn run_cycle(input: &str, cycles: i32) -> (String, String) {
    let mut platform = input
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect::<Vec<Vec<_>>>();

    let mut cycle_cache: HashMap<String, (usize, Vec<Vec<Tile>>)> = HashMap::new();

    let mut remaining_cycles = 0;
    let mut cycle_count = 0;

    for i in 0..cycles {
        let platform_string = platform
            .iter()
            .map(|row| row.iter().map(|c| c.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        if remaining_cycles == 0 {
            if let Some(hit) = cycle_cache.get(&platform_string) {
                let start_index = hit.0 as i32;
                remaining_cycles = (1_000_000_000 - start_index) % (i - start_index);
                println!(
                    "cycle: {}, start: {}, period: {}",
                    i, hit.0, remaining_cycles
                );
            } else {
                cycle_cache.insert(platform_string, (i as usize, platform.clone()));
            }
        } else if cycle_count == remaining_cycles - 1 {
            println!(
                "i: {}, period: {}, count: {}",
                i, remaining_cycles, cycle_count
            );
            break;
        } else if cycle_count < remaining_cycles {
            cycle_count += 1;
            println!(
                "i: {}, period: {}, count: {}",
                i, remaining_cycles, cycle_count
            );
        }

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
    }

    let len = platform.len();
    let load = platform.iter().enumerate().fold(0, |mut acc, (i, row)| {
        let stones = row.iter().filter(|c| c == &&Tile::O).count();
        acc += (len - i) * stones;
        acc
    });

    (
        load.to_string(),
        platform
            .iter()
            .map(|row| row.iter().map(|c| c.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"),
    )
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
        println!("result:\n{}\n---", result.1);
        assert_eq!(result.1, expected);
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
