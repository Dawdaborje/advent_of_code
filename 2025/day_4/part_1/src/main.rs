use std::fs;

fn main() {
    let input = fs::read_to_string("src/puzzle_input.txt").unwrap();

    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect();

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

    let mut accessible = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut count = 0;

            for (dr, dc) in neighbors {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    if grid[nr as usize][nc as usize] == '@' {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                accessible += 1;
            }
        }
    }

    println!("{}", accessible);
}
