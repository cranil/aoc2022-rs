use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<Vec<i64>> {
    let content = crate::utils::read_all(filename);
    content
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn part1(bags: &Vec<Vec<i64>>) -> i64 {
    bags.iter().map(|bag| bag.iter().sum()).max().unwrap()
}

fn part2(bags: &Vec<Vec<i64>>) -> i64 {
    let mut calories = bags
        .iter()
        .map(|bag| bag.iter().sum())
        .collect::<Vec<i64>>();
    calories.sort_by(|a, b| b.cmp(a));
    return calories[0] + calories[1] + calories[2];
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 1] = [("day09_example.txt", 127)];
    pub const PART2_INPUTS: [(&str, i64); 1] = [("day09_example.txt", 62)];
}

main!();
test!();
