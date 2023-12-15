use std::fmt::Display;

pub struct Grid3D<T> {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Grid3D<T> {
    pub fn new(width: usize, height: usize, depth: usize, value: T) -> Self {
        let data = vec![value; width * height * depth];
        return Self {
            width,
            height,
            depth,
            data,
        };
    }

    pub fn at(&self, x: usize, y: usize, z: usize) -> Option<&T> {
        if x >= self.width || y >= self.height || z >= self.depth {
            return None;
        }
        return Some(&self.data[x + y * self.width + z * self.width * self.height]);
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        if x >= self.width || y >= self.height || z >= self.depth {
            panic!("Invalid index");
        }
        self.data[x + y * self.width + z * self.width * self.height] = value;
    }

    pub fn iter(&self) -> Grid3DIterator<T> {
        return Grid3DIterator::new(self);
    }
}

impl<T: Display + Default + Clone> Grid3D<T> {
    pub fn print(&self) {
        for z in 0..self.depth {
            println!("z = {}", z);
            for y in 0..self.height {
                for x in 0..self.width {
                    print!("{}", self.at(x, y, z).unwrap());
                }
                println!();
            }
            println!();
        }
    }
}

impl std::fmt::Display for Grid3D<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in 0..self.depth {
            writeln!(f, "z = {}", z)?;
            for y in 0..self.height {
                for x in 0..self.width {
                    write!(f, "{}", self.at(x, y, z).unwrap())?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid3D<i64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in 0..self.depth {
            writeln!(f, "z = {}", z)?;
            for y in 0..self.height {
                for x in 0..self.width {
                    write!(f, "{:^5} ", self.at(x, y, z).unwrap())?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid3D<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in 0..self.depth {
            writeln!(f, "z = {}", z)?;
            for y in 0..self.height {
                for x in 0..self.width {
                    write!(f, "{}", if *self.at(x, y, z).unwrap() { '#' } else { '.' })?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Grid3DIterator<'a, T> {
    grid: &'a Grid3D<T>,
    x: usize,
    y: usize,
    z: usize,
}

impl<'a, T> Grid3DIterator<'a, T> {
    fn new(grid: &'a Grid3D<T>) -> Self {
        return Self {
            grid,
            x: 0,
            y: 0,
            z: 0,
        };
    }
}

impl<T: Clone> Iterator for Grid3DIterator<'_, T> {
    type Item = (usize, usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.z >= self.grid.depth {
            return None;
        }
        let item = (
            self.x,
            self.y,
            self.z,
            self.grid.at(self.x, self.y, self.z).unwrap().clone(),
        );
        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
            if self.y >= self.grid.height {
                self.y = 0;
                self.z += 1;
            }
        }
        return Some(item);
    }
}

pub struct Neighbors3DIterator<'a, T> {
    grid: &'a Grid3D<T>,
    neighbor_size: (usize, usize, usize),
    x: usize,
    y: usize,
    z: usize,
}

impl<'a, T> Neighbors3DIterator<'a, T> {
    fn new(grid: &'a Grid3D<T>, neighbor_size: (usize, usize, usize)) -> Self {
        return Self {
            grid,
            neighbor_size,
            x: 0,
            y: 0,
            z: 0,
        };
    }
}

impl<T: Clone> Iterator for Neighbors3DIterator<'_, T> {
    type Item = ((usize, usize, usize), Grid3D<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.z >= self.grid.depth {
            return None;
        }
        let mut grid = Grid3D::new(
            self.neighbor_size.0,
            self.neighbor_size.1,
            self.neighbor_size.2,
            self.grid.at(0, 0, 0).unwrap().clone(),
        );
        for (x, y, z, c) in self.grid.iter() {
            if x >= self.x
                && x < self.x + self.neighbor_size.0
                && y >= self.y
                && y < self.y + self.neighbor_size.1
                && z >= self.z
                && z < self.z + self.neighbor_size.2
            {
                grid.set(x - self.x, y - self.y, z - self.z, c.clone());
            }
        }
        let item = ((self.x, self.y, self.z), grid);
        self.x += 1;
        if self.x + self.neighbor_size.0 > self.grid.width {
            self.x = 0;
            self.y += 1;
            if self.y + self.neighbor_size.1 > self.grid.height {
                self.y = 0;
                self.z += 1;
            }
        }
        return Some(item);
    }
}
