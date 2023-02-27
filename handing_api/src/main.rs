use error_chain::error_chain;
use std::io::Read;
use reqwest;
use serde::Deserialize;
use reqwest::header::USER_AGENT;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
    id : u32,
}

fn main()  {
    blocking_request();
    async_request();
    get_stars();
}

#[tokio::main]
async fn get_stars() -> Result<()>{
    let url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner="rust-lang-nursery",
    repo = "rust-cookbook");

    let response = reqwest::Client::new().get(&url).header(USER_AGENT, "rust demo")
        .send().await?;

    let users : Vec<User> = response.json().await?;
    println!("users : {:?}", users);
    Ok(())
}


#[tokio::main]
async fn async_request() -> Result<()> {
    let result = reqwest::get("http://httpbin.org/get").await?;

    println!("Status: {}", result.status());
    println!("Headers : \n{:#?}", result.headers());

    let body = result.text().await?;
    println!("Body : \n{}", body);
    Ok(())
}


fn blocking_request() -> Result<()> {
    let mut result = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    result.read_to_string(&mut body)?;

    println!("Status: {}", result.status());
    println!("Headers : \n{:#?}", result.headers());
    println!("Body : \n{}", body);
    Ok(())
}