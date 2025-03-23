use std::collections::HashMap;

fn main() {
    let text = "hello world hello";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", word_count);
}