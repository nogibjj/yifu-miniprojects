use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a string as a command line argument.");
        return;
    }

    let input = &args[1];

    let mut word_counts = std::collections::HashMap::new();

    for word in input.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    let mut count_vec: Vec<(&String, &u32)> = word_counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (i, (word, count)) in count_vec.iter().take(10).enumerate() {
        println!("{:2}. {:<10} {:>4}", i + 1, word, count);
    }
}
