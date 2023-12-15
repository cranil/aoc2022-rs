use std::collections::HashMap;

use crate::{
    grid::Grid,
    utils::{main, read_lines, test},
};

fn get_contents(filename: &str) -> Vec<(i64, i64)> {
    let mut v = Vec::new();
    for line in read_lines(filename) {
        let mut iter = line.split(" ");
        let dir = iter.next().unwrap();
        let count = iter.next().unwrap().parse::<i64>().unwrap();
        let p = match dir {
            "L" => (-count, 0),
            "R" => (count, 0),
            "U" => (0, count),
            "D" => (0, -count),
            _ => panic!("Unknown direction {}", dir),
        };
        v.push(p);
    }
    return v;
}

#[derive(Debug, Clone, Copy)]
struct Knot {
    h_pos: (i64, i64),
    t_pos: (i64, i64),
}

fn move_knots(knots: &mut Vec<Knot>, motion: (i64, i64)) {
    let mut motion = motion;
    for knot in knots.iter_mut() {
        let t_pos = knot.t_pos;
        move_knot(knot, &motion);
        motion = (knot.t_pos.0 - t_pos.0, knot.t_pos.1 - t_pos.1);
    }
}

fn move_knot(knot: &mut Knot, motion: &(i64, i64)) {
    let mut h_pos = knot.h_pos;
    let mut t_pos = knot.t_pos;
    let (dx, dy) = motion;
    let (x, y) = h_pos;
    h_pos = (x + dx, y + dy);
    let (mut xt, mut yt) = t_pos;
    let (dxt, dyt) = (h_pos.0 - xt, h_pos.1 - yt);
    match (dxt, dyt) {
        (0, 0) => {}
        (0, 1) => {}
        (1, 0) => {}
        (1, 1) => {}
        (0, -1) => {}
        (-1, 0) => {}
        (-1, -1) => {}
        (1, -1) => {}
        (-1, 1) => {}
        (0, 2) => {
            yt += 1;
        }
        (0, -2) => {
            yt -= 1;
        }
        (2, 0) => {
            xt += 1;
        }
        (-2, 0) => {
            xt -= 1;
        }
        (1, 2) => {
            xt += 1;
            yt += 1;
        }
        (1, -2) => {
            xt += 1;
            yt -= 1;
        }
        (-1, 2) => {
            xt -= 1;
            yt += 1;
        }
        (-1, -2) => {
            xt -= 1;
            yt -= 1;
        }
        (2, 1) => {
            xt += 1;
            yt += 1;
        }
        (2, -1) => {
            xt += 1;
            yt -= 1;
        }
        (-2, 1) => {
            xt -= 1;
            yt += 1;
        }
        (-2, -1) => {
            xt -= 1;
            yt -= 1;
        }
        (2, 2) => {
            xt += 1;
            yt += 1;
        }
        (2, -2) => {
            xt += 1;
            yt -= 1;
        }
        (-2, 2) => {
            xt -= 1;
            yt += 1;
        }
        (-2, -2) => {
            xt -= 1;
            yt -= 1;
        }
        _ => panic!("Invalid direction"),
    }
    t_pos = (xt, yt);

    knot.h_pos = h_pos;
    knot.t_pos = t_pos;
}

fn part1(motions: &Vec<(i64, i64)>) -> i64 {
    let mut trail = HashMap::new();
    let h_pos = (0, 0);
    let t_pos = (0, 0);
    let mut knot = Knot { h_pos, t_pos };
    trail.insert(h_pos, 1);
    for (dx, dy) in motions.iter() {
        let n = dx.abs() + dy.abs();
        let dir = (dx / n, dy / n);
        for _ in 1..=n {
            move_knot(&mut knot, &dir);
            let t_pos = knot.t_pos;
            if let Some(v) = trail.get(&t_pos) {
                trail.insert(t_pos, v + 1);
            } else {
                trail.insert(t_pos, 1);
            }
        }
    }
    // let max_x = trail.keys().map(|(x, _)| x).max().unwrap();
    // let min_x = trail.keys().map(|(x, _)| x).min().unwrap();
    // let max_y = trail.keys().map(|(_, y)| y).max().unwrap();
    // let min_y = trail.keys().map(|(_, y)| y).min().unwrap();

    // let grid_size = ((max_x - min_x + 1), (max_y - min_y + 1));
    // let mut grid = Grid::new(grid_size.0 as usize, grid_size.1 as usize);
    // for x in 0..grid.width {
    //     for y in 0..grid.height {
    //         grid.set(x, y, '.');
    //     }
    // }
    // for (x, y) in trail.keys() {
    //     grid.set((x - min_x) as usize, (y - min_y) as usize, '#');
    // }
    // println!("{}", grid);
    trail.keys().len() as i64
}

fn part2(motions: &Vec<(i64, i64)>) -> i64 {
    let mut trail = HashMap::new();
    let mut knots = vec![
        Knot {
            h_pos: (0, 0),
            t_pos: (0, 0),
        };
        9
    ];
    for (dx, dy) in motions.iter() {
        let n = dx.abs() + dy.abs();
        let dir = (dx / n, dy / n);
        for _ in 1..=n {
            move_knots(&mut knots, dir);
            if let Some(v) = trail.get(&knots[8].t_pos) {
                trail.insert(knots[8].t_pos, v + 1);
            } else {
                trail.insert(knots[8].t_pos, 1);
            }
        }
    }
    trail.keys().len() as i64
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
