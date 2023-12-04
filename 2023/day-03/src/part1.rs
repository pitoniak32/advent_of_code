use anyhow::Result;

#[derive(Debug)]
pub enum Token {
    Symbol,
    Part(u32),
    Period,
}

pub fn touch_c(c: &char, i: i32, j: i32, toks: &Vec<Vec<char>>) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    if let Some(j_toks) = toks.get(i as usize) {
        if let Some(ij_c) = j_toks.get(j as usize) {
            return c == ij_c;
        }
    }
    false
}

// 884871023127 = too high
// 498559
// 497027 = too low
pub fn process(input: &str) -> Result<String> {
    let syms = vec!['$', '+', '#', '*', '/', '%', '=', '-', '&', '@'];
    let mut tokens: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        tokens.push(line.chars().collect())
    }
    let mut valid = vec![];
    for (i, i_tok) in tokens.iter().enumerate() {
        let mut num: Vec<char> = vec![];
        let mut touching = false;
        for (j, j_tok) in i_tok.iter().enumerate() {
            dbg!(j_tok);
            if j_tok.is_ascii_digit() {
                num.push(*j_tok);
                if syms.iter().any(|s| {
                    touch_c(s, i as i32 - 1, j as i32 - 1, &tokens)
                        || touch_c(s, i as i32 - 1, j as i32, &tokens)
                        || touch_c(s, i as i32 - 1, j as i32 + 1, &tokens)
                        || touch_c(s, i as i32, j as i32 - 1, &tokens)
                        || touch_c(s, i as i32, j as i32 + 1, &tokens)
                        || touch_c(s, i as i32 + 1, j as i32 - 1, &tokens)
                        || touch_c(s, i as i32 + 1, j as i32, &tokens)
                        || touch_c(s, i as i32 + 1, j as i32 + 1, &tokens)
                }) {
                    touching = true;
                }
                println!("is: {j_tok}, {touching}, {num:?}");
            } else {
                println!("not: {j_tok}, {touching}, {num:?}");
                if touching {
                    let num = String::from_iter(num.clone())
                        .parse::<i64>()
                        .expect("should be a number");
                    println!("pushing {num}");
                    valid.push(num);
                }
                println!("clearing {num:?}");
                num.clear();
                touching = false;
            }
        }
        if touching {
            let num = String::from_iter(num.clone())
                .parse::<i64>()
                .expect("should be a number");
            println!("pushing {num}");
            valid.push(num);
        }
    }
    dbg!(valid.clone());
    let val: i64 = valid.iter().sum();
    Ok(val.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process_initial() -> Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_edge() -> Result<()> {
        let input = "
467..114..
...*......
..35...633
......#...";
        assert_eq!("1135", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_initial_one_line() -> Result<()> {
        let input = "
..................................#...*.........*..........................$...........*....................................................
........*....978*275.................974...*....81.........../...........%.405.........224.....-....*......409*20....../............*...*...
.122.273...............759....137...........574............229...................................497.41............................872.78...";
        assert_eq!("5930", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_one_line() -> Result<()> {
        let input = "
............................#...........................................................................*......................*............
......159.................553..294#...*.........*......................42..$...........*......884.......................950...482..611...249
........*....978*275.................974...*....81.........../...........%.405.........224.....-....*......409*20....../............*...*...";
        assert_eq!("5555", process(input)?);
        Ok(())
    }
    // 20 + 409 + 224 + 405 + 81 + 974 + 275 + 978 + 249 + 611 + 482 + 294 + 553
}
