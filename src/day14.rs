use ahash::AHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Round,
    Square,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    // we can squeeze 4 cells into a byte by using 2 bits per cell
    cells: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Cell {
        let idx = y * self.width + x;
        let byte = self.cells[idx / 4];
        let shift = (idx % 4) * 2;
        match (byte >> shift) & 0b11 {
            0b00 => Cell::Empty,
            0b01 => Cell::Round,
            0b10 => Cell::Square,
            _ => panic!("Invalid cell"),
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let idx = y * self.width + x;
        let byte = &mut self.cells[idx / 4];
        let shift = (idx % 4) * 2;
        *byte &= !(0b11 << shift);
        *byte |= match cell {
            Cell::Empty => 0b00,
            Cell::Round => 0b01,
            Cell::Square => 0b10,
        } << shift;
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut shift = 0;
    for line in input.lines() {
        width = line.len();
        for c in line.chars() {
            if shift == 0 {
                cells.push(0);
            }
            let byte = cells.last_mut().unwrap();
            *byte |= match c {
                '.' => 0b00,
                'O' => 0b01,
                '#' => 0b10,
                _ => panic!("Invalid cell"),
            } << shift;
            shift = (shift + 2) % 8;
        }
        height += 1;
    }
    Grid {
        cells,
        width,
        height,
    }
}

fn slide_north(grid: &mut Grid) {
    for x in 0..grid.width {
        let mut run_start = 0;
        let mut num_round = 0;
        for y in 0..grid.height {
            match grid.get(x, y) {
                Cell::Empty => {}
                Cell::Round => {
                    grid.set(x, y, Cell::Empty);
                    grid.set(x, run_start + num_round, Cell::Round);
                    num_round += 1;
                }
                Cell::Square => {
                    num_round = 0;
                    run_start = y + 1;
                }
            }
        }
    }
}

fn slide_west(grid: &mut Grid) {
    for y in 0..grid.height {
        let mut run_start = 0;
        let mut num_round = 0;
        for x in 0..grid.width {
            match grid.get(x, y) {
                Cell::Empty => {}
                Cell::Round => {
                    grid.set(x, y, Cell::Empty);
                    grid.set(run_start + num_round, y, Cell::Round);
                    num_round += 1;
                }
                Cell::Square => {
                    num_round = 0;
                    run_start = x + 1;
                }
            }
        }
    }
}

fn slide_south(grid: &mut Grid) {
    for x in 0..grid.width {
        let mut run_start = grid.height - 1;
        let mut num_round = 0;
        for y in (0..grid.height).rev() {
            match grid.get(x, y) {
                Cell::Empty => {}
                Cell::Round => {
                    grid.set(x, y, Cell::Empty);
                    grid.set(x, run_start - num_round, Cell::Round);
                    num_round += 1;
                }
                Cell::Square => {
                    num_round = 0;
                    run_start = y - 1;
                }
            }
        }
    }
}

fn slide_east(grid: &mut Grid) {
    for y in 0..grid.height {
        let mut run_start = grid.width - 1;
        let mut num_round = 0;
        for x in (0..grid.width).rev() {
            match grid.get(x, y) {
                Cell::Empty => {}
                Cell::Round => {
                    grid.set(x, y, Cell::Empty);
                    grid.set(run_start - num_round, y, Cell::Round);
                    num_round += 1;
                }
                Cell::Square => {
                    num_round = 0;
                    run_start = x - 1;
                }
            }
        }
    }
}

fn spin_cycle(grid: &mut Grid) {
    slide_north(grid);
    slide_west(grid);
    slide_south(grid);
    slide_east(grid);
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
    slide_north(&mut grid);
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
