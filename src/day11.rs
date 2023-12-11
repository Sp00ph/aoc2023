use bit_set::BitSet;
use bit_vec::BitVec;

#[derive(Debug)]
struct Grid {
    planets: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

fn parse_input(input: &str) -> Grid {
    let mut planets = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        height += 1;
        width = line.len();
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                planets.push((y, x));
            }
        }
    }
    Grid {
        planets,
        width,
        height,
    }
}

fn empty_rows_and_cols(grid: &Grid) -> (Vec<usize>, Vec<usize>) {
    let mut rows = BitSet::from_bit_vec(BitVec::from_elem(grid.height, true));
    let mut cols = BitSet::from_bit_vec(BitVec::from_elem(grid.width, true));

    for &(y, x) in &grid.planets {
        rows.remove(y);
        cols.remove(x);
    }

    (
        rows.iter().collect::<Vec<_>>(),
        cols.iter().collect::<Vec<_>>(),
    )
}

fn apply_offsets(
    planets: &mut [(usize, usize)],
    empty_rows: &[usize],
    empty_cols: &[usize],
    factor: usize,
) {
    for p in &mut *planets {
        let (y, x) = *p;
        // We could sort the empty rows and cols and use binary search here, but the
        // number of empty rows and columns is small enough that it's not worth it.
        let i = empty_rows.iter().filter(|&&row| row < y).count();
        let j = empty_cols.iter().filter(|&&col| col < x).count();
        p.0 += i * factor;
        p.1 += j * factor;
    }
}

fn planet_pairs(g: &Grid) -> impl Iterator<Item = [(usize, usize); 2]> + '_ {
    g.planets
        .iter()
        .enumerate()
        .flat_map(move |(i, &p1)| g.planets[i + 1..].iter().map(move |&p2| [p1, p2]))
}

fn dist((y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> usize {
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);
    dx + dy
}

pub fn part1(input: &str) -> String {
    let mut grid = parse_input(input);
    let (rows, cols) = empty_rows_and_cols(&grid);
    apply_offsets(&mut grid.planets, &rows, &cols, 1);

    planet_pairs(&grid)
        .map(|[p1, p2]| dist(p1, p2))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = parse_input(input);
    let (rows, cols) = empty_rows_and_cols(&grid);
    apply_offsets(&mut grid.planets, &rows, &cols, 999_999);

    planet_pairs(&grid)
        .map(|[p1, p2]| dist(p1, p2))
        .sum::<usize>()
        .to_string()
}
