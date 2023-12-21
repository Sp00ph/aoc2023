use ahash::AHashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Floor,
    Wall,
}

struct Grid {
    cells: Vec<Cell>,
    width: u8,
    height: u8,
    start: (u8, u8),
}

impl Grid {
    fn get(&self, x: u8, y: u8) -> Cell {
        assert!(x < self.width && y < self.height);
        self.cells[(y as usize) * (self.width as usize) + (x as usize)]
    }

    fn get_wrapping(&self, x: i16, y: i16) -> Cell {
        let x = x.rem_euclid(self.width as i16) as u8;
        let y = y.rem_euclid(self.height as i16) as u8;
        self.get(x, y)
    }
}

fn parse_input(input: &str) -> Grid {
    let mut cells = vec![];
    let mut width = 0;
    let mut height = 0u8;
    let mut start = (0, 0);
    for line in input.lines() {
        width = u8::try_from(line.len()).expect("grid too wide");
        for (i, cell) in line.bytes().enumerate() {
            let cell = match cell {
                b'.' => Cell::Floor,
                b'#' => Cell::Wall,
                b'S' => {
                    start = (i as u8, height);
                    Cell::Floor
                }
                _ => unreachable!("Invalid cell"),
            };
            cells.push(cell);
        }
        height = height.checked_add(1).expect("grid too tall");
    }
    Grid { cells, width, height, start }
}

pub fn part1(input: &str) -> String {
    let grid = parse_input(input);
    let (sx, sy) = grid.start;
    let mut accessible = AHashSet::from([(sx as i16, sy as i16)]);
    let mut next = AHashSet::new();
    for _ in 0..64 {
        for (x, y) in accessible.drain() {
            if grid.get_wrapping(x - 1, y) == Cell::Floor {
                next.insert((x - 1, y));
            }
            if grid.get_wrapping(x + 1, y) == Cell::Floor {
                next.insert((x + 1, y));
            }

            if grid.get_wrapping(x, y - 1) == Cell::Floor {
                next.insert((x, y - 1));
            }
            if grid.get_wrapping(x, y + 1) == Cell::Floor {
                next.insert((x, y + 1));
            }
        }
        std::mem::swap(&mut accessible, &mut next);
    }

    accessible.len().to_string()
}

// extrapolate the quadratic function that passes through the points
// (x0, y0), (x1, y1), (x2, y2) and return its value at x.
fn eval_lagrange(xs: [isize; 3], ys: [usize; 3], x: usize) -> usize {
    // ew
    let [x0, x1, x2] = xs.map(|x| x as i128);
    let [y0, y1, y2] = ys.map(|y| y as i128);
    let x = x as i128;

    let result = ((x - x1) * (x - x2) * y0 / ((x0 - x1) * (x0 - x2)))
        + ((x - x0) * (x - x2) * y1 / ((x1 - x0) * (x1 - x2)))
        + ((x - x0) * (x - x1) * y2 / ((x2 - x0) * (x2 - x1)));

    result as usize
}

pub fn part2(input: &str) -> String {
    let grid = parse_input(input);
    let (sx, sy) = grid.start;
    let mut accessible = AHashSet::from([(sx as i16, sy as i16)]);
    let mut next = AHashSet::new();
    // we store [f(-66), f(65), f(196)] in this array, which is
    // enough to extrapolate the quadratic function that calculates
    // f(65 + 131 * n).
    let mut values = [0; 3];
    for i in 1..=196 {
        for (x, y) in accessible.drain() {
            if grid.get_wrapping(x - 1, y) == Cell::Floor {
                next.insert((x - 1, y));
            }
            if grid.get_wrapping(x + 1, y) == Cell::Floor {
                next.insert((x + 1, y));
            }

            if grid.get_wrapping(x, y - 1) == Cell::Floor {
                next.insert((x, y - 1));
            }
            if grid.get_wrapping(x, y + 1) == Cell::Floor {
                next.insert((x, y + 1));
            }
        }
        std::mem::swap(&mut accessible, &mut next);
        match i {
            // Seems like f(-66) = f(64). I guess f is symmetric around -1?
            64 => values[0] = accessible.len(),
            65 => values[1] = accessible.len(),
            196 => values[2] = accessible.len(),
            _ => {}
        }
    }

    eval_lagrange([-66, 65, 196], values, 26501365).to_string()
}
