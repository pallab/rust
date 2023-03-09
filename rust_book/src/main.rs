fn main() {
    println!("Hello, world!");
    try_hash_map();
}

fn try_hash_map() -> () {
    let text = "this is a text which is a good text";
    let mut hmap: HashMap<String, i32> = std::collections::HashMap::new();

    for word in text.split_whitespace() {
        let mut count = hmap.entry(word.to_string()).or_insert(0);
        *count+=1;
    }

    println!("{:?}", hmap);

    for (k, v) in hmap {
        println!("{k} -> {v}")
    }

}