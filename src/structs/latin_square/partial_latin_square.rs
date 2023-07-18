use super::LatinSquare;

#[derive(Debug, Clone, PartialEq)]
pub struct PartialLatinSquare(pub Vec<Vec<usize>>);

// Takes a partial latin square, and recursively creates the next partial latin squares,
// until there are only full latin squares left.
// TODO: May run faster if split up the cases where col = n or not?
pub fn latin_square_recursion(n: usize, partial: PartialLatinSquare) -> Vec<LatinSquare> {
    let mut result: Vec<LatinSquare> = vec![];

    let last = partial.0.last().unwrap();

    let mut col = last.len();
    let mut row = partial.0.len() - 1;

    if col == n {
        col = 0;
        row += 1;
    }

    'val: for i in 0..n {
        // Check if i exists on current row.
        if let Some(l) = partial.0.get(row) {
            if l.contains(&i) {
                continue 'val;
            }
        }

        // Check if i exist on column.
        for r in partial.0.iter().take(row) {
            if r[col] == i {
                continue 'val;
            }
        }

        // Add i to the partial latin square.
        let mut p = partial.0.clone();

        if col == 0 {
            p.push(vec![i]);
        } else {
            p.last_mut().unwrap().push(i);

            if row == n - 1 && col == n - 1 {
                return vec![LatinSquare(p)];
            }
        }

        let p = PartialLatinSquare(p);

        // Recursive call.
        result.append(&mut latin_square_recursion(n, p));
    }

    result
}
