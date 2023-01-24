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

fn part_2() {
    let input = include_bytes!("../input");
    let grid: Vec<_> = input.split(|b| b == &b'\n').collect();

    let mut max_scenic_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let left_to_center = (1..j)
                .map(|index| grid[j - index][i])
                .position(|height| height >= grid[j][i])
                .unwrap_or_else(|| j.wrapping_sub(1))
                .wrapping_add(1);

            let center_to_right = (j + 1..grid.len())
                .map(|index| grid[index][i])
                .position(|height| height >= grid[j][i])
                .unwrap_or_else(|| (grid.len()-j).wrapping_sub(2))
                .wrapping_add(1);

            let top_to_center = (1..i)
                .map(|index| grid[j][i - index])
                .position(|height| height >= grid[j][i])
                .unwrap_or_else(|| i.wrapping_sub(1))
                .wrapping_add(1);

            let center_to_bottom = (i + 1..grid.len())
                .map(|index| grid[j][index])
                .position(|height| height >= grid[j][i])
                .unwrap_or_else(|| (grid.len()-i).wrapping_sub(2))
                .wrapping_add(1);

            max_scenic_score = max_scenic_score
                .max(left_to_center * center_to_right * top_to_center * center_to_bottom);
        }
    }

    println!("{}", max_scenic_score);
}

fn main() {
    part_1();
    part_2();
}
