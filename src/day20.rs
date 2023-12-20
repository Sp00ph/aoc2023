use std::collections::VecDeque;

use ahash::AHashMap;
use smallvec::SmallVec;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Module {
    FlipFlop(bool),
    // The input has less than 64 nodes, so storing 64
    // bits is always enough. We can just fill up all
    // unused bits with 1s.
    Conjunction(u64),
    Broadcast,
    Output,
}
#[derive(Clone, PartialEq, Eq)]
struct Network {
    modules: Vec<Module>,
    // The max outdegree seems to be 7, so we use a SmallVec
    // to avoid heap allocations. If there was ever a node
    // with a higher outdegree, it would just fall back to allocating.
    connections: Vec<SmallVec<[usize; 7]>>,
    preds: Vec<SmallVec<[usize; 7]>>,
    broadcast_idx: usize,
    rx_idx: Option<usize>,
}

fn parse_network(input: &str) -> Network {
    let mut modules = Vec::new();
    // We only need this map during parsing, to find
    // the index associated with a node.
    let mut indices = AHashMap::new();
    let mut preds = Vec::new();
    let mut connections = Vec::new();

    // First pass: parse all nodes and create the indices.
    for line in input.lines() {
        let (label, _) = line.split_once(" -> ").unwrap();
        let (label, module) = if label == "broadcaster" {
            (label, Module::Broadcast)
        } else if let Some(label) = label.strip_prefix('%') {
            (label, Module::FlipFlop(false))
        } else {
            (
                label.strip_prefix('&').unwrap(),
                // We initialize the conjunctions with all bits set,
                // and set its predecessors bits to 0 during the second pass.
                Module::Conjunction(u64::MAX),
            )
        };
        indices.insert(label, modules.len());
        modules.push(module);
        preds.push(SmallVec::new());
    }

    // Second pass: parse all connections and initialize conjunction bitsets.
    for line in input.lines() {
        let (label, out) = line.split_once(" -> ").unwrap();
        let label = label.trim_start_matches(['%', '&']);
        let idx = indices[label];
        let out_edges = out.split(", ");
        let mut out_indices = SmallVec::new();
        for out_edge in out_edges {
            // If the dest node doesn't exist, then it's an output node.
            // We can just create it on the fly.
            let out_idx = match indices.get(out_edge) {
                Some(&idx) => idx,
                None => {
                    let idx = modules.len();
                    indices.insert(out_edge, idx);
                    modules.push(Module::Output);
                    preds.push(SmallVec::new());
                    idx
                }
            };
            out_indices.push(out_idx);
            preds[out_idx].push(idx);
            // If the dest node is a conjunction, then we need to clear
            // its bit corresponding to the source node.
            if let Module::Conjunction(mask) = &mut modules[out_idx] {
                *mask &= !(1u64.checked_shl(idx as u32).expect("too many nodes"));
            }
        }
        connections.push(out_indices);
    }

    Network {
        modules,
        connections,
        preds,
        broadcast_idx: indices["broadcaster"],
        rx_idx: indices.get("rx").copied(),
    }
}

pub fn part1(input: &str) -> String {
    let mut network = parse_network(input);
    let mut queue = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        queue.push_back((usize::MAX, network.broadcast_idx, Pulse::Low));
        low_pulses += 1;
        while let Some((pred, node_idx, pulse)) = queue.pop_front() {
            let out_signal = match &mut network.modules[node_idx] {
                Module::FlipFlop(b) => {
                    if pulse == Pulse::High {
                        continue;
                    }
                    if *b {
                        *b = false;
                        Pulse::Low
                    } else {
                        *b = true;
                        Pulse::High
                    }
                }
                Module::Conjunction(mask) => {
                    let bit = 1u64 << pred;
                    if pulse == Pulse::Low {
                        *mask &= !bit;
                    } else {
                        *mask |= bit;
                    }
                    if *mask == u64::MAX {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
                Module::Broadcast => pulse,
                Module::Output => continue,
            };
            for &out_idx in &network.connections[node_idx] {
                queue.push_back((node_idx, out_idx, out_signal));
            }
            if out_signal == Pulse::Low {
                low_pulses += network.connections[node_idx].len();
            } else {
                high_pulses += network.connections[node_idx].len();
            }
        }
    }

    (low_pulses * high_pulses).to_string()
}

pub fn part2(input: &str) -> String {
    let network = parse_network(input);
    // It seems that rx is always the child of a single
    // conjunction, which itself is the child of 4 conjunctions.
    // Each of those 4 grandparents lies on a separate cycle
    // of the input graph, so it's enough to find the first iteration
    // where each grandparent gets a low pulse, and then take the LCM
    // of those. This is not a general solution, but the inputs seem
    // to have been chosen to make this work.
    let rx_idx = network.rx_idx.unwrap();
    let parent = network.preds[rx_idx][0];
    let grandparents = &network.preds[parent];

    // Try to optimize the low iteration scanning as much as possible.
    // We use a bitset to find the grandparents, and a fixed-size array
    // for the low counts.
    let mut gp_bitset = grandparents
        .iter()
        .fold(0u64, |acc, &idx| acc | 1u64 << idx);
    let mut low_counts = [1; 64];

    let mut network = network.clone();
    let mut queue = VecDeque::new();
    for i in 1.. {
        if gp_bitset == 0 {
            break;
        }
        // Each queue element has the form (predecessor, node, pulse),
        // the broadcaster just gets a dummy predecessor, since it doesn't
        // care about its predecessor anyways.
        queue.push_back((usize::MAX, network.broadcast_idx, Pulse::Low));
        while let Some((pred, node_idx, pulse)) = queue.pop_front() {
            if pulse == Pulse::Low && gp_bitset & 1u64 << node_idx != 0 {
                gp_bitset &= !(1u64 << node_idx);
                low_counts[node_idx] = i;
            }
            let out_signal = match &mut network.modules[node_idx] {
                Module::FlipFlop(b) => {
                    if pulse == Pulse::High {
                        continue;
                    }
                    if *b {
                        *b = false;
                        Pulse::Low
                    } else {
                        *b = true;
                        Pulse::High
                    }
                }
                Module::Conjunction(mask) => {
                    let bit = 1u64 << pred;
                    if pulse == Pulse::Low {
                        *mask &= !bit;
                    } else {
                        *mask |= bit;
                    }
                    if *mask == u64::MAX {
                        Pulse::Low
                    } else {
                        Pulse::High
                    }
                }
                Module::Broadcast => pulse,
                Module::Output => continue,
            };
            for &out_idx in &network.connections[node_idx] {
                queue.push_back((node_idx, out_idx, out_signal));
            }
        }
    }

    // All non-grandparent nodes have a count of 1, which is the
    // identity for lcm, so we don't have to filter them out.
    low_counts
        .into_iter()
        .fold(1usize, num::integer::lcm)
        .to_string()
}
