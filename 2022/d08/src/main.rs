use std::collections::HashSet;

fn part_1() {
    let input = include_bytes!("../input");
    let grid: Vec<_> = input.split(|b| b == &b'\n').collect();
    let mut seen: HashSet<usize> = HashSet::default(); // store inlined location

    for i in 1..grid.len() - 1 {
        seen.extend(
            (1..grid.len() - 1)
                .scan(grid[0][i], |max, y| match (*max, grid[y][i]) {
                    (m, element) if element > m => {
                        *max = element;
                        Some(Some(i + y * grid.len()))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..grid.len() - 1)
                .scan(grid[grid.len() - 1][i], |max, y| {
                    match (*max, grid[grid.len() - y][i]) {
                        (m, element) if element > m => {
                            *max = element;
                            Some(Some(i + (grid.len() - y) * grid.len()))
                        }
                        (m, _) if m >= b'9' => None,
                        _ => Some(None),
                    }
                })
                .flatten(),
        );

        seen.extend(
            (1..grid.len() - 1)
                .scan(grid[i][0], |max, y| match (*max, grid[i][y]) {
                    (m, element) if element > m => {
                        *max = element;
                        Some(Some(y + i * grid.len()))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..grid.len() - 1)
                .scan(grid[i][grid.len() - 1], |max, y| {
                    match (*max, grid[i][grid.len() - y]) {
                        (m, element) if element > m => {
                            *max = element;
                            Some(Some(grid.len() - y + i * grid.len()))
                        }
                        (m, _) if m >= b'9' => None,
                        _ => Some(None),
                    }
                })
                .flatten(),
        );
    }

    let ntree_border = (grid.len() - 1) * 4;
    println!("{}", seen.len() + ntree_border);
}

fn main() {
    part_1();
}
