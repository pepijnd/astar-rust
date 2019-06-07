use hashbrown::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node {
    x: i64,
    y: i64,
}

impl Node {
    pub fn new(x: i64, y: i64) -> Node {
        return Node { x, y };
    }

    pub fn dist(&self, other: &Node) -> f64 {
        return (((self.x - other.x).abs().pow(2) +
            (self.y - other.y).abs().pow(2)
        ) as f64).sqrt();
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Size { w: i64, h: i64 }

impl Size {
    pub fn new(w: i64, h: i64) -> Size {
        return Size { w, h };
    }
}

type ScoreMap = Vec<(Node, i64)>;
type RouteMap = HashMap<Node, Node>;
type NodeSet = HashSet<Node>;

trait InsertScore {
    fn insert_score(&mut self, node: Node, score: i64);
    fn remove_score(&mut self, node: &Node) -> bool;
    fn get_score(&self, node: &Node) -> Option<i64>;
}

impl InsertScore for ScoreMap {
    fn insert_score(&mut self, node: Node, score: i64) {
        for n in self.iter().enumerate() {
            if score < (n.1).1 {
                self.insert(n.0, (node, score));
                return;
            }
        }
        self.push((node, score));
    }

    fn remove_score(&mut self, node: &Node) -> bool {
        for n in self.iter().enumerate() {
            if
            (n.1).0 == *node {
                self.remove(n.0);
                return true;
            }
        }
        return false;
    }

    fn get_score(&self, node: &Node) -> Option<i64> {
        for n in self {
            if node == &n.0 { return Some(n.1); }
        }
        return None;
    }
}

pub struct AStar {
    size: Size,
    grid: Vec<i64>,
    start: Node,
    end: Node,
}

impl AStar {
    pub fn new(size: Size, start: Node, end: Node) -> AStar {
        let mut grid = Vec::new();
        let mut rng = rand::thread_rng();
        let length = size.w * size.h;
        for _n in 0..length {
            grid.push(rng.gen_range(0, 100));
        }
        return AStar { size, grid, start, end };
    }


    pub fn calc(&self) -> Option<Vec<Node>> {
        let mut set_open: NodeSet = HashSet::new();
        let mut set_closed: NodeSet = HashSet::new();
        let mut map_route: RouteMap = HashMap::new();
        let mut map_gscore: ScoreMap = Vec::with_capacity((self.size.w * self.size.h) as usize);
        let mut map_fscore: ScoreMap = Vec::with_capacity((self.size.w * self.size.h) as usize);

        let mut current: Node;

        set_open.insert(self.start);
        map_gscore.insert_score(self.start, 0);
        map_fscore.insert_score(self.start, self.estimate_cost(&self.start, &self.end));

        while set_open.len() != 0 {
            current = AStar::get_lowest(&set_open, &map_fscore);
            if current == self.end {
                return Some(self.reconstruct(map_route, current));
            }

            set_open.remove(&current);
            map_gscore.remove_score(&current);
            map_fscore.remove_score(&current);
            set_closed.insert(current);
            let neighbors = self.get_neighbors(current);
            for neighbor in neighbors {
                if set_closed.contains(&neighbor) { continue; };
                let tentative_score = map_gscore.get_score(&neighbor).unwrap_or(0) * 10 +
                    ((current.dist(&neighbor) * 10.0) as i64);
                if !set_open.insert(neighbor) {
                    if tentative_score >= map_gscore.get_score(&neighbor).unwrap_or(0) { continue; }
                }
                map_route.insert(neighbor, current);
                map_gscore.insert_score(neighbor, tentative_score);
                map_fscore.insert_score(neighbor, tentative_score
                    + self.estimate_cost(&neighbor, &self.end));
            }
        }
        return None;
    }

    fn node_idx(&self, node: &Node) -> i64 {
        return self.size.w * node.y + node.x;
    }

    fn estimate_cost(&self, node1: &Node, node2: &Node) -> i64 {
        return (node1.x - node2.x).abs() +
            (node1.y - node2.y).abs() +
            self.grid[self.node_idx(node1) as usize] +
            self.grid[self.node_idx(node2) as usize];
    }

    fn get_neighbors(&self, node: Node) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if (x != 0 || y != 0) &&
                    (node.x + x >= 0 && node.x + x < self.size.w) &&
                    (node.y + y >= 0 && node.y + y < self.size.h) {
                    nodes.push(Node { x: node.x + x, y: node.y + y });
                }
            }
        }

        return nodes;
    }

    fn get_lowest(set_nodes: &NodeSet, map_scores: &ScoreMap) -> Node {
        for (node, _score) in map_scores {
            if set_nodes.contains(node) {
                return *node;
            }
        }
        return map_scores[0].0;
    }

    fn reconstruct(&self, map_route: RouteMap, node: Node) -> Vec<Node> {
        let mut total_path: Vec<Node> = Vec::new();
        let mut current = node;


        total_path.push(current);
        while map_route.contains_key(&current) {
            current = *map_route.get(&current).unwrap();
            total_path.push(current);
        }
        return total_path;
    }
}

