use cp_rs::io::*;
use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    let mut io = Io::from_file("day12.txt");
    let mut grid = vec![];
    let mut start_idx = 0;
    let mut end_idx = 0;
    for mut line in io.line_io() {
        let line = line.chars();
        grid.push(
            line.iter()
                .enumerate()
                .map(|(i, c)| match c {
                    'S' => {
                        start_idx = i;
                        0
                    }
                    'E' => {
                        end_idx = i;
                        'z' as u8 - 'a' as u8
                    }
                    num => *num as u8 - 'a' as u8,
                })
                .collect::<Vec<u8>>(),
        );
    }
    let mut nodes: Vec<Node> = Vec::with_capacity(grid.len() * grid[0].len());
    let mut edges: Vec<Edge> = Vec::with_capacity(4 * grid.len() * grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let idx = row * grid[0].len() + col;
            let new_node: Node = idx.into();
            nodes.push(new_node);
            let num = grid[row][col];
            // up
            if row >= 1 && (grid[row - 1][col]) <= num + 1 {
                let o_idx: usize = (row - 1) * grid[0].len() + col;
                edges.push((idx, o_idx).into());
            }
            // down
            if row + 1 < grid.len() && (grid[row + 1][col]) <= num + 1 {
                let o_idx: usize = (row + 1) * grid[0].len() + col;
                edges.push((idx, o_idx).into());
            }
            // right
            if col + 1 < grid[0].len() && (grid[row][col + 1]) <= num + 1 {
                let o_idx: usize = (row) * grid[0].len() + col + 1;
                edges.push((idx, o_idx).into());
            }
            // left
            if col >= 1 && (grid[row][col - 1]) <= num + 1 {
                let o_idx: usize = (row) * grid[0].len() + col - 1;
                edges.push((idx, o_idx).into());
            }
        }
    }

    let graph: Graph = Graph::new(nodes, edges);
    let grid = grid.into_iter().flatten().collect::<Vec<u8>>();
    let res1 = bfs(&grid, &graph, start_idx.into(), end_idx.into(), false).unwrap();
    dbg!("yay");
    let res2 = bfs(&grid, &graph, end_idx.into(), Node(0 as u32), true).unwrap();
    io.write("Part 1: ");
    io.writeln(res1);
    io.write("Part 2: ");
    io.writeln(res2);
}

pub fn bfs(grid: &Vec<u8>, graph: &Graph, root: Node, target: Node, rev: bool) -> Option<u32> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(root);
    queue.push_back(root);
    queue.push_back(u32::MAX.into());
    let mut depth = 0;
    while let Some(currentnode) = queue.pop_front() {
        if currentnode.value() == u32::MAX {
            depth += 1;
            queue.push_back(u32::MAX.into());
            continue;
        }
        if rev {
            if grid[currentnode.value() as usize] == 0 {
                Some(depth);
            }
            for neighbor in currentnode.rev_neighbors(graph) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        } else {
            if currentnode == target {
                Some(depth);
            }
            for neighbor in currentnode.neighbors(graph) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }
    None
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }
}

impl From<usize> for Node {
    fn from(item: usize) -> Self {
        Node(item as u32)
    }
}
impl From<u32> for Node {
    fn from(item: u32) -> Self {
        Node(item)
    }
}

impl Node {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
    pub fn rev_neighbors(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edges
            .iter()
            .filter(|e| e.1 == self.0)
            .map(|e| e.0.into())
            .collect()
    }
}

impl From<(usize, usize)> for Edge {
    fn from(item: (usize, usize)) -> Self {
        Edge(item.0 as u32, item.1 as u32)
    }
}
