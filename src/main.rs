mod algo;
use algo::*;

const NSIZE: i64 = 16;

fn main() {
    let astar = index::AStar::new(
        index::Size::new(NSIZE, NSIZE),
        index::Node::new(0, 0),
        index::Node::new(NSIZE - 1, NSIZE - 1)
    );
    let result = astar.calc().unwrap();
    println!("{:?}", result);

    let astar = hash::AStar::new(
        hash::Size::new(NSIZE, NSIZE),
        hash::Node::new(0, 0),
        hash::Node::new(NSIZE - 1, NSIZE - 1)
    );
    let result = astar.calc().unwrap();
    println!("{:?}", result);
}