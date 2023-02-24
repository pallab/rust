use std::error::Error;
use csv;


fn main() {
    if let Err(e) = read_csv("./addresses.csv"){
        eprintln!("{}", e);
    }
}

fn read_csv(path: &str) -> Result<(), Box<dyn Error>>{

    let mut reader = csv::Reader::from_path(path)?;

    for record in reader.records() {
        let result = record?;
        println!("{:?}",result);
        for c in &result {
            println!("{}", c);
        }
    }
     Ok(())
}
