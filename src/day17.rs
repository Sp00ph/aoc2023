use std::{cmp::Reverse, collections::BinaryHeap};

struct Grid {
    data: Vec<u8>,
    width: u8,
    height: u8,
}

impl Grid {
    fn get(&self, x: u8, y: u8) -> u8 {
        self.data[y as usize * self.width as usize + x as usize]
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut data = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        width = line.len() as u8;
        height += 1;
        data.extend(line.bytes().map(|b| b - b'0'));
    }
    Grid {
        data,
        width,
        height,
    }
}

fn min_heat_loss(grid: &Grid, min_steps: u8, max_steps: u8) -> usize {
    // conceptually we want to do a dijkstra search on the following graph:
    // the vertex set is [0..width) x [0..height) x { North, South, East, West, Start }
    // each vertex describes one grid cell as well as the direction from its predecessor
    // (where the start vertex gets the special Start predecessor).
    // the edge set is the set of all possible moves from one vertex to another.
    //
    // we never fully compute this graph, we just compute the edges on the fly.
    type Node = (u8, u8, u8);

    // we need to use Reverse<usize> as the priority type, because the priority queue is a max-heap.
    type Queue = BinaryHeap<(Reverse<usize>, Node)>;

    // Use a dense array instead of a HashMap. Indexing into the array is faster than hashing,
    // and the map would contain every possible key anyways, so there's not much space wastage
    // by storing every distance.
    type DistMap = Vec<usize>;

    const NORTH: u8 = 0;
    const SOUTH: u8 = 1;
    const EAST: u8 = 2;
    const WEST: u8 = 3;
    const START: u8 = 4;

    // the start node gets the special Start predecessor, so it can go either down or right.
    let mut queue = Queue::from_iter([(Reverse(0), (0, 0, START))]);
    let mut dists = vec![usize::MAX; grid.width as usize * grid.height as usize * 4];

    fn update_dists_and_queue(
        grid: &Grid,
        queue: &mut Queue,
        dists: &mut DistMap,
        node @ (x, y, dir): Node,
        dist: usize,
    ) {
        let idx = (x as usize * grid.width as usize + y as usize) * 4 + dir as usize;
        if dist < dists[idx] {
            dists[idx] = dist;
            queue.push((Reverse(dist), node));
        }
    }

    while let Some((Reverse(dist), (x, y, dir))) = queue.pop() {
        let dist_idx = (x as usize * grid.width as usize + y as usize) * 4 + dir as usize;
        if dist > dists[dist_idx] {
            continue;
        }

        // these can both be false, if the predecessor was the start node
        let was_horizontal = dir == EAST || dir == WEST;
        let was_vertical = dir == NORTH || dir == SOUTH;

        // The max number of steps that we can walk north, before we either need to turn
        // because of the instability, or we hit the top of the grid.
        let max_north = max_steps.min(y);
        if max_north >= min_steps && !was_vertical {
            // precompute the distances to the closest possible neighbors, so we don't have to do it
            // on each iteration of the loop. Unfortunately, this only saves a few milliseconds.
            let mut north_dist = (1..min_steps)
                .map(|i| grid.get(x, y - i) as usize)
                .sum::<usize>();
            for i in min_steps..=max_north {
                north_dist += grid.get(x, y - i) as usize;
                let neighbor = (x, y - i, NORTH);
                let neighbor_dist = dist + north_dist;
                update_dists_and_queue(grid, &mut queue, &mut dists, neighbor, neighbor_dist);
            }
        }

        let max_south = max_steps.min(grid.height - y - 1);
        if max_south >= min_steps && !was_vertical {
            let mut south_dist = (1..min_steps)
                .map(|i| grid.get(x, y + i) as usize)
                .sum::<usize>();
            for i in min_steps..=max_south {
                south_dist += grid.get(x, y + i) as usize;
                let neighbor = (x, y + i, SOUTH);
                let neighbor_dist = dist + south_dist;
                update_dists_and_queue(grid, &mut queue, &mut dists, neighbor, neighbor_dist);
            }
        }

        let max_east = max_steps.min(grid.width - x - 1);
        if max_east >= min_steps && !was_horizontal {
            let mut east_dist = (1..min_steps)
                .map(|i| grid.get(x + i, y) as usize)
                .sum::<usize>();
            for i in min_steps..=max_east {
                east_dist += grid.get(x + i, y) as usize;
                let neighbor = (x + i, y, EAST);
                let neighbor_dist = dist + east_dist;
                update_dists_and_queue(grid, &mut queue, &mut dists, neighbor, neighbor_dist);
            }
        }

        let max_west = max_steps.min(x);
        if max_west >= min_steps && !was_horizontal {
            let mut west_dist = (1..min_steps)
                .map(|i| grid.get(x - i, y) as usize)
                .sum::<usize>();
            for i in min_steps..=max_west {
                west_dist += grid.get(x - i, y) as usize;
                let neighbor = (x - i, y, WEST);
                let neighbor_dist = dist + west_dist;
                update_dists_and_queue(grid, &mut queue, &mut dists, neighbor, neighbor_dist);
            }
        }
    }

    let end = (grid.width - 1, grid.height - 1);

    // filter through all the vertices that represent the end cell,
    // and find the one with the minimum distance.
    let end_idx = (end.1 as usize * grid.width as usize + end.0 as usize) * 4;
    let end_range = end_idx..end_idx + 4;
    *dists[end_range]
        .iter()
        .filter(|&&dist| dist != usize::MAX)
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> String {
    let grid = parse_grid(input);
    min_heat_loss(&grid, 1, 3).to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse_grid(input);
    min_heat_loss(&grid, 4, 10).to_string()
}
