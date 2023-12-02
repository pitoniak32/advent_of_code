use anyhow::Result;

fn main() -> Result<()> {
    let input_file = include_str!("../../input2.txt");
    let mut total_code = 0;
    for line in input_file.lines() {
        let code = get_code(line);
        total_code += code;
    }
    println!("final code: {total_code}");
    Ok(())
}

// 52859 too high
// 52840 just right
// 52722 too low
fn get_code(code: &str) -> u32 {
    let str_nums = [
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ];
    let mut highest_idx = 0;
    let mut lowest_idx = code.len();
    let mut first_num: i32 = -1;
    let mut last_num: i32 = -1;

    for str_num in str_nums {
        let indices = code.match_indices(&str_num.0);
        for (idx, _) in indices {
            if idx <= lowest_idx {
                lowest_idx = idx;
                first_num = str_num.1;
            }
            if idx >= highest_idx {
                highest_idx = idx;
                last_num = str_num.1;
            }
        }
    }

    for (idx, l_char) in code.chars().enumerate() {
        if l_char.is_digit(10) {
            if idx <= lowest_idx {
                lowest_idx = idx;
                first_num = l_char
                    .to_digit(10)
                    .expect("to be u32")
                    .try_into()
                    .unwrap();
            }
            if idx >= highest_idx {
                highest_idx = idx;
                last_num = l_char
                    .to_digit(10)
                    .expect("to be u32")
                    .try_into()
                    .unwrap();
            }
        }
    }

    format!("{}{}", first_num, last_num)
        .parse::<u32>()
        .expect("to be u32")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("4md", 44)]
    #[case("5sixfive", 55)]
    #[case("rv1", 11)]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("1eightwothree", 13)]
    #[case("df1dsfd", 11)]
    #[case("dftwodsfd", 22)]
    #[case("threemctclrrzvqzdnmkpgffive3snhxseven", 37)]
    #[case("qxtbbtwo7jrdgxlcpxbczxhnpjthreetwogcfl", 22)]
    fn test_it_works_(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(get_code(input), expected);
    }
}
