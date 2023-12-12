use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Eq)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone)]
pub struct PosPos(Pos, Pos);

impl PartialEq for PosPos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || (self.0 == other.1) && (self.1 == other.0)
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
        } else {
            if rows > 0 {
                for _ in 0..(rows * 2) {
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

pub fn expand_image(image: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    transpose_image(&expand_rows(&transpose_image(&expand_rows(image))))
}

// 9227826 = too low
// 9274989 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, image) = parse_image(input).expect("valid parse");

    let expanded = expand_image(&image);

    println!(
        "(pre)space: {}x{}",
        image.get(0).unwrap().len(),
        image.len()
    );
    println!(
        "(exp)space: {}x{}",
        expanded.get(0).unwrap().len(),
        expanded.len()
    );

    let galaxies = expanded
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

    println!("galaxies: {}", galaxies.len());

    let mut unique_pairs = Vec::<PosPos>::new();
    for outer_gal in galaxies.clone() {
        galaxies.iter().for_each(|inner_gal| {
            if !unique_pairs.contains(&PosPos(inner_gal.clone(), outer_gal.clone()))
                && inner_gal != &outer_gal
            {
                unique_pairs.push(PosPos(inner_gal.clone(), outer_gal.clone()));
            }
        });
    }

    // println!("{:?}", unique_pairs);

    let dist = unique_pairs.iter().fold(0, |mut acc, gal_pair| {
        acc += manhattan_dist(&gal_pair.0, &gal_pair.1);
        acc
    });

    Ok(dist.to_string())
}

fn manhattan_dist(pos1: &Pos, pos2: &Pos) -> i32 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_unique() -> Result<()> {
        // let mut set: HashSet<PosPos> = HashSet::new();
        // set.insert(PosPos(Pos { x: 1, y: 1 }, Pos { x: 2, y: 2 }));
        // set.insert(PosPos(Pos { x: 2, y: 2 }, Pos { x: 1, y: 1 }));
        let mut ve = Vec::new();
        let group1 = PosPos(Pos { x: 2, y: 2 }, Pos { x: 1, y: 1 });
        let group2 = PosPos(Pos { x: 1, y: 1 }, Pos { x: 2, y: 2 });

        ve.push(group1);

        if !ve.contains(&group2) {
            ve.push(group2);
        }

        assert_eq!(
            PosPos(Pos { x: 1, y: 1 }, Pos { x: 2, y: 2 }),
            PosPos(Pos { x: 2, y: 2 }, Pos { x: 1, y: 1 })
        );
        // assert_eq!(set.len(), 1);
        assert_eq!(ve.len(), 1);
        Ok(())
    }

    #[rstest]
    #[case(Pos { x: 1, y: 1 }, Pos { x: 1, y: 1 }, 0)]
    #[case(Pos { x: 1, y: 6 }, Pos { x: 5, y: 11 }, 9)]
    #[case(Pos { x: 1, y: 1 }, Pos { x: 4, y: 4 }, 6)]
    fn test_point_dist(#[case] pos1: Pos, #[case] pos2: Pos, #[case] expected: i32) -> Result<()> {
        assert_eq!(manhattan_dist(&pos1, &pos2), expected);
        Ok(())
    }

    #[test]
    fn test_expand() -> Result<()> {
        let input = "#####
.....
.....
....#";
        dbg!(expand_image(&parse_image(input)?.1));

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
