fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let key = String::from("key");
    let value = String::from("value");
    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{} {}", key, value);
    println!("{}", map[&key]);
    println!("{}", map[&String::from("key")]);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{}", scores[&String::from("Blue")]);

    scores.entry(String::from("Yellow")).or_insert(999);
    scores.entry(String::from("Green")).or_insert(999);
    println!("{:#?}", scores);

    let text = "hello world nice world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}

use std::collections::HashMap;