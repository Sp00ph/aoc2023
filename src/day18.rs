#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
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

fn parse_input(input: &str) -> impl Iterator<Item = Trench> + '_ {
    input.lines().map(|s| parse_trench(s.trim()))
}

// Very similar area calculation to part 10, except that this time it has to include
// the boundary, whereas in day 10 it didn't. It uses the shoelace formula in
// combination with Pick's theorem.
fn enclosed_area(trenches: &[Instruction]) -> usize {
    let mut area = 0isize;
    let mut perimeter = 0;
    let mut pos = (0, 0);

    // Simple shoelace formula implementation.
    for trench in trenches {
        perimeter += trench.len;

        let (x_i, y_i) = pos;
        match trench.dir {
            Dir::Up => pos.1 -= trench.len as isize,
            Dir::Down => pos.1 += trench.len as isize,
            Dir::Left => pos.0 -= trench.len as isize,
            Dir::Right => pos.0 += trench.len as isize,
        }
        let (x_j, y_j) = pos;

        area += x_i * y_j - x_j * y_i;
    }

    // Since we want the enclosing area of the polygon including the boundary,
    // we need to adjust the result using the perimeter. Pick's theorem states
    // that i + b = A + b/2 + 1, where i is the number of interior points, b is
    // the number of boundary points, and A is the area of the polygon. We calculated
    // A and b, and quantity we're interested in is i + b.
    (area.unsigned_abs() + perimeter) / 2 + 1
}

pub fn part1(input: &str) -> String {
    let trenches = parse_input(input);
    let insts = trenches
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
        .map(|t| Instruction {
            dir: [Dir::Up, Dir::Left, Dir::Down, Dir::Right][(t.col[2] & 0x0F) as usize],
            len: (usize::from(t.col[0]) << 12)
                | (usize::from(t.col[1]) << 4)
                | (t.col[2] as usize >> 4),
        })
        .collect::<Vec<_>>();
    enclosed_area(&insts).to_string()
}
