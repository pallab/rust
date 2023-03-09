use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // mean_median_mode();
    pigletin();
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

fn mean_median_mode() {
    let text = "apple banana apple guava cherry berry cherry apple orange kiwi kiwi berry cherry cherry";
    let mut wordmap : HashMap<String, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let mut counter = wordmap.entry(word.to_string()).or_insert(0);
        *counter +=1;
    }

    let sum = wordmap.values().fold(0, |accm, e|  accm +e);
    let sum_alt = wordmap.values().copied().reduce(|accm, e| accm + e ).unwrap_or(0);
    let sum_alt_alt : i32 = wordmap.values().sum();

    let entries = wordmap.len();
    let mean = sum as f32 / entries as f32;

    println!("Entries {entries}, Sum {sum} or {sum_alt} or {sum_alt_alt} and Mean {mean}");

    // conver a hashmap to vector of tuple

    //one way
    let mut tuple_vec : Vec<(String, i32)> = Vec::new();
    for e in wordmap.iter() {
        tuple_vec.push((e.0.clone(), e.1.clone()));
    }
    println!("Vec from HashMap {:?}", tuple_vec);

    //another way
    let mut tuple_vec_sorted: Vec<(String,i32)> = wordmap.iter().map(|(a, b)| (a.clone(),b.clone())).collect();
    tuple_vec_sorted.sort_by(|a,b| a.clone().1.partial_cmp(&b.clone().1).unwrap());

    println!("Sorted Vec from HashMap {:?}", tuple_vec_sorted);

    if wordmap.len() %2 == 0 {
        println!("Median not available for even length collection");
    } else {
        let middle_index  = wordmap.len()/2;
        let median = tuple_vec_sorted.get(middle_index);
        println!("Median @ index {middle_index} is {:?} ", median)
    }

    let mode = tuple_vec_sorted.last().unwrap();
    println!("Mode is {:?}", mode);

}

fn pigletin() {

    let text = "aron is why we have oranges up that shelve";

    fn mangle_string(word: &str) -> String{
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        if(vowels.iter().cloned().find(|c| word.starts_with(c.clone())).is_some()){
            format!("{word}-hay")
        } else {
            let mut chars: Vec<char> = word.chars().collect();
            chars.swap(0, word.len()-1);
            let swapped_str = String::from_iter(chars);
            format!("{swapped_str}-ay")
        }
    }

    let words : Vec<&str> = text.split_whitespace().collect();

    let piglets : Vec<String> = words.iter().map(|word| mangle_string(word)).collect();

    let pigletin = piglets.join(" ");
    println!("{text} \n{pigletin}")
}