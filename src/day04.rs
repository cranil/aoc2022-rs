use std::num::ParseIntError;
use crate::utils::{read_lines, main, test};

fn get_contents(filename: &str) -> Vec<String> {
    read_lines(filename)
}

struct SectionRange {
    first: usize,
    last: usize,
}

impl SectionRange {
    fn new(first: usize, last: usize) -> SectionRange {
        return SectionRange { first, last };
    }

    fn from(s: &str) -> Result<SectionRange, ParseIntError> {
        let mut iter = s.split('-');
        let first = iter.next().unwrap().parse::<usize>()?;
        let last = iter.next().unwrap().parse::<usize>()?;
        return Ok(SectionRange::new(first, last));
    }

    fn contains(&self, other: &SectionRange) -> bool {
        return other.first >= self.first && other.last <= self.last;
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        return !(self.last < other.first || other.last < self.first);
    }
}

fn section_pairs(line: &str) -> (SectionRange, SectionRange) {
    let mut iter = line.split(",");
    let pair0 = SectionRange::from(iter.next().unwrap()).unwrap();
    let pair1 = SectionRange::from(iter.next().unwrap()).unwrap();
    return (pair0, pair1);
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| section_pairs(line))
        .filter(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0))
        .count()
}

fn part2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| section_pairs(line))
        .filter(|pair| pair.0.overlaps(&pair.1))
        .count()
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, usize); 0] = [];
    pub const PART2_INPUTS: [(&str, usize); 0] = [];
}

main!();
test!();
