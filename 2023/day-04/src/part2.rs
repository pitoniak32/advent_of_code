use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

// 5571760 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, mut cards) = parse_cards(input).unwrap();
    let mut cards_matching_count: BTreeMap<usize, usize> = BTreeMap::new();

    for card in &cards {
        cards_matching_count.insert(card.id, 1);
    }

    for card in cards.iter() {
        let mut matching_card_count = 0;
        for yc in &card.yours {
            for wc in &card.winning {
                if yc == wc {
                    matching_card_count += 1;
                }
            }
        }
        cards_matching_count.insert(card.id, matching_card_count);
    }

    let _ = cards_matching_count
        .iter()
        .enumerate()
        .map(|(idx, (k, v))| {
            let range = *k..k + *v;
            let curr_card_count = &cards.get(idx).unwrap().count.clone();
            for i in range {
                if let Some(card) = cards.get_mut(i) {
                    card.count += curr_card_count;
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(cards.iter().map(|c| c.count).sum::<usize>().to_string())
}

#[derive(Debug)]
pub struct Card<'a> {
    pub id: usize,
    pub count: usize,
    pub yours: Vec<&'a str>,
    pub winning: Vec<&'a str>,
}

pub fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(tag("Card"), preceded(space1, digit1))(input)?;
    let (input, (yours, winning)) = preceded(
        tag(":"),
        preceded(
            space0,
            separated_pair(
                separated_list1(space1, digit1),
                tag(" | "),
                preceded(space0, separated_list1(space1, digit1)),
            ),
        ),
    )(input)?;

    Ok((
        input,
        Card {
            id: id.parse().unwrap(),
            count: 1,
            yours,
            winning,
        },
    ))
}
pub fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, parse_card)(input)?;
    Ok((input, cards))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
