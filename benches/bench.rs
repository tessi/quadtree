use std::iter;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use quadtree::{Point2D, QuadTree, QuadTreeOption, Rectangle};
use rand::Rng;

fn create_rootleaf_tree(elements: &[Point2D<u8>]) -> QuadTree<u8> {
    let mut quadtree = QuadTree::<u8>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
    for point in elements {
        quadtree.insert(*point).unwrap();
    }
    quadtree
}

fn create_struct_tree(elements: &[Point2D<u8>]) -> QuadTreeOption<u8> {
    let mut quadtree = QuadTreeOption::<u8>::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));
    for point in elements {
        quadtree.insert(*point).unwrap();
    }
    quadtree
}

fn insert_nodes(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("insert_nodes");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        let mut rng = rand::thread_rng();
        let points = iter::repeat_with(|| Point2D {
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
            data: 42,
        })
        .take(*size)
        .collect::<Vec<Point2D<u8>>>();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("Leaf+Root", size), size, |b, _i| {
            b.iter(|| create_rootleaf_tree(&points))
        });
        group.bench_with_input(BenchmarkId::new("Common Structs", size), size, |b, _i| {
            b.iter(|| create_struct_tree(&points))
        });
    }
    group.finish();
}

fn query_tree_leaf_root(quadtree: &QuadTree<u8>, regions: &[Rectangle]) -> usize {
    let mut sum = 0;
    for region in regions {
        sum += quadtree.query(*region).len();
    }
    sum
}

fn query_tree_struct(quadtree: &QuadTreeOption<u8>, regions: &[Rectangle]) -> usize {
    let mut sum = 0;
    for region in regions {
        sum += quadtree.query(*region).len();
    }
    sum
}

fn query_nodes(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("query_nodes");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB].iter() {
        let mut rng = rand::thread_rng();
        let points = iter::repeat_with(|| Point2D {
            x: rng.gen_range(0.0..100.0),
            y: rng.gen_range(0.0..100.0),
            data: 42,
        })
        .take(*size)
        .collect::<Vec<Point2D<u8>>>();

        let regions = iter::repeat_with(|| Rectangle::new(0.0, 0.0, 100.0, 100.0))
            .take(*size)
            .collect::<Vec<Rectangle>>();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("Leaf+Root", size), size, |b, _i| {
            let quadtree = create_rootleaf_tree(&points);
            b.iter(|| query_tree_leaf_root(&quadtree, &regions))
        });
        group.bench_with_input(BenchmarkId::new("Common Structs", size), size, |b, _i| {
            let quadtree = create_struct_tree(&points);
            b.iter(|| query_tree_struct(&quadtree, &regions))
        });
    }
    group.finish();
}

criterion_group!(benches, insert_nodes, query_nodes);
criterion_main!(benches);
