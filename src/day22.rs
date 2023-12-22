use smallvec::SmallVec;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: (u16, u16, u16),
    end: (u16, u16, u16),
}

impl Brick {
    fn from_start_end(start: (u16, u16, u16), end: (u16, u16, u16)) -> Self {
        let (x1, y1, z1) = start;
        let (x2, y2, z2) = end;
        let start = (x1.min(x2), y1.min(y2), z1.min(z2));
        let end = (x1.max(x2), y1.max(y2), z1.max(z2));
        Self { start, end }
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<_> = input
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
        .collect();
    bricks.sort_unstable_by_key(|brick| brick.start.2);
    bricks
}

fn xy_limits(bricks: &[Brick]) -> ((u16, u16), (u16, u16)) {
    let (mut x_min, mut x_max) = (u16::MAX, 0);
    let (mut y_min, mut y_max) = (u16::MAX, 0);
    for brick in bricks {
        (x_min, x_max) = (x_min.min(brick.start.0), x_max.max(brick.end.0));
        (y_min, y_max) = (y_min.min(brick.start.1), y_max.max(brick.end.1));
    }
    ((x_min, x_max), (y_min, y_max))
}

struct State<'a> {
    bricks: &'a mut [Brick],
    x_lims: (u16, u16),
    y_lims: (u16, u16),
    touching_above: Vec<SmallVec<[u16; 4]>>,
    touching_below: Vec<SmallVec<[u16; 4]>>,
}

fn fall(state: &mut State) {
    let width = (state.x_lims.1 - state.x_lims.0 + 1) as usize;
    let height = (state.y_lims.1 - state.y_lims.0 + 1) as usize;

    let mut grid = vec![usize::MAX; width * height];
    let grid_idx = |x: u16, y: u16| {
        (y as usize - state.y_lims.0 as usize) * width + (x as usize - state.x_lims.0 as usize)
    };

    for brick_idx in 0..state.bricks.len() {
        let mut max_z = 0;
        // first, do a pass to find the maximum z of any brick below
        // the current one. the current brick will then be one above
        // that maximum z.
        let brick = state.bricks[brick_idx];
        for y in brick.start.1..=brick.end.1 {
            for x in brick.start.0..=brick.end.0 {
                let below_idx = grid[grid_idx(x, y)];
                if below_idx != usize::MAX {
                    let top_of_below = state.bricks[below_idx].end.2;
                    max_z = max_z.max(top_of_below);
                }
            }
        }
        let offset = brick.start.2 - max_z - 1;
        state.bricks[brick_idx].start.2 -= offset;
        state.bricks[brick_idx].end.2 -= offset;
        // in the second pass, compute all the bricks that now
        // touch the current one.
        let brick = state.bricks[brick_idx];
        for y in brick.start.1..=brick.end.1 {
            for x in brick.start.0..=brick.end.0 {
                let grid_idx = grid_idx(x, y);
                let below_idx = grid[grid_idx];
                if below_idx != usize::MAX {
                    let top_of_below = state.bricks[below_idx].end.2;
                    if top_of_below == max_z
                        && !state.touching_below[brick_idx].contains(&(below_idx as u16))
                    {
                        state.touching_below[brick_idx].push(below_idx as u16);
                        state.touching_above[below_idx].push(brick_idx as u16);
                    }
                }
                grid[grid_idx] = brick_idx;
            }
        }
    }
}

// counts the number of bricks that, if removed, would lead
// to other bricks falling down.
fn count_loadbearing(state: &State) -> usize {
    let n = state.bricks.len();
    let mut loadbearing = vec![false; n];
    for below in &state.touching_below {
        if below.len() == 1 {
            loadbearing[below[0] as usize] = true;
        }
    }
    loadbearing.iter().filter(|&&bit| bit).count()
}

fn sum_of_falling(state: &State) -> usize {
    let n = state.bricks.len();
    let mut falling = vec![false; n];
    let mut sum = 0;
    for piece_idx in 0..n {
        falling.fill(false);
        falling[piece_idx] = true;
        'outer: for falling_idx in piece_idx + 1..n {
            // in this case the piece is already on the bottom layer.
            if state.touching_below[falling_idx].is_empty() {
                continue;
            }
            // if there's any piece below that isn't falling, then
            // the current piece isn't falling either.
            for &below_idx in &state.touching_below[falling_idx] {
                if !falling[below_idx as usize] {
                    continue 'outer;
                }
            }
            falling[falling_idx] = true;
            sum += 1;
        }
    }

    sum
}

pub fn part1(input: &str) -> String {
    let bricks = parse_input(input);
    let (x_lims, y_lims) = xy_limits(&bricks);
    let mut state = State {
        bricks: &mut bricks.clone(),
        x_lims,
        y_lims,
        touching_above: vec![SmallVec::new(); bricks.len()],
        touching_below: vec![SmallVec::new(); bricks.len()],
    };
    fall(&mut state);

    let non_loadbearing = state.bricks.len() - count_loadbearing(&state);
    non_loadbearing.to_string()
}

pub fn part2(input: &str) -> String {
    let bricks = parse_input(input);
    let (x_lims, y_lims) = xy_limits(&bricks);
    let mut state = State {
        bricks: &mut bricks.clone(),
        x_lims,
        y_lims,
        touching_above: vec![SmallVec::new(); bricks.len()],
        touching_below: vec![SmallVec::new(); bricks.len()],
    };
    fall(&mut state);

    sum_of_falling(&state).to_string()
}
