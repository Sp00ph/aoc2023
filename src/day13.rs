use std::str::Lines;

struct Grid {
    // It looks like the largest patterns in the input are 17x17.
    // Integer comparisons are a lot faster than bit slice comparisons,
    // so we store each row/col in a 32 bit number, with 15+ bits of padding
    // instead of storing the rows/cols in a dense bit array. We store
    // both the row bitmaps and the transposed col bitmaps, to improve the
    // speed of col comparisons.
    rows: Vec<u32>,
    cols: Vec<u32>,
}

fn parse_grid(lines: &mut Lines) -> Grid {
    let mut rows = vec![];
    let mut cols = vec![];

    for (y, line) in lines.enumerate() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        if cols.is_empty() {
            cols.resize(line.len(), 0);
        }

        rows.push(0u32);

        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                cols[x] |= 1u32.checked_shl(y as u32).expect("grid too tall");
                rows[y] |= 1u32.checked_shl(x as u32).expect("grid too wide");
            }
        }
    }

    Grid { rows, cols }
}

fn parse_input(input: &str) -> Vec<Grid> {
    let mut lines = input.lines();
    let mut grids = vec![];
    // yikes
    while lines.clone().next().is_some() {
        grids.push(parse_grid(&mut lines));
    }
    grids
}

enum Axis {
    Vertical(u8),
    Horizontal(u8),
}

fn find_symmetry(g: &Grid) -> Axis {
    fn search(data: &[u32]) -> Option<u8> {
        for c in 1..data.len() {
            let n = c.min(data.len() - c);
            if (0..n).all(|i| data[c - i - 1] == data[c + i]) {
                return Some(c as u8);
            }
        }
        None
    }

    if let Some(i) = search(&g.cols) {
        return Axis::Vertical(i);
    }
    if let Some(i) = search(&g.rows) {
        return Axis::Horizontal(i);
    }

    unreachable!("grid without symmetry")
}

fn find_symmetry_with_error(g: &Grid) -> Axis {
    fn search(data: &[u32]) -> Option<u8> {
        for c in 1..data.len() {
            let n = c.min(data.len() - c);
            if (0..n).map(|i| (data[c - i - 1] ^ data[c + i]).count_ones()).sum::<u32>() == 1 {
                return Some(c as u8);
            }
        }
        None
    }

    if let Some(i) = search(&g.cols) {
        return Axis::Vertical(i);
    }
    if let Some(i) = search(&g.rows) {
        return Axis::Horizontal(i);
    }

    unreachable!("grid without symmetry")
}

pub fn part1(input: &str) -> String {
    let grids = parse_input(input);
    grids
        .iter()
        .map(find_symmetry)
        .map(|axis| match axis {
            Axis::Vertical(col) => col as usize,
            Axis::Horizontal(row) => (row as usize) * 100,
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let grids = parse_input(input);
    grids
        .iter()
        .map(find_symmetry_with_error)
        .map(|axis| match axis {
            Axis::Vertical(col) => col as usize,
            Axis::Horizontal(row) => (row as usize) * 100,
        })
        .sum::<usize>()
        .to_string()
}
