use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);
    println!("Scores are {scores:?}");

    for (k, v) in &scores {
        println!("k: {}\tv: {}", k, v)
    }

    scores.entry("blue".to_string()).or_insert(50);
    scores.entry("red".to_string()).or_insert(5);

    println!("Scores are {scores:?}");

    let mut word_count = HashMap::new();
    let plain_test = "Hello world , this is a greeting from rust . Hello !";
    for word in plain_test.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("The wor count result is:\n{word_count:#?}")
}
