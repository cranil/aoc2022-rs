use std::collections::HashSet;

use crate::utils::{main, test};

fn get_contents(filename: &str) -> String {
    return crate::utils::read_all(filename);
}

fn part1(stream: &String) -> i64 {
    let stream = stream.chars().collect::<Vec<char>>();
    if let Some(ws) = stream
        .windows(4)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == w.len())
    {
        return ws as i64 + 4;
    }
    panic!("Invalid input");
}

fn part2(stream: &String) -> i64 {
    let stream = stream.chars().collect::<Vec<char>>();
    if let Some(ws) = stream
        .windows(14)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == w.len())
    {
        return ws as i64 + 14;
    }
    panic!("Invalid input");
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
