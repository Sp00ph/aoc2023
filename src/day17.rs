use ahash::AHashMap as Map;
use std::cmp::Reverse;
use std::collections::hash_map::Entry;

use keyed_priority_queue::KeyedPriorityQueue;

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
    type Queue = KeyedPriorityQueue<Node, Reverse<usize>, ahash::RandomState>;
    type DistMap = Map<Node, usize>;

    const NORTH: u8 = 0;
    const SOUTH: u8 = 1;
    const EAST: u8 = 2;
    const WEST: u8 = 3;
    const START: u8 = 4;

    // the start node gets the special Start predecessor, so it can go either down or right.
    let mut queue = Queue::from_iter([((0, 0, START), Reverse(0))]);
    let mut dists = DistMap::new();

    fn update_dists_and_queue(queue: &mut Queue, dists: &mut DistMap, node: Node, dist: usize) {
        match dists.entry(node) {
            Entry::Vacant(entry) => {
                entry.insert(dist);
                queue.push(node, Reverse(dist));
            }
            Entry::Occupied(mut entry) => {
                if dist < *entry.get() {
                    entry.insert(dist);
                    if queue.set_priority(&node, Reverse(dist)).is_err() {
                        queue.push(node, Reverse(dist));
                    }
                }
            }
        }
    }

    while let Some(((x, y, dir), Reverse(dist))) = queue.pop() {
        // precompute the distances to the closest possible neighbors, so we don't have to do it
        // on each iteration of the loop. Unfortunately, this only saves a few milliseconds.
        let mut north_dist = (1..min_steps.min(y))
            .map(|i| grid.get(x, y - i) as usize)
            .sum::<usize>();
        let mut south_dist = (1..min_steps.min(grid.height - y - 1))
            .map(|i| grid.get(x, y + i) as usize)
            .sum::<usize>();
        let mut east_dist = (1..min_steps.min(grid.width - x - 1))
            .map(|i| grid.get(x + i, y) as usize)
            .sum::<usize>();
        let mut west_dist = (1..min_steps.min(x))
            .map(|i| grid.get(x - i, y) as usize)
            .sum::<usize>();

        for steps in min_steps..=max_steps {
            // these can both be false, if the predecessor was the start node
            let was_horizontal = dir == EAST || dir == WEST;
            let was_vertical = dir == NORTH || dir == SOUTH;

            let can_go_up = !was_vertical && y >= steps;
            if can_go_up {
                north_dist += grid.get(x, y - steps) as usize;
                let neighbor = (x, y - steps, NORTH);
                let neighbor_dist = dist + north_dist;
                update_dists_and_queue(&mut queue, &mut dists, neighbor, neighbor_dist);
            }

            let can_go_right = !was_horizontal && x + steps < grid.width;
            if can_go_right {
                east_dist += grid.get(x + steps, y) as usize;
                let neighbor = (x + steps, y, EAST);
                let neighbor_dist = dist + east_dist;
                update_dists_and_queue(&mut queue, &mut dists, neighbor, neighbor_dist);
            }

            let can_go_down = !was_vertical && y + steps < grid.height;
            if can_go_down {
                south_dist += grid.get(x, y + steps) as usize;
                let neighbor = (x, y + steps, SOUTH);
                let neighbor_dist = dist + south_dist;
                update_dists_and_queue(&mut queue, &mut dists, neighbor, neighbor_dist);
            }

            let can_go_left = !was_horizontal && x >= steps;
            if can_go_left {
                west_dist += grid.get(x - steps, y) as usize;
                let neighbor = (x - steps, y, WEST);
                let neighbor_dist = dist + west_dist;
                update_dists_and_queue(&mut queue, &mut dists, neighbor, neighbor_dist);
            }
        }
    }

    let end = (grid.width - 1, grid.height - 1);

    // filter through all the vertices that represent the end cell,
    // and find the one with the minimum distance.
    *dists
        .iter()
        .filter(|(node, _)| node.0 == end.0 && node.1 == end.1)
        .map(|(_, dist)| dist)
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
