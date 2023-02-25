use serde;
use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct GlossEntry {
    ID : String,
    SortAs : String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct glossary {
    title : String,
    GlossList : Vec<GlossEntry>
}

fn main() {
    let parsed = read_json("./example.json");
    println!("{:?}", parsed);
}

fn read_json(path : &str) -> glossary{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let json_parsed : glossary = serde_json::from_reader(reader).unwrap();

    // alternate
    let mut f = File::open(path).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let result:glossary = serde_json::from_str(&content).unwrap();
    println!("another {:?}", result);

    
    json_parsed
}