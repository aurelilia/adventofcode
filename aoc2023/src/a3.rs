use std::{collections::HashSet, ops::Range};

#[derive(Debug)]
struct PartNum {
    x: Range<usize>,
    y: usize,
    num: usize,
}

pub fn a3_1() {
    let mut sum = 0;

    let lines = crate::lines(3);
    let numbers = build_nums(&lines);

    let grid: Vec<_> = lines
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();

    let is_part = |x: isize, y: isize| -> bool {
        if x < 0 || x as usize >= grid[0].len() || y < 0 || y as usize >= grid.len() {
            return false;
        }
        let ch = grid[y as usize][x as usize];
        !ch.is_ascii_digit() && ch != '.'
    };

    let has_adj = |x: isize, y: isize| -> bool {
        is_part(x - 1, y - 1)
            || is_part(x - 1, y)
            || is_part(x - 1, y + 1)
            || is_part(x, y - 1)
            || is_part(x, y + 1)
            || is_part(x + 1, y - 1)
            || is_part(x + 1, y)
            || is_part(x + 1, y + 1)
    };

    'n: for num in numbers {
        for x in num.x.clone() {
            if has_adj(x as isize, num.y as isize) {
                sum += num.num;
                continue 'n;
            }
        }
    }

    println!("{sum}");
}

pub fn a3_2() {
    let mut sum = 0;

    let lines = crate::lines(3);
    let numbers = build_nums(&lines);

    let grid: Vec<_> = lines
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();

    let num_at = |x: isize, y: isize| -> Option<usize> {
        if x < 0 || x as usize >= grid[0].len() || y < 0 || y as usize >= grid.len() {
            return None;
        }
        numbers
            .iter()
            .find(|n| n.y == y as usize && n.x.contains(&(x as usize)))
            .map(|n| n.num)
    };

    let get_numbers = |x: isize, y: isize| -> HashSet<usize> {
        let parts = &[
            num_at(x - 1, y - 1),
            num_at(x - 1, y),
            num_at(x - 1, y + 1),
            num_at(x, y - 1),
            num_at(x, y + 1),
            num_at(x + 1, y - 1),
            num_at(x + 1, y),
            num_at(x + 1, y + 1),
        ];
        parts.iter().filter_map(|t| *t).collect()
    };

    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate().filter(|(_, c)| **c == '*') {
            let numbers = get_numbers(x as isize, y as isize);
            if numbers.len() != 2 {
                continue;
            }
            let mut iter = numbers.iter();
            sum += iter.next().unwrap() * iter.next().unwrap();
        }
    }

    println!("{sum}");
}

fn build_nums(input: &[String]) -> Vec<PartNum> {
    let mut numbers = Vec::new();
    for (y, line) in input.iter().enumerate() {
        let mut current = String::new();
        for (i, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                current.push(char);
            } else if !current.is_empty() {
                let num = current.parse().unwrap();
                numbers.push(PartNum {
                    x: (i - current.len())..i,
                    y,
                    num,
                });
                current.clear();
            }
        }
        if !current.is_empty() {
            let num = current.parse().unwrap();
            numbers.push(PartNum {
                x: (line.len() - current.len())..line.len(),
                y,
                num,
            });
            current.clear();
        }
    }
    numbers
}
