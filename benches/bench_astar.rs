#![feature(test)]

extern crate astar_rust;
extern crate test;

const NSIZE: i64 = 64;

mod hash {
    use astar_rust::algo::hash::{AStar, Node, Size};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_astar(b: &mut Bencher) {
        let nsize = super::NSIZE;
        b.iter(|| {
            let bench = black_box(AStar::new(
                Size::new(nsize, nsize),
                Node::new(0, 0),
                Node::new(nsize - 1, nsize - 1),
            ));
            bench.calc();
        })
    }
}

mod index {
    use astar_rust::algo::index::{AStar, Node, Size};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_astar(b: &mut Bencher) {
        let nsize = super::NSIZE;
        b.iter(|| {
            let bench = black_box(AStar::new(
                Size::new(nsize, nsize),
                Node::new(0, 0),
                Node::new(nsize - 1, nsize - 1),
            ));
            bench.calc();
        })
    }
}
