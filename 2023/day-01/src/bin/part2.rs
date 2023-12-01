use anyhow::Result;

fn main() -> Result<()> {
    let input_file = include_str!("../../input2.txt");
    let mut total_code = 0;
    for line in input_file.split("\n") {
        if line.trim().len() < 1 {
            continue;
        }
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

    #[test]
    fn test_it_works_qxtbbtwo7jrdgxlcpxbczxhnpjthreetwogcfl() {
        assert_eq!(get_code("qxtbbtwo7jrdgxlcpxbczxhnpjthreetwogcfl"), 22);
    }

    #[test]
    fn test_it_works_4md() {
        assert_eq!(get_code("4md"), 44);
    }

    #[test]
    fn test_it_works_5sixfive() {
        assert_eq!(get_code("5sixfive"), 55);
    }

    #[test]
    fn test_it_works_rv1() {
        assert_eq!(get_code("rv1"), 11);
    }

    #[test]
    fn test_it_works_two1nine() {
        assert_eq!(get_code("two1nine"), 29);
    }

    #[test]
    fn test_it_works_eightwothree() {
        assert_eq!(get_code("eightwothree"), 83);
    }

    #[test]
    fn test_it_works_1eightwothree() {
        assert_eq!(get_code("1eightwothree"), 13);
    }

    #[test]
    fn test_it_works_df1dsfd() {
        assert_eq!(get_code("df1dsfd"), 11);
    }

    #[test]
    fn test_it_works_dftwodsfd() {
        assert_eq!(get_code("dftwodsfd"), 22);
    }

    #[test]
    fn test_it_works_threemctclrrzvqzdnmkpgffive3snhxseven() {
        assert_eq!(get_code("threemctclrrzvqzdnmkpgffive3snhxseven"), 37);
    }
}
