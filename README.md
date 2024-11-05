# rust_matrix

A simple matrix library.

## Features

- Basic operations
- Transpose
- Inverse
- Determinant
- Minors and cofactors
- Adjugate matrix
- Elementary row operations
- Reduce to RREF
- LU decomposition

## Usage

```rust
use rust_matrix::{matrix, Matrix, MatrixElement};

fn main() {
  let m1 = matrix![
      0, -3, -6, 4, 9;
      -1, -2, -1, 3, 1;
      -2, -3, 0, 3, -1;
      1, 4, 5, -9, -7;
  ];
  let m2 = matrix![
      1, 2;
      4, 5;
      7, 8;
      3, 6;
      9, -1;
  ];

  assert!(m1.to_rref().epsilon_equals(&matrix![
      1, 0, -3, 0, 5;
      0, 1, 2, 0, -3;
      0, 0, 0, 1, 0;
      0, 0, 0, 0, 0;
  ]));

  assert!(
      m1 * m2
          == matrix![
              39, -48;
              2, -3;
              -14, 0;
              -38, 15;
          ]
  );
}
```
