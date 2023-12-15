use std::collections::VecDeque;

use crate::utils::{main, read_lines, test};
use scanf::sscanf;

fn get_contents(filename: &str) -> (Vec<VecDeque<char>>, Vec<Rearrangement>) {
    let lines = read_lines(filename);
    let mut columns = vec![VecDeque::new(); 9];
    let pos = lines.iter().position(|line| line.len() == 0).unwrap_or(0);
    let initial_config = &lines[..pos];

    for line in initial_config {
        let mut count = 0_usize;
        for (i, c) in line.chars().enumerate() {
            if i % 4 == 0 && i != 0 {
                count += 1;
            }
            if c.is_alphabetic() {
                columns[count].push_back(c);
            }
        }
    }
    let rearrangents = lines
        .iter()
        .skip(pos + 1)
        .map(|line| {
            let mut from = 0_usize;
            let mut to = 0_usize;
            let mut number = 0_usize;
            if sscanf!(line.as_str(), "move {} from {} to {}", number, from, to).is_ok() {
                return Rearrangement {
                    from: from - 1,
                    to: to - 1,
                    number,
                };
            }
            panic!("Invalid input");
        })
        .collect::<Vec<Rearrangement>>();
    return (columns, rearrangents);
}

struct Rearrangement {
    from: usize,
    to: usize,
    number: usize,
}

fn part1(problem: &(Vec<VecDeque<char>>, Vec<Rearrangement>)) -> String {
    let (columns, rearrangements) = problem;
    let mut columns = columns.clone();
    for rearrangement in rearrangements {
        let Rearrangement { from, to, number } = rearrangement;
        for _ in 0..*number {
            let c = (columns[*from]).pop_front().unwrap();
            columns[*to].push_front(c);
        }
    }
    let mut result = String::new();
    for column in columns {
        result.push(column[0]);
    }
    return result;
}

fn part2(problem: &(Vec<VecDeque<char>>, Vec<Rearrangement>)) -> String {
    let (columns, rearrangements) = problem;
    let mut columns = columns.clone();
    for rearrangement in rearrangements {
        let Rearrangement { from, to, number } = rearrangement;
        let mut v = Vec::new();
        for _ in 0..*number {
            let co = (columns[*from]).pop_front();
            match co {
                Some(c) => v.push(c),
                None => panic!(
                    "{:?}\nInvalid input: move {} from {} to {}",
                    columns, number, from, to
                ),
            }
        }
        for c in v.iter().rev() {
            columns[*to].push_front(*c);
        }
    }
    let mut result = String::new();
    for column in columns {
        result.push(column[0]);
    }
    return result;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, String); 0] = [];
    pub const PART2_INPUTS: [(&str, String); 0] = [];
}

main!();
test!();
