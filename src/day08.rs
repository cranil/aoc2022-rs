use crate::{
    grid::Grid,
    utils::{main, test},
};

fn get_contents(filename: &str) -> Grid<i32> {
    let lines = crate::utils::read_lines(filename);
    let mut grid = Grid::<i32>::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c.to_string().parse::<i32>().unwrap());
        }
    }
    return grid;
}

fn part2(grid: &Grid<i32>) -> i64 {
    let mut score = Grid::new(grid.width, grid.height);
    let mut max_score = 0;
    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            let mut s = 1;
            for (dx, dy) in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let mut nx = x as isize + dx;
                let mut ny = y as isize + dy;
                let mut trees = 0;
                while let Some(h) = grid.at(nx as usize, ny as usize) {
                    trees += 1;
                    if h >= grid.at(x, y).unwrap() {
                        break;
                    }
                    nx += dx;
                    ny += dy;
                }
                s *= trees;
            }
            score.set(x, y, s);
            if s > max_score {
                max_score = s;
            }
        }
    }
    return max_score as i64;
}

fn part1(grid: &Grid<i32>) -> i64 {
    let mut visibility = Grid::new(grid.width, grid.height);
    for x in 0..grid.width {
        visibility.set(x, 0, true);
        visibility.set(x, grid.height - 1, true);
    }
    for y in 0..grid.height {
        visibility.set(0, y, true);
        visibility.set(grid.width - 1, y, true);
    }

    let mut visible = 2 * (grid.width + grid.height) - 4;
    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            for (dx, dy) in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let mut nx = x as isize + dx;
                let mut ny = y as isize + dy;
                let mut v = true;
                while let Some(h) = grid.at(nx as usize, ny as usize) {
                    if h >= grid.at(x, y).unwrap() {
                        v = false;
                        break;
                    }
                    nx += dx;
                    ny += dy;
                }
                if v {
                    visibility.set(x, y, true);
                    visible += 1;
                    break;
                }
            }
        }
    }
    return visible as i64;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
