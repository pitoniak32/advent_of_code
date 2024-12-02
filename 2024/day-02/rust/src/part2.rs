use anyhow::Result;
use itertools::Itertools;

pub fn process(input: &str) -> Result<String> {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<u64>().expect("should always be a valid u64"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let num_safe = reports
        .iter()
        .filter(|report| {
            let mut safe = is_report_safe(report);

            if !safe {
                for (i, _) in report.iter().enumerate() {
                    let mut temp_report = report.to_vec();
                    temp_report.remove(i);
                    let second_check = is_report_safe(&temp_report);
                    if second_check {
                        safe = true;
                        continue;
                    }
                }
            }

            safe
        })
        .count();

    Ok(num_safe.to_string())
}

pub fn is_report_safe(report: &[u64]) -> bool {
    let mut safe = true;
    let mut neg_count = 0;
    let mut pos_count = 0;
    report.iter().tuple_windows().for_each(|(cur, next)| {
        let diff: i64 = *cur as i64 - *next as i64;
        if diff < 0 {
            neg_count += 1;
        } else {
            pos_count += 1;
        }

        let abs_diff = diff.abs();
        if !(1..=3).contains(&abs_diff) {
            safe = false;
        }
    });
    safe && ((pos_count > 0 && neg_count == 0) || (pos_count == 0 && neg_count > 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process(input)?, "4");
        Ok(())
    }
}
