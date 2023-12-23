use std::{collections::hash_map::Entry, fmt};

use ahash::{AHashMap, AHashSet};
use enum_map::{Enum, EnumMap};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Slope(Dir),
}

struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                let c = match cell {
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Slope(Dir::West) => '<',
                    Cell::Slope(Dir::East) => '>',
                    Cell::Slope(Dir::North) => '^',
                    Cell::Slope(Dir::South) => 'v',
                };
                fmt::Write::write_char(f, c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        width = line.len();
        height += 1;
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

type Coords = (usize, usize);
type Vertex = (Coords, EnumMap<Dir, Option<(usize, usize)>>);

#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
    start: usize,
    end: usize,
}

fn grid_to_graph(grid: &Grid) -> Graph {
    fn intersection_index(
        coords: Coords,
        indices: &mut AHashMap<Coords, usize>,
        vertices: &mut Vec<Vertex>,
    ) -> usize {
        match indices.entry(coords) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let idx = vertices.len();
                vertices.push((coords, EnumMap::default()));
                *v.insert(idx)
            }
        }
    }

    fn walk(grid: &Grid, (mut x, mut y): Coords, mut dir: Dir) -> (Coords, usize, Dir) {
        let mut steps = 0;
        loop {
            if (x == 0 && dir == Dir::West)
                || (x + 1 == grid.width && dir == Dir::East)
                || (y == 0 && dir == Dir::North)
                || (y + 1 == grid.height && dir == Dir::South)
            {
                return ((x, y), steps, dir);
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
            if dir != Dir::East
                && x > 0
                && matches!(grid.get(x - 1, y), Cell::Empty | Cell::Slope(Dir::West))
            {
                neighbor_dirs.push(Dir::West);
            }

            if dir != Dir::West
                && x + 1 < grid.width
                && matches!(grid.get(x + 1, y), Cell::Empty | Cell::Slope(Dir::East))
            {
                neighbor_dirs.push(Dir::East);
            }

            if dir != Dir::South
                && y > 0
                && matches!(grid.get(x, y - 1), Cell::Empty | Cell::Slope(Dir::North))
            {
                neighbor_dirs.push(Dir::North);
            }

            if dir != Dir::North
                && y + 1 < grid.height
                && matches!(grid.get(x, y + 1), Cell::Empty | Cell::Slope(Dir::South))
            {
                neighbor_dirs.push(Dir::South);
            }

            // dbg!((x, y), dir, &neighbor_dirs);

            match neighbor_dirs[..] {
                // exactly one neighbor => go there
                [next_dir] => {
                    dir = next_dir;
                }
                // no neighbors or more than one neighbor => node
                _ => {
                    return ((x, y), steps, dir);
                }
            }
        }
    }

    let mut indices = AHashMap::new();
    let mut vertices = Vec::new();
    let start_x =
        (0..grid.width).find(|&x| grid.get(x, 0) == Cell::Empty).expect("No start node found");
    let start_idx = intersection_index((start_x, 0), &mut indices, &mut vertices);
    let mut visited = AHashSet::new();
    let mut stack = vec![(start_idx, None)];

    while let Some((vertex_idx, last_dir)) = stack.pop() {
        if !visited.insert(vertex_idx) {
            continue;
        }
        let ((x, y), _) = vertices[vertex_idx];

        if last_dir != Some(Dir::West)
            && x + 1 < grid.width
            && matches!(grid.get(x + 1, y), Cell::Empty | Cell::Slope(Dir::East))
        {
            // walk east
            let (coords, dist, dir) = walk(grid, (x, y), Dir::East);
            let neighbor_idx = intersection_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::East] = Some((neighbor_idx, dist));
            stack.push((neighbor_idx, Some(dir)));
        }

        if last_dir != Some(Dir::East)
            && x > 0
            && matches!(grid.get(x - 1, y), Cell::Empty | Cell::Slope(Dir::West))
        {
            // walk west
            let (coords, dist, dir) = walk(grid, (x, y), Dir::West);
            let neighbor_idx = intersection_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::West] = Some((neighbor_idx, dist));
            stack.push((neighbor_idx, Some(dir)));
        }

        if last_dir != Some(Dir::South)
            && y > 0
            && matches!(grid.get(x, y - 1), Cell::Empty | Cell::Slope(Dir::North))
        {
            // walk north
            let (coords, dist, dir) = walk(grid, (x, y), Dir::North);
            let neighbor_idx = intersection_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::North] = Some((neighbor_idx, dist));
            stack.push((neighbor_idx, Some(dir)));
        }

        if last_dir != Some(Dir::North)
            && y + 1 < grid.height
            && matches!(grid.get(x, y + 1), Cell::Empty | Cell::Slope(Dir::South))
        {
            // walk south
            let (coords, dist, dir) = walk(grid, (x, y), Dir::South);
            let neighbor_idx = intersection_index(coords, &mut indices, &mut vertices);
            vertices[vertex_idx].1[Dir::South] = Some((neighbor_idx, dist));
            stack.push((neighbor_idx, Some(dir)));
        }
    }

    let end_x = (0..grid.width).find(|&x| grid.get(x, grid.height - 1) == Cell::Empty).unwrap();
    let end_idx = intersection_index((end_x, grid.height - 1), &mut indices, &mut vertices);

    Graph { vertices, start: start_idx, end: end_idx }
}

// Assumes that the graph is a DAG.
fn longest_path(graph: &Graph, start: usize, end: usize) -> usize {
    if start == end {
        return 0;
    }
    let neighbors = graph.vertices[start].1;
    neighbors
        .values()
        .flatten()
        .map(|(idx, dist)| dist + longest_path(graph, *idx, end))
        .max()
        .unwrap()
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);
    let graph = grid_to_graph(&grid);

    longest_path(&graph, graph.start, graph.end).to_string()
}

pub fn part2(_input: &str) -> String {
    unimplemented!()
}
