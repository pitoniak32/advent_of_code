use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

pub fn parse_history(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), complete::i64)(input)
}

pub fn parse_histories(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, parse_history)(input)
}

// 1062 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, histories) = parse_histories(input).expect("should be valid parse");

    let ends: i64 = histories
        .into_iter()
        .fold(Vec::new(), |mut acc, hist| {
            let mut ex_hist = expand_history(hist);
            ex_hist.reverse();
            let inner_placeholders =
                ex_hist
                    .iter()
                    .enumerate()
                    .fold(Vec::new(), |mut inner_acc, (i, row)| {
                        let curr_first = row.first().expect("row should have last element");

                        if i == 0 {
                            inner_acc.push(*curr_first)
                        }

                        if let Some(next_row) = ex_hist.get(i + 1) {
                            let next_first =
                                next_row.first().expect("row should have last element");
                            let placeholder_filler = next_first
                                - *inner_acc.get(i).expect("should have placeholder filled");
                            inner_acc.push(placeholder_filler);
                        }
                        inner_acc
                    });
            acc.push(*inner_placeholders.last().unwrap());
            acc
        })
        .iter()
        .sum();

    Ok(ends.to_string())
}

pub fn expand_row(input: Vec<i64>) -> Vec<i64> {
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, val)| {
            if let Some(next) = input.get(i + 1) {
                acc.push(next - *val);
            };
            acc
        })
}

pub fn expand_history(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut rows = vec![input.clone()];

    let mut row = expand_row(input);
    rows.push(row.clone());

    while !row.iter().all(|e| e == &0) {
        row = expand_row(row);
        rows.push(row.clone());
    }

    dbg!(&rows);

    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3])]
    #[case(vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0])]
    #[case(vec![1, 3, 6, 10, 15, 21], vec![2, 3, 4, 5, 6])]
    #[case(vec![2, 3, 4, 5, 6], vec![1, 1, 1, 1])]
    fn test_expand_row(#[case] input: Vec<i64>, #[case] expected: Vec<i64>) -> Result<()> {
        assert_eq!(expand_row(input), expected);
        Ok(())
    }

    #[test]
    fn test_expand_history_1() -> Result<()> {
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(expand_history(input), expected);
        Ok(())
    }

    #[test]
    fn test_expand_history_2() -> Result<()> {
        let input = vec![1, 3, 6, 10, 15, 21];
        let expected = vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
            vec![0, 0, 0],
        ];

        assert_eq!(expand_history(input), expected);
        Ok(())
    }

    #[test]
    fn test_expand_history_3() -> Result<()> {
        let input = vec![10, 13, 16, 21, 30, 45];
        let expected = vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2],
            vec![0, 0],
        ];

        assert_eq!(expand_history(input), expected);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input)?, "2");
        Ok(())
    }
}
