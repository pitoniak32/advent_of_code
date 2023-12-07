use std::collections::HashMap;

use anyhow::Result;
use nom::{
    character::complete::{self, alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct Game {
    pub hand: Vec<char>,
    pub bid: u32,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: HashMap<char, u32>,
    pub strength: HandStrength,
    pub rank: u32,
}

/// Variants are ordered by their value
#[derive(Debug, PartialEq, PartialOrd)]
pub enum HandStrength {
    /// High card, where all cards' labels are distinct: 23456
    HighCard,
    /// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    /// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    /// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    /// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    /// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind,
}

impl Hand {
    pub fn from_cards(cards: &HashMap<char, u32>) -> Self {
        let r = cards
            .iter()
            .map(|(card, count)| {
                match count {
                    // 5 => HandStrength::FiveOfAKind,
                    // 4 => HandStrength::FourOfAKind,
                    // 3 if todo!() => HandStrength::FullHouse,
                    // 3 => HandStrength::ThreeOfAKind,
                    // 2 if todo!() => HandStrength::TwoPair,
                    // 1 if todo!() => {
                    //     HandStrength::OnePair
                    //     // HandStrength::HighCard
                    // }
                    _ => HandStrength::ThreeOfAKind /* unreachable!("unhandled count value") */,
                }
            })
            .collect::<Vec<_>>();

        dbg!(r);

        Hand {
            cards: cards.clone(),
            strength: HandStrength::ThreeOfAKind,
            rank: 0,
        }
    }
}

pub fn calc_hand(hand: &Vec<char>) -> Hand {
    let mut hand_counts: HashMap<char, u32> = HashMap::new();

    let _ = hand
        .iter()
        .map(|card| {
            if let Some(hc) = hand_counts.get(card) {
                let count = hc + 1;
                hand_counts.insert(*card, count);
            } else {
                hand_counts.insert(*card, 1);
            }
        })
        .collect::<Vec<_>>();

    Hand::from_cards(&hand_counts)
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (o, (hand, bid)) =
        separated_pair(alphanumeric1, nom::bytes::complete::tag(" "), complete::u32)(input)?;
    Ok((
        o,
        Game {
            hand: hand.chars().collect(),
            bid,
        },
    ))
}

pub fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(input)
}

pub fn process(input: &str) -> Result<String> {
    let (_, games) = parse_games(input).expect("valid parse");

    let hands = games.iter().fold(Vec::new(), |mut acc, game| {
        acc.push(calc_hand(&game.hand));
        acc
    });

    dbg!(games, hands);

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(process(input)?, "6440");
        Ok(())
    }
}
