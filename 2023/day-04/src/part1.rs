use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

// 24 = not right
// 23941 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, cards) = parse_cards(input).unwrap();
    let mut counter = 0;
    for card in &cards {
        let mut card_count = 0;
        for yc in &card.yours {
            for wc in &card.winning {
                if yc == wc {
                    if card_count > 1 {
                        card_count *= 2;
                    } else {
                        card_count += 1;
                    }
                }
            }
        }
        counter += card_count;
    }
    Ok(counter.to_string())
}

#[derive(Debug)]
pub struct Card<'a> {
    pub id: &'a str,
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

    Ok((input, Card { id, yours, winning }))
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
        assert_eq!("13", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_one() -> Result<()> {
        let input = "Card   3: 60 78 77 44 62 54 94 50 32 11 |  2  6 89 50 11 60 57 53 71 44 47 62 49 42 73 78 77 54 99 29 35 94 32 68 74";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
