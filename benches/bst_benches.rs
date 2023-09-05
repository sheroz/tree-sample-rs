use criterion::{criterion_group, criterion_main, Criterion};
use tree_samples_rs::binary_search_tree::*;
use tree_samples_rs::binary_tree::*;

fn benchmark_is_binary_search_tree_versions(c: &mut Criterion) {
    let root = utils::populate_balanced_binary_search_tree();

    c.bench_function("is_binary_search_tree_v1", |b| {
        b.iter(|| BinarySearchTree::is_binary_search_tree_v1(&root))
    });

    c.bench_function("is_binary_search_tree_v2", |b| {
        b.iter(|| BinarySearchTree::is_binary_search_tree_v2(&root))
    });

    c.bench_function("is_binary_search_tree_v3", |b| {
        b.iter(|| BinarySearchTree::is_binary_search_tree_v3(&root))
    });
}

criterion_group!(benches, benchmark_is_binary_search_tree_versions,);

criterion_main!(benches);
