use bst::Tree;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// 还是那个生成伪随机数的辅助函数
fn generate_random_data(count: usize) -> Vec<i32> {
    let mut data = Vec::with_capacity(count);
    let mut state: u32 = 12345;
    for _ in 0..count {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        data.push((state % 10000) as i32);
    }
    data
}

fn bench_tree_inserts(c: &mut Criterion) {
    let nums = generate_random_data(1000); // 测试插入 1000 个节点

    // 创建一个测试组，方便在报告中将两者放在同一张图表里对比
    let mut group = c.benchmark_group("BST Insertions");

    // 1. 测试函数式插入 (Functional)
    group.bench_function("Functional insert", |b| {
        // b.iter 会反复执行大括号里的代码，并精确测量时间
        b.iter(|| {
            let mut tree = Tree::new();
            for &n in &nums {
                tree = tree.insert(n);
            }
            // 使用 black_box 防止编译器过度优化（比如发现 tree 没被使用就直接把循环删了）
            // black_box(tree);
        })
    });

    // 2. 测试命令式插入 (Imperative)
    group.bench_function("Imperative insert_mut", |b| {
        b.iter(|| {
            let mut tree = Tree::new();
            for &n in &nums {
                tree.insert_mut(n);
            }
            // black_box(tree);
        })
    });

    group.finish();
}

// 注册基准测试
criterion_group!(benches, bench_tree_inserts);
// 自动生成 main 函数来运行基准测试
criterion_main!(benches);
