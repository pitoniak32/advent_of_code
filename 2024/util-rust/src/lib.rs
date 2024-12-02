use std::fmt::Display;

pub fn print_matrix<T>(matrix: &[Vec<T>])
where
    T: Display + Clone,
{
    for row in matrix {
        for element in row {
            print!("{}", element);
        }
        println!();
    }
}

pub fn transpose_matrix<T>(image: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Copy,
{
    let ncols = image[0].len();
    (0..ncols)
        .map(|col_idx| image.iter().map(|row| row[col_idx]).collect::<Vec<T>>())
        .collect::<Vec<Vec<T>>>()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     assert_eq!(4, 4);
    // }
}
