use std::collections::HashMap;

fn main() {
    // 1. 定义数据源
    let network: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Alice", vec!["Bob", "Charlie", "David"]),
        ("Eve", vec!["Bob", "Charlie", "Frank"]),
        ("Grace", vec!["David", "Bob"]),
    ]);

    // 2. 纯迭代器流水线
    let common_friends = network
        .iter()
        // --- MAP 阶段 ---
        // flat_map 负责将一个中介人产生的多个对“拍平”到一个流中
        .flat_map(|(_, friends)| {
            let mut pairs = Vec::new();
            // 标准库手动实现两两组合
            for i in 0..friends.len() {
                for j in i + 1..friends.len() {
                    let mut pair = [friends[i].to_string(), friends[j].to_string()];
                    pair.sort(); // 核心去重：排序
                    pairs.push(((pair[0].clone(), pair[1].clone()), 1));
                }
            }
            pairs // 返回 Vec，flat_map 会自动将其元素展开
        })
        // --- REDUCE 阶段 ---
        // 使用 fold 将流中的 (Pair, 1) 聚合进 HashMap
        .fold(
            HashMap::new(),
            |mut acc: HashMap<(String, String), usize>, (pair, count)| {
                *acc.entry(pair).or_insert(0) += count;
                acc
            },
        );

    // 3. 输出结果
    for (pair, count) in common_friends {
        println!("{:?} 共有 {} 个共同好友", pair, count);
    }
}
