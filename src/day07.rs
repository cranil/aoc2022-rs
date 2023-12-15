use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<String> {
    crate::utils::read_lines(filename)
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    size: i64,
    files: Vec<File>,
}

#[derive(Debug, Clone)]
enum File {
    Dir(Dir),
    File(i64),
}

fn walk(dir: &Dir) -> i64 {
    let mut total = if dir.size <= 100_000 { dir.size } else { 0 };
    for file in dir.files.iter() {
        match file {
            File::Dir(d) => {
                total += walk(d);
            }
            _ => {}
        }
    }
    return total;
}

fn get_top_dir(terminal_strings: &Vec<String>) -> Dir {
    let mut dir_stack = Vec::<Dir>::new();
    let mut total = 0;
    for line in terminal_strings.iter() {
        if line.starts_with("$ cd") {
            let dir_name = line.split(" ").nth(2).unwrap();
            if let Some(top) = dir_stack.last_mut() {
                top.size += total;
            }
            if dir_name == ".." {
                if let Some(dir) = dir_stack.pop() {
                    if let Some(top) = dir_stack.last_mut() {
                        top.size += dir.size;
                        top.files.iter_mut().for_each(|f| match f {
                            File::Dir(d) => {
                                if d.name == dir.name {
                                    *f = File::Dir(dir.clone());
                                }
                            }
                            _ => {}
                        });
                    }
                }
            } else {
                dir_stack.push(Dir {
                    name: dir_name.to_string(),
                    size: 0,
                    files: Vec::new(),
                });
            }
        } else if line.starts_with("$ ls") {
            total = 0;
        } else {
            let dir = dir_stack.last_mut().unwrap();
            match line.split_whitespace().nth(0).unwrap().parse::<i64>() {
                Ok(sz) => {
                    dir.files.push(File::File(sz));
                    if let Some(top) = dir_stack.last_mut() {
                        top.size += sz;
                    }
                }
                _ => {
                    dir.files.push(File::Dir(Dir {
                        name: line.split_whitespace().nth(1).unwrap().to_string(),
                        size: 0,
                        files: Vec::new(),
                    }));
                }
            }
        }
    }

    let mut d = None;
    while let Some(dir) = dir_stack.pop() {
        if let Some(top) = dir_stack.last_mut() {
            top.size += dir.size;
            top.files.iter_mut().for_each(|f| match f {
                File::Dir(d) => {
                    if d.name == dir.name {
                        *f = File::Dir(dir.clone());
                    }
                }
                _ => {}
            });
        } else {
            d = Some(dir);
        }
    }
    d.unwrap()
}

fn part1(terminal_strings: &Vec<String>) -> i64 {
    let top_dir = get_top_dir(terminal_strings);
    walk(&top_dir)
}

fn find_smallest_dir_to_delete(dir: &Dir, isz: i64, n: i64) -> i64 {
    let mut smallest_size = isz;
    for file in dir.files.iter() {
        match file {
            File::Dir(d) => {
                if d.size >= n {
                    if d.size < smallest_size {
                        smallest_size = d.size;
                    }
                    smallest_size = find_smallest_dir_to_delete(d, smallest_size, n);
                }
            }
            _ => {}
        }
    }
    smallest_size
}

fn part2(terminal_strings: &Vec<String>) -> i64 {
    let top_dir = get_top_dir(terminal_strings);
    let availabe_space = 70_000_000 - top_dir.size;
    let n = 30_000_000 - availabe_space;
    find_smallest_dir_to_delete(&top_dir, top_dir.size, n)
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
