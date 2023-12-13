use ahash::AHashMap as Map;

#[derive(Debug)]
enum Inst {
    Left,
    Right,
}

#[derive(Debug)]
struct Network<'a> {
    indices: Map<&'a str, u16>,
    nodes: Vec<(u16, u16)>,
}

fn parse_input(input: &str) -> (Vec<Inst>, Network<'_>) {
    let (insts, network) = input.trim().split_once('\n').unwrap();
    let insts = insts
        .trim_end()
        .bytes()
        .map(|b| match b {
            b'L' => Inst::Left,
            b'R' => Inst::Right,
            _ => panic!("invalid instruction"),
        })
        .collect();

    let mut indices = Map::new();
    let mut nodes = Vec::new();

    fn index<'a>(
        name: &'a str,
        indices: &mut Map<&'a str, u16>,
        nodes: &mut Vec<(u16, u16)>,
    ) -> u16 {
        if let Some(&i) = indices.get(name) {
            i
        } else {
            let i = nodes.len() as u16;
            indices.insert(name, i);
            nodes.push((0, 0));
            i
        }
    }

    for line in network.trim_start().lines() {
        let (node, neighbors) = line.split_once(" = (").unwrap();
        let (left, right) = neighbors
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        let node = index(node, &mut indices, &mut nodes);
        let left = index(left, &mut indices, &mut nodes);
        let right = index(right, &mut indices, &mut nodes);
        nodes[node as usize] = (left, right);
    }

    (insts, Network { indices, nodes })
}

fn count_steps(
    insts: &[Inst],
    network: &Network<'_>,
    start: u16,
    end: impl Fn(u16) -> bool,
) -> usize {
    let mut cur = start;

    for (i, inst) in insts.iter().cycle().enumerate() {
        if end(cur) {
            return i;
        }
        let (left, right) = network.nodes[cur as usize];
        cur = match inst {
            Inst::Left => left,
            Inst::Right => right,
        };
    }

    unreachable!()
}

pub fn part1(input: &str) -> String {
    let (insts, network) = parse_input(input);
    let start = network.indices["AAA"];
    let end = network.indices["ZZZ"];
    count_steps(&insts, &network, start, |i| i == end).to_string()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part2(input: &str) -> String {
    let (insts, network) = parse_input(input);
    // There's so few end vertices (6 for my input) that a linear scan
    // over a vector is faster than a hash set lookup.
    let end: Vec<u16> = network
        .indices
        .iter()
        .filter(|(n, _)| n.ends_with('Z'))
        .map(|(_, &i)| i)
        .collect();

    let start = network
        .indices
        .iter()
        .filter(|(n, _)| n.ends_with('A'))
        .map(|(_, &i)| i);

    start
        .map(|start| count_steps(&insts, &network, start, |i| end.contains(&i)))
        .fold(1usize, lcm)
        .to_string()
}
