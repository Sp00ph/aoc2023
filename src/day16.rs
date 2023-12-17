#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    Mirror45Degree,
    Mirror135Degree,
}

struct Grid {
    cells: Vec<Cell>,
    width: u8,
    height: u8,
}

impl Grid {
    fn get(&self, x: u8, y: u8) -> Cell {
        let idx = (y as usize) * (self.width as usize) + (x as usize);
        self.cells[idx]
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        width = line.len() as u8;
        height += 1;
        for c in line.chars() {
            cells.push(match c {
                '.' => Cell::Empty,
                '-' => Cell::HorizontalSplitter,
                '|' => Cell::VerticalSplitter,
                '/' => Cell::Mirror45Degree,
                '\\' => Cell::Mirror135Degree,
                _ => unreachable!("invalid input"),
            });
        }
    }
    Grid {
        cells,
        width,
        height,
    }
}

const RIGHT: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const UP: u8 = 0b1000;

fn count_energized_tiles(grid: &Grid, (start_x, start_y, from_dir): (u8, u8, u8)) -> usize {
    use Cell::*;

    // Use the lower 4 bits of each element for one direction each.
    // TODO: Pack 2 cells into each byte?
    let mut visited = vec![0u8; grid.cells.len()];
    let was_visited = |visited: &[u8], x: u8, y: u8, mask: u8| {
        let idx = (y as usize) * (grid.width as usize) + (x as usize);
        visited[idx] & mask != 0
    };
    let mark_visited = |visited: &mut [u8], x: u8, y: u8, mask: u8| {
        let idx = (y as usize) * (grid.width as usize) + (x as usize);
        visited[idx] |= mask;
    };

    let mut stack = vec![(start_x, start_y, from_dir)];

    while let Some((x, y, from_dir)) = stack.pop() {
        if was_visited(&visited, x, y, from_dir) {
            continue;
        }
        mark_visited(&mut visited, x, y, from_dir);
        let cell = grid.get(x, y);
        // all the cases to move right:
        if x + 1 < grid.width
            && ((cell == Empty && from_dir == LEFT)
                || (cell == Mirror45Degree && from_dir == DOWN)
                || (cell == Mirror135Degree && from_dir == UP)
                || (cell == HorizontalSplitter && from_dir != RIGHT))
        {
            // Make a copy of x and mutate only the copy. In case we want to move both left and right,
            // not making a copy of x would result in more moves than necessary.
            let mut x = x;
            // greedily move right until we hit either the wall, a vertical splitter or a mirror.
            while x + 1 < grid.width && matches!(grid.get(x + 1, y), Empty | HorizontalSplitter) {
                mark_visited(&mut visited, x + 1, y, LEFT);
                x += 1;
            }
            if x + 1 < grid.width {
                stack.push((x + 1, y, LEFT));
            }
        }

        // all the cases to move down:
        if y + 1 < grid.height
            && ((cell == Empty && from_dir == UP)
                || (cell == Mirror45Degree && from_dir == RIGHT)
                || (cell == Mirror135Degree && from_dir == LEFT)
                || (cell == VerticalSplitter && from_dir != DOWN))
        {
            let mut y = y;
            // greedily move down until we hit either the wall, a horizontal splitter or a mirror.
            while y + 1 < grid.height && matches!(grid.get(x, y + 1), Empty | VerticalSplitter) {
                mark_visited(&mut visited, x, y + 1, UP);
                y += 1;
            }
            if y + 1 < grid.height {
                stack.push((x, y + 1, UP));
            }
        }

        // all the cases to move left:
        if x > 0
            && ((cell == Empty && from_dir == RIGHT)
                || (cell == Mirror45Degree && from_dir == UP)
                || (cell == Mirror135Degree && from_dir == DOWN)
                || (cell == HorizontalSplitter && from_dir != LEFT))
        {
            let mut x = x;
            // greedily move left until we hit either the wall, a vertical splitter or a mirror.
            while x > 0 && matches!(grid.get(x - 1, y), Empty | HorizontalSplitter) {
                mark_visited(&mut visited, x - 1, y, RIGHT);
                x -= 1;
            }
            if x > 0 {
                stack.push((x - 1, y, RIGHT));
            }
        }

        // all the cases to move up:
        if y > 0
            && ((cell == Empty && from_dir == DOWN)
                || (cell == Mirror45Degree && from_dir == LEFT)
                || (cell == Mirror135Degree && from_dir == RIGHT)
                || (cell == VerticalSplitter && from_dir != UP))
        {
            let mut y = y;
            // greedily move up until we hit either the wall, a horizontal splitter or a mirror.
            while y > 0 && matches!(grid.get(x, y - 1), Empty | VerticalSplitter) {
                mark_visited(&mut visited, x, y - 1, DOWN);
                y -= 1;
            }
            if y > 0 {
                stack.push((x, y - 1, DOWN));
            }
        }
    }

    visited.iter().filter(|&&v| v != 0).count()
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);
    count_energized_tiles(&grid, (0, 0, LEFT)).to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);
    let mut max_energized = 0;
    for x in 0..grid.width {
        max_energized = max_energized.max(count_energized_tiles(&grid, (x, 0, UP)));
        max_energized = max_energized.max(count_energized_tiles(&grid, (x, grid.height - 1, DOWN)));
    }
    for y in 0..grid.height {
        max_energized = max_energized.max(count_energized_tiles(&grid, (0, y, RIGHT)));
        max_energized = max_energized.max(count_energized_tiles(&grid, (grid.width - 1, y, LEFT)));
    }
    max_energized.to_string()
}
