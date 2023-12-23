use std::collections::hash_map::Entry;

use ahash::{AHashMap, AHashSet};
use enum_map::{Enum, EnumMap};
use smallvec::SmallVec;

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Slope(Dir),
}

struct Grid {
    cells: Vec<Cell>,
    width: u8,
    height: u8,
}

impl Grid {
    fn get(&self, x: u8, y: u8) -> Cell {
        assert!(x < self.width && y < self.height);
        let (x, y) = (x as usize, y as usize);
        self.cells[y * self.width as usize + x]
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = Vec::new();
    let mut width = 0;
    let mut height = 0u8;
    for line in input.lines() {
        width = u8::try_from(line.len()).expect("grid too wide");
        height = height.checked_add(1).expect("grid too tall");
        for c in line.chars() {
            cells.push(match c {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                '<' => Cell::Slope(Dir::West),
                '>' => Cell::Slope(Dir::East),
                '^' => Cell::Slope(Dir::North),
                'v' => Cell::Slope(Dir::South),
                _ => panic!("invalid cell: {c}"),
            });
        }
    }
    Grid { cells, width, height }
}

type Coords = (u8, u8);
type Vertex = (Coords, EnumMap<Dir, Option<(u8, u16)>>);

struct Graph {
    vertices: Vec<Vertex>,
    start: u8,
    end: u8,
}

fn grid_to_graph(grid: &Grid, climb_slopes: bool) -> Graph {
    fn vertex_index(
        coords: Coords,
        indices: &mut AHashMap<Coords, u8>,
        vertices: &mut Vec<Vertex>,
    ) -> u8 {
        match indices.entry(coords) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let idx = vertices.len();
                vertices.push((coords, EnumMap::default()));
                *v.insert(u8::try_from(idx).expect("too many vertices"))
            }
        }
    }

    fn can_step_north(grid: &Grid, (x, y): Coords, climb_slopes: bool) -> bool {
        if climb_slopes {
            y > 0 && grid.get(x, y - 1) != Cell::Wall
        } else {
            y > 0 && matches!(grid.get(x, y - 1), Cell::Empty | Cell::Slope(Dir::North))
        }
    }

    fn can_step_south(grid: &Grid, (x, y): Coords, climb_slopes: bool) -> bool {
        if climb_slopes {
            y + 1 < grid.height && grid.get(x, y + 1) != Cell::Wall
        } else {
            y + 1 < grid.height
                && matches!(grid.get(x, y + 1), Cell::Empty | Cell::Slope(Dir::South))
        }
    }

    fn can_step_east(grid: &Grid, (x, y): Coords, climb_slopes: bool) -> bool {
        if climb_slopes {
            x + 1 < grid.width && grid.get(x + 1, y) != Cell::Wall
        } else {
            x + 1 < grid.width && matches!(grid.get(x + 1, y), Cell::Empty | Cell::Slope(Dir::East))
        }
    }

    fn can_step_west(grid: &Grid, (x, y): Coords, climb_slopes: bool) -> bool {
        if climb_slopes {
            x > 0 && grid.get(x - 1, y) != Cell::Wall
        } else {
            x > 0 && matches!(grid.get(x - 1, y), Cell::Empty | Cell::Slope(Dir::West))
        }
    }

    fn walk(
        grid: &Grid,
        (mut x, mut y): Coords,
        mut dir: Dir,
        climb_slopes: bool,
    ) -> (Coords, u16) {
        let mut steps = 0;
        loop {
            if (x == 0 && dir == Dir::West)
                || (x + 1 == grid.width && dir == Dir::East)
                || (y == 0 && dir == Dir::North)
                || (y + 1 == grid.height && dir == Dir::South)
            {
                return ((x, y), steps);
            }
            (x, y) = match dir {
                Dir::North => (x, y - 1),
                Dir::South => (x, y + 1),
                Dir::East => (x + 1, y),
                Dir::West => (x - 1, y),
            };
            steps += 1;
            // All the directions that we can walk to, except for the one we came from.
            let mut neighbor_dirs = SmallVec::<[Dir; 4]>::new();
            if dir != Dir::East && can_step_west(grid, (x, y), climb_slopes) {
                neighbor_dirs.push(Dir::West);
            }

            if dir != Dir::West && can_step_east(grid, (x, y), climb_slopes) {
                neighbor_dirs.push(Dir::East);
            }

            if dir != Dir::South && can_step_north(grid, (x, y), climb_slopes) {
                neighbor_dirs.push(Dir::North);
            }

            if dir != Dir::North && can_step_south(grid, (x, y), climb_slopes) {
                neighbor_dirs.push(Dir::South);
            }

            match neighbor_dirs[..] {
                // exactly one neighbor => go there
                [next_dir] => {
                    dir = next_dir;
                }
                // no neighbors or more than one neighbor => node
                _ => {
                    return ((x, y), steps);
                }
            }
        }
    }

    let mut indices = AHashMap::new();
    let mut vertices = Vec::new();
    let start_x =
        (0..grid.width).find(|&x| grid.get(x, 0) == Cell::Empty).expect("No start node found");
    let start_idx = vertex_index((start_x, 0), &mut indices, &mut vertices);
    let mut visited = AHashSet::new();
    let mut stack = vec![(start_idx)];

    while let Some(vertex_idx) = stack.pop() {
        let vertex_idx = vertex_idx as usize;
        if !visited.insert(vertex_idx) {
            continue;
        }
        let ((x, y), _) = vertices[vertex_idx];

        if can_step_east(grid, (x, y), climb_slopes) {
            // walk east
            let (coords, dist) = walk(grid, (x, y), Dir::East, climb_slopes);
            let neighbor_idx = vertex_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::East] = Some((neighbor_idx, dist));
            stack.push(neighbor_idx);
        }

        if can_step_west(grid, (x, y), climb_slopes) {
            // walk west
            let (coords, dist) = walk(grid, (x, y), Dir::West, climb_slopes);
            let neighbor_idx = vertex_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::West] = Some((neighbor_idx, dist));
            stack.push(neighbor_idx);
        }

        if can_step_north(grid, (x, y), climb_slopes) {
            // walk north
            let (coords, dist) = walk(grid, (x, y), Dir::North, climb_slopes);
            let neighbor_idx = vertex_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::North] = Some((neighbor_idx, dist));
            stack.push(neighbor_idx);
        }

        if can_step_south(grid, (x, y), climb_slopes) {
            // walk south
            let (coords, dist) = walk(grid, (x, y), Dir::South, climb_slopes);
            let neighbor_idx = vertex_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::South] = Some((neighbor_idx, dist));
            stack.push(neighbor_idx);
        }
    }

    let end_x = (0..grid.width).find(|&x| grid.get(x, grid.height - 1) == Cell::Empty).unwrap();
    let end_idx = vertex_index((end_x, grid.height - 1), &mut indices, &mut vertices);

    Graph { vertices, start: start_idx, end: end_idx }
}


fn longest_path(graph: &Graph, start: u8, end: u8) -> usize {
    let mut visited = vec![false; graph.vertices.len()];

    fn dfs(graph: &Graph, visited: &mut [bool], start: u8, end: u8, dist: usize) -> usize {
        if start == end {
            return dist;
        }
        visited[start as usize] = true;
        let mut max_dist = 0;
        for (_, neighbor) in &graph.vertices[start as usize].1 {
            if let Some((idx, neighbor_dist)) = neighbor {
                if !visited[*idx as usize] {
                    max_dist = max_dist.max(dfs(graph, visited, *idx, end, dist + *neighbor_dist as usize));
                }
            }
        }
        visited[start as usize] = false;
        max_dist
    }

    dfs(graph, &mut visited, start, end, 0)
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);
    let graph = grid_to_graph(&grid, false);

    longest_path(&graph, graph.start, graph.end).to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);
    let graph = grid_to_graph(&grid, true);

    longest_path(&graph, graph.start, graph.end).to_string()
}
