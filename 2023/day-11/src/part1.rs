use anyhow::Result;
use itertools::izip;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Space,
    Galaxy,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Galaxy,
            '.' => Tile::Space,
            _ => unreachable!("invaild tile"),
        }
    }
}

pub fn parse_image(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        line_ending,
        fold_many1(
            alt((complete::char('#'), complete::char('.'))),
            Vec::new,
            |mut acc, item| {
                acc.push(Tile::from(item));
                acc
            },
        ),
    )(input)
}

pub fn transpose_image(image: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let ncols = image[0].len();
    (0..ncols)
        .map(|col_idx| {
            image
                .iter()
                .map(|row| row[col_idx].clone())
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>()
}

pub fn expand_rows(image: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut rows = 0;
    image.iter().fold(Vec::new(), |mut acc, row| {
        if !row.contains(&Tile::Galaxy) {
            rows += 1;
            println!("{:?}", rows);
        } else {
            if rows > 0 {
                for _ in 0..=rows {
                    acc.push(
                        std::iter::repeat(Tile::Space)
                            .take(row.len())
                            .collect::<Vec<Tile>>(),
                    );
                }
            }
            acc.push(row.clone());
            rows = 0;
        }
        acc
    })
}

pub fn expand_image(image: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    transpose_image(&expand_rows(&transpose_image(&expand_rows(image))))
}

pub fn process(input: &str) -> Result<String> {
    let (_, image) = parse_image(input).expect("valid parse");

    let expanded_image = expand_image(&image);

    let galaxies = expanded_image
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, tile)| {
                if *tile == Tile::Galaxy {
                    Some(Pos::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Pos>>();

    todo!("calculate the unique pairs of galaxies");

    todo!("find manhattan distance between pairs");

    Ok("".to_string())
}

fn manhattan_dist(pos1: &Pos, pos2: &Pos) -> i32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(Pos { x: 1, y: 1 }, Pos { x: 1, y: 1 }, 0)]
    #[case(Pos { x: 1, y: 6 }, Pos { x: 5, y: 11 }, 9)]
    #[case(Pos { x: 1, y: 1 }, Pos { x: 4, y: 4 }, 6)]
    fn test_point_dist(#[case] pos1: Pos, #[case] pos2: Pos, #[case] expected: i32) -> Result<()> {
        assert_eq!(manhattan_dist(&pos1, &pos2), expected);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(process(input)?, "374");
        Ok(())
    }
}
