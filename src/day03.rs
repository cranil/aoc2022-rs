use crate::utils::{main, read_lines, test};

fn get_contents(filename: &str) -> Vec<String> {
    read_lines(filename)
}

fn get_index(c: char) -> usize {
    match c {
        'a'..='z' => return (c as u32 - 'a' as u32) as usize,
        'A'..='Z' => return (c as u32 - 'A' as u32) as usize + 26,
        _ => panic!("Invalid char"),
    }
}

fn get_priority(c: char) -> usize {
    return get_index(c) + 1;
}

fn get_char(n: usize) -> char {
    match n {
        0..=25 => return (n as u8 + 'a' as u8) as char,
        26..=51 => return (n as u8 - 26 + 'A' as u8) as char,
        _ => panic!("Invalid index"),
    }
}

fn part1(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut lst = vec![0; 52];
    for line in lines {
        let len = line.len() / 2;
        for i in 0..len {
            let c = line.chars().nth(i).unwrap();
            let n = get_index(c);
            lst[n] = 1;
        }
        for i in len..line.len() {
            let c = line.chars().nth(i).unwrap();
            let n = get_index(c);
            if lst[n] == 1 {
                sum += get_priority(c);
                lst[n] += 1;
            }
        }
        lst.iter_mut().for_each(|x| *x = 0);
    }
    return sum;
}

fn part2(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut set1 = vec![0; 52];
    let mut set2 = vec![0; 52];
    let mut set3 = vec![0; 52];
    for sets in lines.chunks(3) {
        if sets.len() < 3 {
            panic!("Invalid input");
        }
        let line = &sets[0];
        for c in line.chars() {
            let n = get_index(c);
            set1[n] += 1;
        }
        let line = &sets[1];
        for c in line.chars() {
            let n = get_index(c);
            set2[n] += 1;
        }
        let line = &sets[2];
        for c in line.chars() {
            let n = get_index(c);
            set3[n] += 1;
        }
        for i in 0..52 {
            if set1[i] > 0 && set2[i] > 0 && set3[i] > 0 {
                sum += i + 1;
            }
        }

        set1.iter_mut().for_each(|x| *x = 0);
        set2.iter_mut().for_each(|x| *x = 0);
        set3.iter_mut().for_each(|x| *x = 0);
    }
    return sum;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, usize); 0] = [];
    pub const PART2_INPUTS: [(&str, usize); 0] = [];
}

main!();
test!();
