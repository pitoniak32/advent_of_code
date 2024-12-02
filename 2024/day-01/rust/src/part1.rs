use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let (mut first_list, mut second_list) = input.lines().fold(
        (Vec::<u32>::new(), Vec::<u32>::new()),
        |(mut first_list, mut second_list), line| {
            let parts = line.split("   ").collect::<Vec<_>>();
            let first = parts.first().expect("input should have a first value");
            let second = parts.get(1).expect("input should have a second value");
            first_list.push(
                first
                    .parse::<u32>()
                    .expect("input should be a valid integer"),
            );
            second_list.push(
                second
                    .parse::<u32>()
                    .expect("input should be a valid integer"),
            );
            (first_list, second_list)
        },
    );

    if first_list.len() != second_list.len() {
        panic!("the lists must be the same length")
    }

    first_list.sort();
    second_list.sort();

    let result = first_list
        .iter()
        .zip(second_list)
        .map(|(first, second)| first.abs_diff(second))
        .sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(process(input)?, "11");
        Ok(())
    }
}
