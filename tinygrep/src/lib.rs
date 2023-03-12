use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub query: String,
}

impl Config {
    pub fn parse(args: &Vec<String>) -> Result<Config, String> {
        if &args.len() != &(3 as usize) {
            return Err("Please provide exactly two arguments. Ex : <file> <expression>".to_string());
        }

        Ok(
            Config {
                file: args[1].clone(),
                query: args[2].clone(),
            }
        )
    }
}

pub fn find_in_file(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(config.file)?;
    let matched_lines = search(&config.query, &content)
        .iter().map(|&l| l.to_owned()).collect();
    Ok(matched_lines)
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    return content.lines().filter(|&line| line.to_lowercase().contains(query)).collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_query() {
        let content = "\
Rust
is a safe, productive
language
        ";
        let query = "duct";
        assert_eq!(vec!["is a safe, productive"], search(query, content))
    }
}