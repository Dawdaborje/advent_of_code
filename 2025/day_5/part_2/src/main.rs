use std::fs;

#[derive(Debug)]
struct IntIntervalModel {
    start: u64,
    end: u64,
}

fn main() {
    let input = fs::read_to_string("src/puzzle_input.txt").unwrap();
    let mut int_val_vec: Vec<IntIntervalModel> = vec![];
    let mut total: u128 = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }

        let parts: Vec<&str> = line.split('-').collect();
        int_val_vec.push(IntIntervalModel {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        });
    }

    int_val_vec.sort_by_key(|i| i.start);

    let mut merged: Vec<(u64, u64)> = vec![];

    for interval in int_val_vec {
        if let Some((_ms, me)) = merged.last_mut() {
            if interval.start <= *me + 1 {
                *me = (*me).max(interval.end);
                continue;
            }
        }
        merged.push((interval.start, interval.end));
    }

    for (s, e) in merged {
        total += (e - s + 1) as u128;
    }

    println!("{}", total);
}
