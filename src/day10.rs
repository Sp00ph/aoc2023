#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Start,
    Ground,
    Horizontal,
    Vertical,
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Pipe {
    fn connects(&self, dir: Dir) -> bool {
        match dir {
            Dir::Up => matches!(
                self,
                Pipe::Start | Pipe::TopLeft | Pipe::TopRight | Pipe::Vertical
            ),
            Dir::Right => matches!(
                self,
                Pipe::Start | Pipe::TopRight | Pipe::BottomRight | Pipe::Horizontal
            ),
            Dir::Down => matches!(
                self,
                Pipe::Start | Pipe::BottomLeft | Pipe::BottomRight | Pipe::Vertical
            ),
            Dir::Left => matches!(
                self,
                Pipe::Start | Pipe::TopLeft | Pipe::BottomLeft | Pipe::Horizontal
            ),
        }
    }
}

struct Grid {
    data: Vec<Pipe>,
    start_pos: (u8, u8),
    width: u8,
    height: u8,
}

impl Grid {
    fn get(&self, x: u8, y: u8) -> Option<Pipe> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.data[y as usize * self.width as usize + x as usize])
        }
    }
}
fn parse_input(input: &str) -> Grid {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut start_pos = None;

    for line in input.trim().lines() {
        width = line.len() as u8;
        for (i, c) in line.bytes().enumerate() {
            data.push(match c {
                b'S' => {
                    start_pos = Some((i as u8, height));
                    Pipe::Start
                }
                b'.' => Pipe::Ground,
                b'-' => Pipe::Horizontal,
                b'|' => Pipe::Vertical,
                b'L' => Pipe::TopRight,
                b'F' => Pipe::BottomRight,
                b'7' => Pipe::BottomLeft,
                b'J' => Pipe::TopLeft,
                _ => panic!("invalid character"),
            });
        }
        height += 1;
    }

    Grid {
        data,
        width,
        height,
        start_pos: start_pos.expect("no start position found"),
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
}

#[derive(Clone, Copy)]
struct Segment {
    start: (u8, u8),
    dir: Dir,
    len: u8,
}

impl Segment {
    fn horizontal(&self) -> bool {
        matches!(self.dir, Dir::Left | Dir::Right)
    }
}

fn loop_segments(grid: &Grid) -> Vec<Segment> {
    let mut cur = grid.start_pos;
    let mut segments = vec![];

    fn walk(grid: &Grid, pos: (u8, u8), dir: Dir) -> u8 {
        let pipe_type = if dir == Dir::Left || dir == Dir::Right {
            Pipe::Horizontal
        } else {
            Pipe::Vertical
        };
        let (dx, dy) = match dir {
            Dir::Up => (0, u8::MAX),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (u8::MAX, 0),
        };

        let mut cur = pos;
        let mut len = 0;

        loop {
            let next = (cur.0.wrapping_add(dx), cur.1.wrapping_add(dy));
            let next_pipe = grid.get(next.0, next.1);
            match next_pipe {
                Some(pipe) if pipe == pipe_type => {}
                Some(pipe) if pipe.connects(dir.opposite()) => return len + 1,
                _ => return len,
            }
            cur = next;
            len += 1;
        }
    }

    loop {
        let pipe = grid.get(cur.0, cur.1).unwrap();
        if pipe == Pipe::Start && !segments.is_empty() {
            return segments;
        }
        let prev_seg_was_horizontal = segments.last().map_or(false, |seg| seg.horizontal());
        let prev_seg_was_vertical = segments.last().map_or(false, |seg| !seg.horizontal());

        let mut check_and_walk = |dir: Dir| -> bool {
            let cond = match dir {
                Dir::Up | Dir::Down => prev_seg_was_vertical,
                Dir::Right | Dir::Left => prev_seg_was_horizontal,
            };
            if pipe.connects(dir) && !cond {
                let len = walk(grid, cur, dir);
                if len != 0 {
                    segments.push(Segment {
                        start: cur,
                        dir,
                        len,
                    });
                    match dir {
                        Dir::Up => cur.1 -= len,
                        Dir::Right => cur.0 += len,
                        Dir::Down => cur.1 += len,
                        Dir::Left => cur.0 -= len,
                    }
                    return true;
                }
            }
            false
        };

        if check_and_walk(Dir::Up) {
            continue;
        }
        if check_and_walk(Dir::Right) {
            continue;
        }
        if check_and_walk(Dir::Down) {
            continue;
        }
        if check_and_walk(Dir::Left) {
            continue;
        }

        unreachable!()
    }
}

fn loop_len(grid: &Grid) -> usize {
    let segments = loop_segments(grid);
    segments.iter().map(|seg| seg.len as usize).sum()
}

pub fn part1(input: &str) -> String {
    let grid = parse_input(input);
    let loop_len = loop_len(&grid);
    (loop_len / 2).to_string()
}

// calculate the area using the shoelace formula and Pick's theorem
fn area(segs: &[Segment]) -> usize {
    let mut area = 0isize;
    let mut perimeter = 0;

    for i in 0..segs.len() {
        let seg = segs[i];
        let next_seg = segs[(i + 1) % segs.len()];
        
        let (x_i, y_i) = seg.start;
        let (x_j, y_j) = next_seg.start;
        
        perimeter += seg.len as usize;

        area += x_i as isize * y_j as isize - x_j as isize * y_i as isize;
    }

    // Pick's theorem: i + b = A + b/2 + 1
    // => i = A - b/2 + 1
    (area.unsigned_abs() - perimeter) / 2 + 1
}

pub fn part2(input: &str) -> String {
    let grid = parse_input(input);
    let segments = loop_segments(&grid);

    area(&segments).to_string()
}
