use std::fmt;

use ahash::AHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Round,
    Square,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x] = cell;
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.cells[y * self.width + x] {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Round => write!(f, "O")?,
                    Cell::Square => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        width = line.len();
        for c in line.chars() {
            match c {
                '.' => cells.push(Cell::Empty),
                'O' => cells.push(Cell::Round),
                '#' => cells.push(Cell::Square),
                _ => panic!("Invalid cell"),
            }
        }
        height += 1;
    }
    Grid {
        cells,
        width,
        height,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    West,
    South,
    East,
}

fn slide(grid: &mut Grid, dir: Dir) {
    fn helper(
        grid: &mut Grid,
        outer_len: usize,
        inner_len: usize,
        cell: impl Fn(&Grid, usize, usize) -> Cell,
        mut slide: impl FnMut(&mut Grid, usize, usize, usize, usize),
    ) {
        for i in 0..outer_len {
            let mut start_of_run = 0;
            let mut rounds_in_run = 0;
            for j in 0..inner_len {
                let cell = cell(&*grid, i, j);
                match cell {
                    Cell::Empty => continue,
                    Cell::Round => rounds_in_run += 1,
                    Cell::Square => {
                        if rounds_in_run > 0 && rounds_in_run < j - start_of_run {
                            slide(grid, rounds_in_run, start_of_run, i, j);
                        }
                        start_of_run = j + 1;
                        rounds_in_run = 0;
                    }
                }
            }

            if rounds_in_run > 0 && rounds_in_run < inner_len - start_of_run {
                slide(grid, rounds_in_run, start_of_run, i, inner_len);
            }
        }
    }

    let (w, h) = (grid.width, grid.height);
    match dir {
        Dir::North => helper(
            &mut *grid,
            w,
            h,
            |g, col, row| g.get(col, row),
            |g, rounds_in_run, start_of_run, col, row| {
                for i in 0..rounds_in_run {
                    g.set(col, start_of_run + i, Cell::Round);
                }
                for i in rounds_in_run..row - start_of_run {
                    g.set(col, start_of_run + i, Cell::Empty);
                }
            },
        ),
        Dir::West => helper(
            &mut *grid,
            h,
            w,
            |g, row, col| g.get(col, row),
            |g, rounds_in_run, start_of_run, row, col| {
                for i in 0..rounds_in_run {
                    g.set(start_of_run + i, row, Cell::Round);
                }
                for i in rounds_in_run..col - start_of_run {
                    g.set(start_of_run + i, row, Cell::Empty);
                }
            },
        ),
        Dir::South => helper(
            &mut *grid,
            w,
            h,
            |g, col, row| g.get(col, h - row - 1),
            |g, rounds_in_run, start_of_run, col, row| {
                for i in 0..rounds_in_run {
                    g.set(col, h - start_of_run - i - 1, Cell::Round);
                }
                for i in rounds_in_run..row - start_of_run {
                    g.set(col, h - start_of_run - i - 1, Cell::Empty);
                }
            },
        ),
        Dir::East => helper(
            &mut *grid,
            h,
            w,
            |g, row, col| g.get(w - col - 1, row),
            |g, rounds_in_run, start_of_run, row, col| {
                for i in 0..rounds_in_run {
                    g.set(w - start_of_run - i - 1, row, Cell::Round);
                }
                for i in rounds_in_run..col - start_of_run {
                    g.set(w - start_of_run - i - 1, row, Cell::Empty);
                }
            },
        ),
    }
}

fn spin_cycle(grid: &mut Grid) {
    slide(grid, Dir::North);
    slide(grid, Dir::West);
    slide(grid, Dir::South);
    slide(grid, Dir::East);
}

fn total_load(grid: &Grid) -> usize {
    let mut total = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == Cell::Round {
                total += grid.height - y;
            }
        }
    }
    total
}

pub fn part1(input: &str) -> String {
    let mut grid = parse_grid(input);
    slide(&mut grid, Dir::North);
    total_load(&grid).to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = parse_grid(input);
    let mut seen = AHashMap::from([(grid.clone(), 0)]);

    for i in 1usize.. {
        spin_cycle(&mut grid);
        if let Some(&prev) = seen.get(&grid) {
            let cycle_len = i - prev;
            let remaining = (1_000_000_000 - i) % cycle_len;
            for _ in 0..remaining {
                spin_cycle(&mut grid);
            }
            break;
        }
        seen.insert(grid.clone(), i);
    }

    total_load(&grid).to_string()
}
