use std::collections::BTreeMap;

use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Symbol(char),
    Gear,
    Dot,
    Digit { pos: Pos, val: char },
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Number {
    pub digits: Vec<Token>,
}

impl Number {
    pub fn get_value(&self) -> u32 {
        String::from_iter(self.digits.iter().map(|tok| match tok {
            Token::Digit { pos: _, val } => val,
            _ => unreachable!("should all be digits"),
        }))
        .parse()
        .expect("should be a number")
    }
}

#[derive(Debug)]
pub struct Gear {
    pub pos: Pos,
    pub touching_nums: Vec<Number>,
}

pub fn process(input: &str) -> Result<String> {
    let tokens: BTreeMap<Pos, Token> = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().map(move |(y, character)| {
                let pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
                (
                    pos,
                    match character {
                        '.' => Token::Dot,
                        c if c.is_ascii_digit() => Token::Digit { pos, val: c },
                        '*' => Token::Gear,
                        s => Token::Symbol(s),
                    },
                )
            })
        })
        .collect::<BTreeMap<Pos, Token>>();

    let mut nums: Vec<Number> = Vec::new();
    let mut num: Number = Number { digits: vec![] };
    let _ = tokens
        .values()
        .map(|tok| {
            if let Token::Digit {
                pos: _pos,
                val: _val,
            } = tok
            {
                num.digits.push(*tok)
            } else if !num.digits.is_empty() {
                nums.push(num.clone());
                num = Number { digits: vec![] };
            }
        })
        .collect::<Vec<_>>();

    let possibles = [
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
        Pos { x: 0, y: -1 },
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: -1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: 1 },
    ];

    let matched = tokens
        .iter()
        .filter_map(|(tok_pos, tok)| match tok {
            Token::Gear => Some(
                nums.iter()
                    .filter(|n| {
                        n.digits.iter().any(|d| match d {
                            Token::Digit { pos, val: _ } => {
                                for possible in possibles {
                                    if pos.x == tok_pos.x + possible.x
                                        && pos.y == tok_pos.y + possible.y
                                    {
                                        return true;
                                    }
                                }
                                false
                            }
                            _ => false,
                        })
                    })
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .collect::<Vec<_>>();

    let nums = matched
        .iter()
        .filter_map(|nums| {
            if nums.len() < 2 {
                return None;
            }
            Some(nums.iter().map(|num| num.get_value()).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let total: u32 = nums.iter().map(|nums| nums.iter().product::<u32>()).sum();

    dbg!(nums);

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process_initial() -> Result<()> {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input)?, "467835");
        Ok(())
    }
}
