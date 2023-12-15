use std::collections::VecDeque;

use crate::utils::{main, test};

#[derive(Debug, Clone)]
struct Add(i64);

#[derive(Debug, Clone)]
enum Instr {
    Noop,
    Add(Add),
}

fn get_contents(filename: &str) -> Vec<Instr> {
    let lines = crate::utils::read_lines(filename);
    return lines
        .iter()
        .map(|line| {
            let mut iter = line.split(" ");
            let instr = iter.next().unwrap();
            match instr {
                "noop" => Instr::Noop,
                "addx" => {
                    let arg = iter.next().unwrap().parse::<i64>().unwrap();
                    Instr::Add(Add(arg))
                }
                _ => panic!("Unknown instruction {}", instr),
            }
        })
        .collect();
}

fn part1(instructions: &Vec<Instr>) -> i64 {
    let mut cycle = 0;
    let mut signal_strength = Vec::new();
    let mut pc = 0;
    let mut x = 1;
    let mut n_cycles = 0;
    loop {
        let instr = instructions[pc].clone();
        match instr {
            Instr::Noop => {
                n_cycles += 1;
            }
            Instr::Add(_) => {
                n_cycles += 2;
            }
        }
        while n_cycles > 0 {
            n_cycles -= 1;
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal_strength.push(x * cycle);
            }
        }
        match instr {
            Instr::Add(Add(arg)) => {
                x += arg;
            }
            _ => {}
        }
        pc += 1;
        if pc >= instructions.len() {
            break;
        }
    }
    signal_strength.iter().sum()
}

fn move_sprite(sprite: &mut [char; 40], pos: usize) {
    for i in 0..40 {
        sprite[i] = '.';
    }
    sprite[pos - 1] = '#';
    sprite[pos] = '#';
    sprite[pos + 1] = '#';
}

fn part2(instructions: &Vec<Instr>) -> i64 {
    let mut crt = [['.'; 40]; 6];
    let mut sprite = ['.'; 40];
    let mut pos = 1;
    sprite[0] = '#';
    sprite[1] = '#';
    sprite[2] = '#';
    crt[0][0] = '#';
    let mut cycle = 0;
    let mut pc = 0;
    let mut n_cycles = 0;
    loop {
        let instr = instructions[pc].clone();
        match instr {
            Instr::Noop => {
                n_cycles += 1;
            }
            Instr::Add(_) => {
                n_cycles += 2;
            }
        }
        while n_cycles > 0 {
            let row = (cycle % 240) / 40;
            let col = (cycle % 240) % 40;
            if col as i64 == pos - 1 || col as i64 == pos || col as i64 == pos + 1 {
                crt[row][col] = '█';
            } else {
                crt[row][col] = ' ';
            }
            n_cycles -= 1;
            cycle += 1;
        }
        match instr {
            Instr::Add(Add(arg)) => {
                pos += arg;
            }
            _ => {}
        }
        pc += 1;
        if pc >= instructions.len() {
            break;
        }
    }
    for row in crt.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    0
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
