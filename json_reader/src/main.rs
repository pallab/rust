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

    let example = glossary{
        title : String::from("Hello"),
        GlossList : vec!(
            GlossEntry{
                ID : String::from("1"),
                SortAs : String::from("up")
            }
        )
    };

    let json_str = create_json(&example);
    println!("Json from object {}", json_str);

    let obj_from_json = parse_json(&json_str);
    println!("Object from json {:?}", obj_from_json);

    let js_str_from_file = read_file_to_str("./example.json");
    println!("Json from file {}", js_str_from_file);

    let obj_from_file = parse_json(&js_str_from_file);
    println!("Obj from file {:?}", obj_from_file);
}

fn create_json(obj : &glossary) -> String {
    serde_json::to_string(obj).unwrap()
}

fn parse_json(json_str : &str) -> glossary {
    let result : glossary = serde_json::from_str(json_str).unwrap();
    result
}

fn read_file_to_str(path : &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    content
}

// fn read_json(path : &str) -> glossary{
//     let file = File::open(path).unwrap();
//     let reader = BufReader::new(file);
//     let json_parsed : glossary = serde_json::from_reader(reader).unwrap();
//
//     // alternate
//     let mut f = File::open(path).unwrap();
//     let mut content = String::new();
//     f.read_to_string(&mut content).unwrap();
//
//     let result:glossary = serde_json::from_str(&content).unwrap();
//     println!("another {:?}", result);
//
//
//     json_parsed
// }