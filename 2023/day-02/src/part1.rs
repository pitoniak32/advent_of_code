use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let mut games = vec![];
    for line in input.lines() {
        games.push(parse_game(line)?)
    }
    dbg!(games);
    Ok("".to_string())
}

pub fn parse_game(input: &str) -> Result<Game> {
    let mut parts = input.split_inclusive(": ");
    let game = parts.next().expect("Game prefix should exist");
    let binding = game.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    let game_id = binding.get(0).expect("should always have game id");

    let rest = parts.next().expect("there should be a set");

    let sets = parse_sets(rest)?;

    Ok(Game { id: game_id.clone(), sets })
}

pub fn parse_sets(input: &str) -> Result<Vec<Set>> {
    let mut sets = vec![];
    for set in input.split(";") {
        sets.push( Set { cubes: parse_cubes(set)? });
    }
    Ok(sets)
}

pub fn parse_cubes(input: &str) -> Result<Vec<Cube>> {
    let mut cubes = vec![];
    for cube in input.split(",") {
        let mut cube_parts = cube.trim().split(" ");
        let count = cube_parts.next().expect("should have a cube count");
        let color = cube_parts.next().expect("should have a cube color");
        cubes.push(Cube::str_cube(color, count.parse().expect("should be a number")));
    }
    Ok(cubes)
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

#[derive(Debug, PartialEq)]
pub struct Set {
    pub cubes: Vec<Cube>,
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
    fn test_process() -> Result<()> {
        let expected = Game {
            id: 1,
            sets: vec![
                Set {
                    cubes: vec![Cube::Blue(3), Cube::Red(4)],
                },
                Set {
                    cubes: vec![Cube::Red(1), Cube::Green(2), Cube::Blue(6)],
                },
                Set {
                    cubes: vec![Cube::Green(2)],
                },
            ],
        };
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(parse_game(input)?, expected);
        Ok(())
    }
}
