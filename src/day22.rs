use std::{collections::VecDeque, fmt};

use ahash::AHashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: (u16, u16, u16),
    len: u16,
    axis: Axis,
}

impl fmt::Debug for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y, z) = self.start;
        let (x2, y2, z2) = match self.axis {
            Axis::X => (x + self.len, y, z),
            Axis::Y => (x, y + self.len, z),
            Axis::Z => (x, y, z + self.len),
        };
        write!(f, "({},{},{})~({},{},{})", x, y, z, x2, y2, z2)
    }
}

impl Brick {
    fn from_start_end(start: (u16, u16, u16), end: (u16, u16, u16)) -> Self {
        let (x1, y1, z1) = start;
        let (x2, y2, z2) = end;
        let axis = if x1 != x2 {
            Axis::X
        } else if y1 != y2 {
            Axis::Y
        } else {
            Axis::Z
        };
        let len = match axis {
            Axis::X => x2.abs_diff(x1),
            Axis::Y => y2.abs_diff(y1),
            Axis::Z => z2.abs_diff(z1),
        };
        let start = (u16::min(x1, x2), u16::min(y1, y2), u16::min(z1, z2));
        Self { start, len, axis }
    }

    fn xy_overlap(self, other: Self) -> bool {
        let (x1, y1, _) = self.start;
        let (x2, y2) = match self.axis {
            Axis::X => (x1 + self.len, y1),
            Axis::Y => (x1, y1 + self.len),
            Axis::Z => (x1, y1),
        };
        let (x3, y3, _) = other.start;
        let (x4, y4) = match other.axis {
            Axis::X => (x3 + other.len, y3),
            Axis::Y => (x3, y3 + other.len),
            Axis::Z => (x3, y3),
        };
        let x_overlap = x3 <= x2 && x1 <= x4;
        let y_overlap = y3 <= y2 && y1 <= y4;
        x_overlap && y_overlap
    }

    fn overlaps(&self, other: Self) -> bool {
        let (x1, y1, z1) = self.start;
        let (x2, y2, z2) = match self.axis {
            Axis::X => (x1 + self.len, y1, z1),
            Axis::Y => (x1, y1 + self.len, z1),
            Axis::Z => (x1, y1, z1 + self.len),
        };
        let (x3, y3, z3) = other.start;
        let (x4, y4, z4) = match other.axis {
            Axis::X => (x3 + other.len, y3, z3),
            Axis::Y => (x3, y3 + other.len, z3),
            Axis::Z => (x3, y3, z3 + other.len),
        };
        let x_overlap = x3 <= x2 && x1 <= x4;
        let y_overlap = y3 <= y2 && y1 <= y4;
        let z_overlap = z3 <= z2 && z1 <= z4;
        x_overlap && y_overlap && z_overlap
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let (sx, syz) = start.split_once(',').unwrap();
            let (sy, sz) = syz.split_once(',').unwrap();
            let (ex, eyz) = end.split_once(',').unwrap();
            let (ey, ez) = eyz.split_once(',').unwrap();
            let start = (sx.parse().unwrap(), sy.parse().unwrap(), sz.parse().unwrap());
            let end = (ex.parse().unwrap(), ey.parse().unwrap(), ez.parse().unwrap());
            Brick::from_start_end(start, end)
        })
        .collect()
}

fn below(bricks: &[Brick]) -> Vec<Vec<usize>> {
    let mut below: Vec<_> = (0..bricks.len()).map(|_| vec![]).collect();
    // for each brick, find the bricks that are above and below it
    for (i, brick) in bricks.iter().enumerate() {
        for (j, other) in bricks.iter().enumerate() {
            if i == j || !brick.xy_overlap(*other) {
                continue;
            }
            if i > j {
                below[i].push(j);
            }
        }
    }
    below
}

fn fall(bricks: &mut [Brick], below: &[Vec<usize>]) {
    for i in 0..bricks.len() {
        loop {
            let mut copy = bricks[i];
            copy.start.2 -= 1;
            let is_valid = copy.start.2 > 0 && below[i].iter().all(|&j| !copy.overlaps(bricks[j]));
            if !is_valid {
                break;
            }
            bricks[i] = copy;
        }
    }
}

fn supporting_and_supported_by(
    bricks: &[Brick],
    below: &[Vec<usize>],
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supporting: Vec<_> = (0..bricks.len()).map(|_| vec![]).collect();
    let mut supported_by: Vec<_> = (0..bricks.len()).map(|_| vec![]).collect();

    for (i, &brick) in bricks.iter().enumerate() {
        for &j in &below[i] {
            let below = bricks[j];
            let top_of_below =
                if below.axis == Axis::Z { below.start.2 + below.len } else { below.start.2 };
            if top_of_below == brick.start.2 - 1 {
                supporting[j].push(i);
                supported_by[i].push(j);
            }
        }
    }

    (supporting, supported_by)
}

pub fn part1(input: &str) -> String {
    let mut bricks = parse_input(input);
    bricks.sort_unstable_by_key(|brick| brick.start.2);
    let below = below(&bricks);
    fall(&mut bricks, &below);
    let (_, supported_by) = supporting_and_supported_by(&bricks, &below);

    let mut lone_supporters = AHashSet::new();
    for supported_by in supported_by.iter() {
        if supported_by.len() == 1 {
            lone_supporters.insert(supported_by[0]);
        }
    }

    (bricks.len() - lone_supporters.len()).to_string()
}

fn count_falling_if_removed(
    supporting: &[Vec<usize>],
    supported_by: &[Vec<usize>],
    i: usize,
) -> usize {
    let mut removed = AHashSet::from([i]);
    let mut queue = VecDeque::from([i]);
    while let Some(brick_idx) = queue.pop_front() {
        for &above in &supporting[brick_idx] {
            if supported_by[above].iter().all(|&j| removed.contains(&j)) {
                removed.insert(above);
                queue.push_back(above);
            }
        }
    }
    removed.len() - 1
}

pub fn part2(input: &str) -> String {
    let mut bricks = parse_input(input);
    bricks.sort_unstable_by_key(|brick| brick.start.2);
    let below = below(&bricks);
    fall(&mut bricks, &below);
    let (supporting, supported_by) = supporting_and_supported_by(&bricks, &below);

    let mut total = 0usize;
    for i in 0..bricks.len() {
        total += count_falling_if_removed(&supporting, &supported_by, i);
    }
    total.to_string()
}
