use std::{cmp::Ordering, ops::Deref};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{self, alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

/// Variants are ordered by their value
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(card: &char) -> Self {
        match card {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!("card type not implemented"),
        }
    }
}

#[derive(Debug, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub strength: HandStrength,
    pub bid: u32,
}

// Implementing PartialEq
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength > other.strength {
            return Ordering::Greater;
        } else if self.strength < other.strength {
            return Ordering::Less;
        }

        let self_iter = self.cards.iter();
        let mut other_iter = other.cards.iter();

        for sc in self_iter {
            let oc = other_iter
                .next()
                .expect("should have the same number of cards as self");

            if sc == oc {
                continue;
            } else if sc > oc {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

/// Variants are ordered by their value
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash)]
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
    pub fn from_cards(cards: &str, bid: u32) -> Self {
        let card_counts = cards.chars().counts();
        let counted = if let Some(j_count) = card_counts.get(&'J') {
            if j_count == &5 {
                "5".to_string()
            } else {
                card_counts
                    .iter()
                    .filter_map(|(key, value)| (*key != 'J').then_some(value))
                    .sorted()
                    .with_position()
                    .map(|(pos, cc)| match pos {
                        itertools::Position::Last => cc + j_count,
                        itertools::Position::Only => cc + j_count,
                        _ => *cc,
                    })
                    .join("")
            }
        } else {
            card_counts.values().sorted().join("")
        };

        let strength = match counted.deref() {
            "5" => HandStrength::FiveOfAKind,
            "14" => HandStrength::FourOfAKind,
            "23" => HandStrength::FullHouse,
            "113" => HandStrength::ThreeOfAKind,
            "122" => HandStrength::TwoPair,
            "1112" => HandStrength::OnePair,
            "11111" => HandStrength::HighCard,
            value => unreachable!("should not be ran, got: {}", value),
        };

        Hand {
            bid,
            cards: cards
                .chars()
                .map(|c| Card::from_char(&c))
                .collect::<Vec<_>>(),
            strength,
        }
    }
}

pub fn parse_game(input: &str) -> IResult<&str, (&str, u32)> {
    let (o, (hand, bid)) =
        separated_pair(alphanumeric1, nom::bytes::complete::tag(" "), complete::u32)(input)?;
    Ok((o, (hand, bid)))
}

pub fn parse_games(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
    separated_list1(line_ending, parse_game)(input)
}

// 247899149 = just right
// 247866544 = too low
pub fn process(input: &str) -> Result<String> {
    let (_, games) = parse_games(input).expect("valid parse");

    let mut hands = games.iter().fold(Vec::new(), |mut acc, game| {
        acc.push(Hand::from_cards(game.0, game.1));
        acc
    });

    hands.sort();

    let res: usize = hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid as usize * (i + 1))
        .sum();

    Ok(res.to_string())
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

        assert_eq!(process(input)?, "5905");
        Ok(())
    }
}
