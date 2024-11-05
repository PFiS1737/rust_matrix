use std::fmt::{self, Display, Formatter};

use crate::{Matrix, MatrixElement};

#[derive(Debug, Clone)]
struct MatrixElementDisplay {
    is_negative: bool,
    str: String,
}

impl MatrixElementDisplay {
    fn new(ele: MatrixElement) -> Self {
        Self {
            is_negative: ele.is_negative(),
            str: if ele.is_integer() {
                format!("{}", ele).trim_start_matches('-').to_string()
            } else {
                format!("{:.14}", ele)
                    .trim_start_matches('-')
                    .trim_end_matches('0')
                    .to_string()
            },
        }
    }
}

impl Display for Matrix {
    // TODO: 优化
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let space = 3;

        let m = self.rows_number;

        let cols = self
            .as_cols()
            .into_iter()
            .map(|col| col.into_iter().map(MatrixElementDisplay::new).collect())
            .map(pad_strings)
            .collect::<Vec<Vec<_>>>();

        for row_index in (-1_isize)..=(m as isize) {
            let total_length = cols
                .iter()
                .map(|col| col[0].str.len() + space)
                .sum::<usize>()
                + space;

            if row_index == -1 {
                writeln!(f, "┌{:total_length$}┐", "")?;
            } else if row_index == m as isize {
                write!(f, "└{:total_length$}┘", "")?;
            } else {
                write!(f, "│")?;

                for col in cols.iter() {
                    let item = col[row_index as usize].clone();

                    write!(
                        f,
                        "{}{}",
                        if item.is_negative {
                            " ".repeat(space - 1) + "-"
                        } else {
                            " ".repeat(space)
                        },
                        item.str
                    )?;
                }

                writeln!(f, "{:space$}│", "")?;
            }
        }

        Ok(())
    }
}

fn pad_strings(vec: Vec<MatrixElementDisplay>) -> Vec<MatrixElementDisplay> {
    let max_len = vec.iter().map(|e| e.str.len()).max().unwrap_or(0);

    vec.into_iter()
        .map(|e| MatrixElementDisplay {
            str: if e.str.len() < max_len {
                format!("{:max_len$}", e.str)
            } else {
                e.str
            },
            ..e
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix, MatrixElement};

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", matrix![1, 2, 3; 4, 5, 6]),
            r"
┌               ┐
│   1   2   3   │
│   4   5   6   │
└               ┘
"
            .trim()
            .to_string()
        );
        assert_eq!(
            format!("{}", matrix![1, 2, 3, 4, 5, 6]),
            r"
┌                           ┐
│   1   2   3   4   5   6   │
└                           ┘
"
            .trim()
            .to_string()
        );
        assert_eq!(
            format!("{}", matrix![1; 2; 3; 4; 5; 6]),
            r"
┌       ┐
│   1   │
│   2   │
│   3   │
│   4   │
│   5   │
│   6   │
└       ┘
"
            .trim()
            .to_string()
        );

        assert_eq!(
            format!(
                "{}",
                matrix![
                    -1, 2, -3, 1.0/6.0;
                    1.0/3.0, -1.0/4.0, 1.0/5.0, -4;
                ]
            ),
            r"
┌                                                      ┐
│  -1                  2     -3     0.16666666666667   │
│   0.33333333333333  -0.25   0.2  -4                  │
└                                                      ┘
"
            .trim()
            .to_string()
        );
        assert_eq!(
            format!(
                "{}",
                matrix![
                    -39, -48;
                    2, -3;
                    -14, 0;
                    -38, 15;
                ]
            ),
            r"
┌             ┐
│  -39  -48   │
│   2   -3    │
│  -14   0    │
│  -38   15   │
└             ┘
"
            .trim()
            .to_string()
        );
    }
}
