use std::borrow::BorrowMut;

use anyhow::Result;

pub fn touch_gear(gears: &mut Vec<Gear>, digits: &Vec<char>, i: i32, j: i32, toks: &Vec<Vec<char>>) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    if let Some(j_toks) = toks.get(i as usize) {
        if let Some(ij_c) = j_toks.get(j as usize) {
            if &'*' == ij_c {
                add_gear(
                    Gear {
                        idx_i: i as usize,
                        idx_j: j as usize,
                        touching_nums: vec![],
                    },
                    gears,
                );
            }
        }
    }

    for gear in gears {
        if gear.idx_i == i as usize && gear.idx_j == j as usize {
            let tn_l = gear.touching_nums.len();
            let tn = &mut gear.touching_nums;
            dbg!(tn_l);
            dbg!(&tn);
            dbg!(&digits);
            if tn_l > 0 {
                tn[tn_l - 1] = digits.clone();
            } else {
                tn.push(digits.clone());
            }
            return true;
        }
    }
    false
}

#[derive(Debug)]
pub struct Gear {
    pub idx_i: usize,
    pub idx_j: usize,
    pub touching_nums: Vec<Vec<char>>,
}

pub fn add_gear(gear: Gear, gears: &mut Vec<Gear>) {
    let matched: Vec<&Gear> = gears
        .iter()
        .filter(|g| g.idx_i == gear.idx_i && g.idx_j == gear.idx_j)
        .collect();

    if matched.len() == 0 {
        gears.push(gear);
    }
}

pub fn process(input: &str) -> Result<String> {
    let mut tokens: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        tokens.push(line.chars().collect())
    }
    let mut gears: Vec<Gear> = vec![];
    for (i, i_tok) in tokens.iter().enumerate() {
        let mut num = vec![];
        for (j, j_tok) in i_tok.iter().enumerate() {
            if j_tok.is_ascii_digit() {
                num.push(*j_tok);
                let touching = touch_gear(&mut gears, &num, i as i32 - 1, j as i32 - 1, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32 - 1, j as i32, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32 - 1, j as i32 + 1, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32, j as i32 - 1, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32, j as i32 + 1, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32 + 1, j as i32 - 1, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32 + 1, j as i32, &tokens) ||
                               touch_gear(&mut gears, &num, i as i32 + 1, j as i32 + 1, &tokens);
            } else {
               let _ = gears.iter_mut().map(|g| {
                   if let Some(ll) = g.touching_nums.last() {
                        if ll.len() > 0 {
                            g.touching_nums.push(vec![])
                        }
                   }
               }).collect::<Vec<_>>();
            }
            dbg!(&gears);
            println!("j_tok: {j_tok}");
        }
    }
    let total: i64 = gears.iter().map(|g| {
        if g.touching_nums.len() > 1 {
            g.touching_nums.iter().filter_map(|n|{
                match n.len() {
                    0 => None,
                    _ => Some(String::from_iter(n).parse::<i64>().expect("should be a number")),
                }
            }).product::<i64>()
        } else { 0 }
    }).sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
