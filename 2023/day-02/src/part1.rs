use anyhow::Result;

// 2176 = just right
// 224 = too low
pub fn process(input: &str) -> Result<u32> {
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut possible = true;
    let mut total = 0;
    for line in input.lines() {
        let game = parse_game(line)?;
        dbg!(game.id);
        for set in game.sets {
            match set.cubes_blue {
                Some(Cube::Blue(x)) => {
                    if x > blue {
                        possible = false;
                    }
                },
                None => {},
                _ => unreachable!("should only have blue")
            }
            match set.cubes_red {
                Some(Cube::Red(x)) => {
                    if x > red {
                        possible = false;
                    }
                },
                None => {},
                _ => unreachable!("should only have red")
            }
            match set.cubes_green {
                Some(Cube::Green(x)) => {
                    if x > green {
                        possible = false;
                    }
                },
                None => {},
                _ => unreachable!("should only have green")
            }
        }
        if possible {
            // dbg!(game.id);
            total += game.id;
        }
        possible = true;
    }
    Ok(total)
}

pub fn parse_game(input: &str) -> Result<Game> {
    let mut parts = input.split_inclusive(": ");
    let game = parts.next().expect("Game prefix should exist");
    let game_id = String::from_iter(game.chars().filter(|c| c.is_digit(10)).collect::<Vec<char>>()).parse::<u32>().expect("should be u32");

    let rest = parts.next().expect("there should be a set");

    let sets = parse_sets(rest)?;

    Ok(Game { id: game_id.clone(), sets })
}

pub fn parse_sets(input: &str) -> Result<Vec<Set>> {
    let mut sets = vec![];
    for set in input.split(";") {
        sets.push(parse_cubes(set)?);
    }
    Ok(sets)
}

pub fn parse_cubes(input: &str) -> Result<Set> {
    let mut set = Set {
        cubes_red: None,
        cubes_green: None,
        cubes_blue: None,
    };
    for cube in input.split(",") {
        let mut cube_parts = cube.trim().split(" ");
        let count = cube_parts.next().expect("should have a cube count");
        let color = cube_parts.next().expect("should have a cube color");
        let cube = Cube::str_cube(color, count.parse().expect("should be a number"));
        match cube {
            Cube::Red(_) => set.cubes_red = Some(cube),
            Cube::Green(_) => set.cubes_green = Some(cube),
            Cube::Blue(_) => set.cubes_blue = Some(cube),
        }
    }

    Ok(set)
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

#[derive(Debug, PartialEq)]
pub struct Set {
    pub cubes_red: Option<Cube>,
    pub cubes_green: Option<Cube>,
    pub cubes_blue: Option<Cube>,
}

#[derive(Debug, PartialEq)]
pub enum Cube {
    Red(u8),
    Blue(u8),
    Green(u8),
}

impl Cube {
    fn str_cube(col: &str, count: u8) -> Self {
        match col {
            "red" => Cube::Red(count),
            "green" => Cube::Green(count),
            "blue" => Cube::Blue(count),
            _ => unreachable!("color is not supported")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_parse_game() -> Result<()> {
        let expected = Game {
            id: 1,
            sets: vec![
                Set {
                    cubes_blue: Some(Cube::Blue(3)),
                    cubes_red: Some(Cube::Red(4)),
                    cubes_green: None,
                },
                Set {
                    cubes_red: Some(Cube::Red(1)), 
                    cubes_green: Some(Cube::Green(2)),
                    cubes_blue: Some(Cube::Blue(6)),
                },
                Set {
                    cubes_green: Some(Cube::Green(2)),
                    cubes_red: None,
                    cubes_blue: None,
                },
            ],
        };
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(parse_game(input)?, expected);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let expected = 8;
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input)?, expected);
        Ok(())
    }
}
