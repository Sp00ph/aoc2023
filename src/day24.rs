struct Hailstone {
    px: isize,
    py: isize,
    pz: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl std::fmt::Debug for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} @ {}, {}, {}", self.px, self.py, self.pz, self.vx, self.vy, self.vz)
    }
}

fn parse_hailstone(line: &str) -> Hailstone {
    let rest = line.trim();
    let (px, rest) = rest.split_once(", ").unwrap();
    let (py, rest) = rest.split_once(", ").unwrap();
    let (pz, rest) = rest.split_once(" @ ").unwrap();
    let (vx, rest) = rest.split_once(", ").unwrap();
    let (vy, vz) = rest.split_once(", ").unwrap();
    Hailstone {
        px: px.trim().parse().unwrap(),
        py: py.trim().parse().unwrap(),
        pz: pz.trim().parse().unwrap(),
        vx: vx.trim().parse().unwrap(),
        vy: vy.trim().parse().unwrap(),
        vz: vz.trim().parse().unwrap(),
    }
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input.lines().map(parse_hailstone).collect()
}

fn xy_intersect_in_xy_range(a: &Hailstone, b: &Hailstone, min: usize, max: usize) -> bool {
    let (px, py, vx, vy) = (a.px as i128, a.py as i128, a.vx as i128, a.vy as i128);
    let (qx, qy, wx, wy) = (b.px as i128, b.py as i128, b.vx as i128, b.vy as i128);
    let (dx, dy) = (px - qx, py - qy);

    // we want to solve the system of linear equations:
    // px + t * vx = qx + s * wx
    // py + t * vy = qy + s * wy
    //
    // Using these definitions:
    // dx := px - qx, dy := py - qy
    // A := (wx, -vx,
    //       wy, -vy),
    // b := (dx, dy)
    //
    // we want to then solve A * (s, t) = b
    // which is equivalent to (s, t) = A^-1 * b = 1/det(A) (vx*dy - vy*dx, wx*dy - wy*dx)

    let det = vx * wy - vy * wx;
    if det == 0 {
        // the lines are either parallel or coincident.
        // the lines are coincident if (dx, dy) is a multiple of (vx, vy)
        // so dx/vx = dy/vy
        // => dx * vy = dy * vx iff the lines are coincident
        return dx * vy == dy * vx;
    }

    let scaled_s = vx * dy - vy * dx;
    let scaled_t = wx * dy - wy * dx;

    // If at least one of the scaled parameters has a different sign than det
    // then the intersection lies in that line's past.
    if ((scaled_s < 0) ^ (det < 0)) || ((scaled_t < 0) ^ (det < 0)) {
        return false;
    }

    // now check if min <= px + t * vx <= max
    // => min - px <= t * vx <= max - px
    // => scaled_t * vx lies between det(min - px) and det(max - px)
    // and same for y

    let mut min_x = (min as i128 - px) * det;
    let mut max_x = (max as i128 - px) * det;
    let mut min_y = (min as i128 - py) * det;
    let mut max_y = (max as i128 - py) * det;
    if det < 0 {
        (min_x, max_x) = (max_x, min_x);
        (min_y, max_y) = (max_y, min_y);
    }
    (min_x..=max_x).contains(&(scaled_t * vx)) && (min_y..=max_y).contains(&(scaled_t * vy))
}

pub fn part1(input: &str) -> String {
    let stones = parse_input(input);
    let mut count = 0usize;
    for (i, a) in stones.iter().enumerate() {
        for b in &stones[i + 1..] {
            count += usize::from(xy_intersect_in_xy_range(a, b, 200000000000000, 400000000000000));
        }
    }
    count.to_string()
}

pub fn part2(_input: &str) -> String {
    unimplemented!()
}
