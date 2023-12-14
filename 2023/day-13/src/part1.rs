use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::line_ending,
    multi::separated_list1,
    IResult,
};

// 37561 = just right
pub fn process(input: &str) -> Result<String> {
    let (_, patterns) = parse_patterns(input).expect("valid parse");

    let result = patterns.iter().fold(0, |mut acc, pattern| {
        acc += check_horizontal_vertical(pattern);
        acc
    });

    Ok(result.to_string())
}

pub fn parse_patterns(input: &str) -> IResult<&str, Vec<Vec<String>>> {
    separated_list1(tag("\n\n"), parse_pattern)(input)
}

pub fn parse_pattern(input: &str) -> IResult<&str, Vec<String>> {
    let (o, pattern) = separated_list1(line_ending, take_till1(|c| c != '.' && c != '#'))(input)?;

    Ok((
        o,
        pattern
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>(),
    ))
}

pub fn transpose_image(image: &[Vec<char>]) -> Vec<Vec<char>> {
    let ncols = image[0].len();
    (0..ncols)
        .map(|col_idx| image.iter().map(|row| row[col_idx]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn check_symetry(pattern: &[String], horizontal_mul: u32) -> u32 {
    dbg!(pattern);

    let mut highest = -1;
    let mut top = 0;

    (0..pattern.len()).for_each(|split_at| {
        let (part1, part2) = pattern.split_at(split_at);

        dbg!(&part1, &part2);
        let part1_it = part1.iter();
        let part2_it = part2.iter();

        let part1_len = part1_it.len();
        let part2_len = part2_it.len();

        let part1_vec: Vec<&String>;
        let part2_vec: Vec<&String>;

        if part1_len > part2_len {
            let skipped = part1_len - part2_len;
            part1_vec = part1_it.skip(skipped).collect();
            part2_vec = part2_it.rev().collect();
        } else {
            part1_vec = part1_it.collect();
            part2_vec = part2_it.take(part1_len).rev().collect();
        }

        if dbg!(part1_vec) == dbg!(part2_vec) {
            highest = split_at as i32;
            top = part1.len();
        }
    });

    if horizontal_mul > 0 {
        top as u32 * horizontal_mul
    } else {
        top as u32
    }
}

fn check_horizontal_vertical(pattern: &[String]) -> u32 {
    let mut result = check_symetry(pattern, 100);
    if result == 0 {
        let chars: Vec<Vec<char>> = pattern
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let transposed_pattern = transpose_image(&chars)
            .iter()
            .map(String::from_iter)
            .collect::<Vec<_>>();

        dbg!(&transposed_pattern);

        result = check_symetry(&transposed_pattern, 0);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        5
    )]
    #[case(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        400
    )]
    #[case(
        "#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
#...##..#",
        300
    )]
    fn test_one(#[case] input: &str, #[case] expected: u32) -> Result<()> {
        let (_, pattern) = parse_pattern(input).expect("valid parse");
        assert_eq!(check_horizontal_vertical(&pattern), expected);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process(input)?, "405");
        Ok(())
    }
}
