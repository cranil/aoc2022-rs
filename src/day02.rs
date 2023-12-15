use crate::utils::{main, read_lines, test};

#[derive(Clone, Copy, Debug)]
enum RPS {
    Rock = (1),
    Paper = (2),
    Scissors = (3),
}

fn get_contents(filename: &str) -> Vec<String> {
    read_lines(filename)
}

impl RPS {
    fn beats(&self, other: &RPS) -> i32 {
        match self {
            RPS::Rock => match other {
                RPS::Rock => 3,
                RPS::Paper => 0,
                RPS::Scissors => 6,
            },
            RPS::Paper => match other {
                RPS::Rock => 6,
                RPS::Paper => 3,
                RPS::Scissors => 0,
            },
            RPS::Scissors => match other {
                RPS::Rock => 0,
                RPS::Paper => 6,
                RPS::Scissors => 3,
            },
        }
    }
}

fn get_play(c: &str) -> RPS {
    match c {
        "A" | "X" => return RPS::Rock,
        "B" | "Y" => return RPS::Paper,
        "C" | "Z" => return RPS::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn find_play(c: &str, other: &RPS) -> RPS {
    let outcome = match c {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Invalid input"),
    };
    match other {
        RPS::Rock => match outcome {
            0 => return RPS::Scissors,
            3 => return RPS::Rock,
            6 => return RPS::Paper,
            _ => panic!("Invalid input"),
        },
        RPS::Paper => match outcome {
            0 => return RPS::Rock,
            3 => return RPS::Paper,
            6 => return RPS::Scissors,
            _ => panic!("Invalid input"),
        },
        RPS::Scissors => match outcome {
            0 => return RPS::Paper,
            3 => return RPS::Scissors,
            6 => return RPS::Rock,
            _ => panic!("Invalid input"),
        },
    }
}

fn get_round(line: &str) -> (RPS, RPS) {
    let mut iter = line.split_ascii_whitespace();
    let mut res = (RPS::Rock, RPS::Rock);
    res.0 = get_play(
        iter.next()
            .expect(format!("Invalid input: {}", line).as_str()),
    );
    res.1 = find_play(
        iter.next()
            .expect(format!("Invalid input: {}", line).as_str()),
        &res.0,
    );
    return res;
}

fn get_score(round: (RPS, RPS)) -> i32 {
    let mut score = 0;
    score += round.1.beats(&round.0);
    score += round.1.clone() as i32;
    return score;
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut score = 0;
    for line in lines {
        let round = (get_play(&line[0..1]), get_play(&line[2..3]));
        score += get_score(round);
    }
    return score;
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut score = 0;
    for line in lines {
        let round = get_round(line);
        score += get_score(round);
    }
    return score;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i32); 0] = [];
    pub const PART2_INPUTS: [(&str, i32); 0] = [];
}

main!();
test!();
