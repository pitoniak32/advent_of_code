

use anyhow::Result;

#[derive(Debug)]
pub enum Tile {
    Vertical,
    Horizontal,
    Space,
    Forward,
    Backward,
}

#[derive(Debug)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct ELoc {
    pos: Pos,
    dir: Dir,
}

pub fn process(input: &str) -> Result<String> {
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    '.' => Tile::Space,
                    '\\' => Tile::Backward,
                    '/' => Tile::Forward,
                    v => unreachable!("{v} is invalid tile."),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let _start_pos = ELoc {
        pos: Pos { x: 0, y: 0 },
        dir: Dir::E,
    };

    let _results = tiles.iter().fold(Vec::<Pos>::new(), |acc, row| {
        row.iter().for_each(|_t| {});
        acc
    });

    dbg!(&tiles);

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(process(input)?, "46");
        Ok(())
    }
}
