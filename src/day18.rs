#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rotate_cw(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn rotate_ccw(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }
}

#[derive(Clone, Copy)]
struct Trench {
    dir: Dir,
    len: u8,
    col: [u8; 3],
}

#[derive(Clone, Copy)]
struct Instruction {
    dir: Dir,
    len: usize,
}

fn parse_trench(line: &str) -> Trench {
    let (dir, rest) = line.split_once(' ').unwrap();
    let (len, rest) = rest.split_once(' ').unwrap();
    let col = rest
        .strip_prefix("(#")
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap();

    let dir = match dir {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        "R" => Dir::Right,
        _ => unreachable!("Invalid direction"),
    };
    let len = len.parse().unwrap();
    let r = u8::from_str_radix(&col[0..2], 16).unwrap();
    let g = u8::from_str_radix(&col[2..4], 16).unwrap();
    let b = u8::from_str_radix(&col[4..6], 16).unwrap();
    Trench {
        dir,
        len,
        col: [r, g, b],
    }
}

fn parse_input(input: &str) -> Vec<Trench> {
    input.lines().map(|s| parse_trench(s.trim())).collect()
}

fn enclosed_area(trenches: &[Instruction]) -> usize {
    fn is_clockwise(trenches: &[Instruction]) -> bool {
        let mut windings = 0;
        for w in trenches.windows(2) {
            if w[0].dir.rotate_cw() == w[1].dir {
                windings += 1;
            } else if w[0].dir.rotate_ccw() == w[1].dir {
                windings -= 1;
            } else {
                unreachable!("Invalid trench configuration");
            }
        }
        windings > 0
    }

    let clockwise = is_clockwise(trenches);
    let mut area = 0isize;
    let mut pos = (0, 0);

    let get_corner = |t1: Instruction, t2: Instruction, pos: (isize, isize)| {
        use Dir::*;
        if clockwise {
            match (t1.dir, t2.dir) {
                (Up, Right) | (Right, Up) => pos,
                (Up, Left) | (Left, Up) => (pos.0, pos.1 + 1),
                (Right, Down) | (Down, Right) => (pos.0 + 1, pos.1),
                (Left, Down) | (Down, Left) => (pos.0 + 1, pos.1 + 1),
                _ => unreachable!(),
            }
        } else {
            match (t1.dir, t2.dir) {
                (Up, Right) | (Right, Up) => (pos.0 + 1, pos.1 + 1),
                (Up, Left) | (Left, Up) => (pos.0 + 1, pos.1),
                (Right, Down) | (Down, Right) => (pos.0, pos.1 + 1),
                (Left, Down) | (Down, Left) => pos,
                _ => unreachable!(),
            }
        }
    };

    for i in 0..trenches.len() {
        let prev = trenches[(i + trenches.len() - 1) % trenches.len()];
        let cur = trenches[i];
        let next = trenches[(i + 1) % trenches.len()];

        let (x_i, y_i) = get_corner(prev, cur, pos);
        match cur.dir {
            Dir::Up => pos.1 -= cur.len as isize,
            Dir::Down => pos.1 += cur.len as isize,
            Dir::Left => pos.0 -= cur.len as isize,
            Dir::Right => pos.0 += cur.len as isize,
        }
        let (x_j, y_j) = get_corner(cur, next, pos);

        area += x_i * y_j - x_j * y_i;
    }

    area.unsigned_abs() / 2
}

pub fn part1(input: &str) -> String {
    let trenches = parse_input(input);
    let insts = trenches
        .iter()
        .map(|t| Instruction {
            dir: t.dir,
            len: t.len as usize,
        })
        .collect::<Vec<_>>();
    enclosed_area(&insts).to_string()
}

pub fn part2(input: &str) -> String {
    let trenches = parse_input(input);
    let insts = trenches
        .iter()
        .map(|t| Instruction {
            dir: [Dir::Up, Dir::Left, Dir::Down, Dir::Right][(t.col[2] & 0x0F) as usize],
            len: (usize::from(t.col[0]) << 12)
                | (usize::from(t.col[1]) << 4)
                | (t.col[2] as usize >> 4),
        })
        .collect::<Vec<_>>();
    enclosed_area(&insts).to_string()
}
