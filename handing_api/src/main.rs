use error_chain::error_chain;
use std::io::Read;
use reqwest;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main()  {
    blocking_request();
    async_request();
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