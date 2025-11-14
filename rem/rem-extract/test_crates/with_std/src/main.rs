use std::collections::HashMap;

fn main() {
    let words = vec!["hello", "world", "hello", "rust"];
    let map = count_words(words);
    println!("{:?}", map);
}

fn count_words(words: Vec<&str>) -> HashMap<String, usize> {
    let mut freq = HashMap::new();

    for w in words {
        let entry = freq.entry(w.to_string()).or_insert(0);
        *entry += 1;
    }

    freq
}
