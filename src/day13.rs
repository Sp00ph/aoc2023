use std::str::Lines;

use smallvec::SmallVec;

struct Grid {
    // It looks like the largest patterns in the input are 17x17.
    // Integer comparisons are a lot faster than bit slice comparisons,
    // so we store each row/col in a 32 bit number, with 15+ bits of padding
    // instead of storing the rows/cols in a dense bit array. We store
    // both the row bitmaps and the transposed col bitmaps, to improve the
    // speed of col comparisons.
    rows: SmallVec<[u32; 20]>,
    cols: SmallVec<[u32; 20]>,
}

fn parse_grid(lines: &mut Lines) -> Grid {
    let mut rows = SmallVec::new();
    let mut cols = SmallVec::new();

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

fn parse_input(input: &str) -> impl Iterator<Item = Grid> + '_ {
    let mut lines = input.lines();
    std::iter::from_fn(move || {
        if lines.clone().next().is_some() {
            Some(parse_grid(&mut lines))
        } else {
            None
        }
    })
}

enum Axis {
    Vertical(u8),
    Horizontal(u8),
}

fn search(data: &[u32], bits_to_flip: u32) -> Option<u8> {
    for c in 1..data.len() {
        let n = c.min(data.len() - c);
        if (0..n)
            .map(|i| (data[c - i - 1] ^ data[c + i]).count_ones())
            .sum::<u32>()
            == bits_to_flip as u32
        {
            return Some(c as u8);
        }
    }
    None
}

fn find_symmetry(g: &Grid, bits_to_flip: u32) -> Axis {
    if let Some(i) = search(&g.cols, bits_to_flip) {
        return Axis::Vertical(i);
    }
    if let Some(i) = search(&g.rows, bits_to_flip) {
        return Axis::Horizontal(i);
    }

    unreachable!("grid without symmetry")
}

pub fn part1(input: &str) -> String {
    let grids = parse_input(input);
    grids
        .map(|g| match find_symmetry(&g, 0) {
            Axis::Vertical(col) => col as usize,
            Axis::Horizontal(row) => (row as usize) * 100,
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let grids = parse_input(input);
    grids
        .map(|g| match find_symmetry(&g, 1) {
            Axis::Vertical(col) => col as usize,
            Axis::Horizontal(row) => (row as usize) * 100,
        })
        .sum::<usize>()
        .to_string()
}
