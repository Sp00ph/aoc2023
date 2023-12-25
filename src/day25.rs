use ahash::AHashMap;
use smallvec::SmallVec;

struct Graph {
    vertices: Vec<SmallVec<[u16; 10]>>,
}
fn parse_input(input: &str) -> Graph {
    fn vertex_index<'a>(
        name: &'a str,
        indices: &mut AHashMap<&'a str, u16>,
        vertices: &mut Vec<SmallVec<[u16; 10]>>,
    ) -> u16 {
        if let Some(&index) = indices.get(name) {
            index
        } else {
            let index = vertices.len() as u16;
            indices.insert(name, index);
            vertices.push(SmallVec::new());
            index
        }
    }

    let mut indices = AHashMap::new();
    let mut vertices = Vec::new();

    for line in input.lines() {
        let (node, out) = line.split_once(':').unwrap();
        let node = vertex_index(node, &mut indices, &mut vertices);
        for edge in out.split_ascii_whitespace() {
            let dst = vertex_index(edge, &mut indices, &mut vertices);
            if !vertices[node as usize].contains(&dst) {
                vertices[node as usize].push(dst);
            }
            if !vertices[dst as usize].contains(&node) {
                vertices[dst as usize].push(node);
            }
        }
    }

    Graph { vertices }
}

struct AdjacencyMatrix {
    matrix: Vec<i32>,
    n: usize,
}

impl AdjacencyMatrix {
    fn get(&self, src: usize, dst: usize) -> i32 {
        self.matrix[src * self.n + dst]
    }

    fn set(&mut self, src: usize, dst: usize, value: i32) {
        self.matrix[src * self.n + dst] = value;
    }
}

fn make_adj_matrix(graph: &Graph) -> AdjacencyMatrix {
    let mut matrix = AdjacencyMatrix {
        matrix: vec![0; graph.vertices.len().pow(2)],
        n: graph.vertices.len(),
    };

    for (src, dsts) in graph.vertices.iter().enumerate() {
        for &dst in dsts {
            matrix.set(src, dst as usize, 1);
        }
    }

    matrix
}

fn stoer_wagner(mat: &mut AdjacencyMatrix) -> (i32, Vec<u16>) {
    let mut best = (i32::MAX, vec![]);
    let n = mat.n;
    let mut co: Vec<Vec<u16>> = vec![];
    for i in 0..n {
        co.push(vec![i as u16]);
    }

    for ph in 1..n {
        let mut w = mat.matrix[..n].to_vec();
        let (mut s, mut t) = (0, 0);
        for _ in 0..n - ph {
            w[t] = i32::MIN;
            s = t;
            t = w.iter().enumerate().max_by_key(|&(_, &x)| x).unwrap().0;
            for (i, w) in w.iter_mut().enumerate() {
                *w += mat.get(t, i);
            }
        }
        if w[t] - mat.get(t, t) < best.0 {
            best = (w[t] - mat.get(t, t), co[t].clone());
        }
        let mut tmp = std::mem::take(&mut co[s]);
        tmp.extend_from_slice(&co[t]);
        co[s] = tmp;
        for i in 0..n {
            mat.set(s, i, mat.get(s, i) + mat.get(t, i));
            mat.set(i, s, mat.get(s, i));
        }
        mat.set(0, t, i32::MIN);
    }

    best
}

pub fn part1(input: &str) -> String {
    let graph = parse_input(input);
    let result = stoer_wagner(&mut make_adj_matrix(&graph));
    (result.1.len() * (graph.vertices.len() - result.1.len())).to_string()
}

pub fn part2(_input: &str) -> String {
    String::from("Day 25 has no part 2!")
}
