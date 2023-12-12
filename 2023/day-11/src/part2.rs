use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Eq)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
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
    AllSpace,
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

pub fn expand_rows(image: &[Vec<Tile>], expand_by: i64) -> (Vec<Vec<Tile>>, Vec<Pos>) {
    let mut rows_expanded: Vec<Pos> = Vec::new();
    let mut rows = 0;
    let image_expanded = image
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, row)| {
            if row.contains(&Tile::Galaxy) || i == image.len() - 1 {
                if rows > 0 {
                    // x = starting index. y = length
                    rows_expanded.push(Pos {
                        x: i as i64 - rows,
                        y: (rows * expand_by) - rows,
                    });
                    for _ in 0..rows {
                        acc.push(
                            std::iter::repeat(Tile::AllSpace)
                                .take(row.len())
                                .collect::<Vec<Tile>>(),
                        );
                    }
                }
                acc.push(row.clone());
                rows = 0;
            } else {
                rows += 1;
            }

            acc
        });

    (image_expanded, rows_expanded)
}

pub fn expand_image(image: &[Vec<Tile>], expand_by: i64) -> (Vec<Vec<Tile>>, Vec<Pos>, Vec<Pos>) {
    //transpose_image(&expand_rows(&transpose_image(&expand_rows(image))))
    let (expand_image1, expanded_rows) = &expand_rows(image, expand_by);
    let (expand_image2, expanded_cols) = &expand_rows(&transpose_image(expand_image1), expand_by);

    (
        transpose_image(expand_image2),
        expanded_rows.clone(),
        expanded_cols.clone(),
    )
}

pub fn process(input: &str, expand_by: i64) -> Result<String> {
    let (_, image) = parse_image(input).expect("valid parse");

    let (expanded_image, expanded_rows, expanded_cols) = expand_image(&image, expand_by);

    dbg!(&expanded_rows);
    dbg!(&expanded_cols);
    dbg!(&expanded_image);

    let galaxies = expanded_image
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, tile)| {
                if *tile == Tile::Galaxy {
                    Some(Pos::new(x as i64, y as i64))
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

    println!("unique pairs: {}", unique_pairs.len());

    let dist = get_dist(unique_pairs, expanded_rows, expanded_cols);

    Ok(dist.to_string())
}

fn get_dist(unique: Vec<PosPos>, exp_rows: Vec<Pos>, exp_cols: Vec<Pos>) -> i64 {
    unique.iter().fold(0, |mut acc, gal_pair| {
        let mut pair1 = gal_pair.0.clone();
        let pair2 = gal_pair.1.clone();

        exp_rows.iter().for_each(|er| {
            if (gal_pair.0.y..gal_pair.1.y).contains(&er.x)
                || (gal_pair.1.y..gal_pair.0.y).contains(&er.x)
            {
                if gal_pair.0.y - gal_pair.1.y > 0 {
                    pair1.y += er.y;
                } else {
                    pair1.y -= er.y;
                }
            }
        });
        exp_cols.iter().for_each(|ec| {
            if (gal_pair.0.x..gal_pair.1.x).contains(&ec.x)
                || (gal_pair.1.x..gal_pair.0.x).contains(&ec.x)
            {
                if gal_pair.0.x - gal_pair.1.x > 0 {
                    pair1.x += ec.y;
                } else {
                    pair1.x -= ec.y;
                }
            }
        });

        acc += manhattan_dist(&pair1, &pair2);
        acc
    })
}

fn manhattan_dist(pos1: &Pos, pos2: &Pos) -> i64 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_unique() -> Result<()> {
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

    // #[rstest]
    // #[case(Pos { x: 1, y: 1 }, Pos { x: 1, y: 1 }, 0)]
    // #[case(Pos { x: 1, y: 6 }, Pos { x: 5, y: 11 }, 9)]
    // #[case(Pos { x: 1, y: 1 }, Pos { x: 4, y: 4 }, 6)]
    // #[case(Pos { x: -1, y: -3 }, Pos { x: 2, y: 3 }, 6)]
    // fn test_man_dist(#[case] pos1: Pos, #[case] pos2: Pos, #[case] expected: i64) -> Result<()> {
    //     assert_eq!(manhattan_dist(&pos1, &pos2), expected);
    //     Ok(())
    // }

    // #[rstest]
    // #[case(vec![PosPos(Pos { x: 0, y: 0 }, Pos { x: 2, y: 3 })], vec![Pos {x: 1, y: 3}], vec![Pos {x:1, y:1}])]
    // fn test_point_dist(
    //     #[case] pairs: Vec<PosPos>,
    //     #[case] exp_rows: Vec<Pos>,
    //     #[case] exp_cols: Vec<Pos>,
    // ) -> Result<()> {
    //     assert_eq!(get_dist(pairs, exp_rows, exp_cols), 0);
    //     Ok(())
    // }

    // #[rstest]
    // #[case(2, "8")]
    // // #[case(10, "1030")]
    // // #[case(100, "8410")]
    // fn test_get_dist(#[case] expand_by: i64, #[case] distance: &str) -> Result<()> {
    //     Ok(())
    // }

    //     #[rstest]
    //     #[case(2, "8")]
    //     // #[case(10, "1030")]
    //     // #[case(100, "8410")]
    //     fn test_process_small(#[case] expand_by: i64, #[case] distance: &str) -> Result<()> {
    //         let input = "#........
    // .........
    // .........
    // ..#......";
    //         // #...........
    //         // ............
    //         // ............
    //         // ............
    //         // ............
    //         // ....#.......
    //         // 2 row ranges of 1 length at y = 3, 7.
    //         // 3 col ranges of 1 length at x = 2, 5, 8.
    //         assert_eq!(process(input, expand_by)?, distance);
    //         Ok(())
    //     }
    //
    #[rstest]
    #[case(2, "5")]
    // #[case(10, "1030")]
    // #[case(100, "8410")]
    fn test_process_2_gap(#[case] expand_by: i64, #[case] distance: &str) -> Result<()> {
        let input = "#.........
..........
..........
#.........";
        // 2 row ranges of 1 length at y = 3, 7.
        // 3 col ranges of 1 length at x = 2, 5, 8.
        assert_eq!(process(input, expand_by)?, distance);
        Ok(())
    }

    #[rstest]
    #[case(2, "374")]
    #[case(10, "1030")]
    #[case(100, "8410")]
    fn test_process(#[case] expand_by: i64, #[case] distance: &str) -> Result<()> {
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
        // 2 row ranges of 1 length at y = 3, 7.
        // 3 col ranges of 1 length at x = 2, 5, 8.
        assert_eq!(process(input, expand_by)?, distance);
        Ok(())
    }
}
