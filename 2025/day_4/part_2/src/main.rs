use std::fs;

fn main() {
    let input = fs::read_to_string("src/puzzle_input.txt").unwrap();

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect();

    if grid.is_empty() {
        println!("0");
        return;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total_removed = 0usize;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut adj_count = 0;
                for (dr, dc) in neighbors.iter() {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if grid[nr as usize][nc as usize] == '@' {
                            adj_count += 1;
                        }
                    }
                }

                if adj_count < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove.iter() {
            grid[*r][*c] = '.';
        }
        total_removed += to_remove.len();
    }

    println!("{}", total_removed);
}
