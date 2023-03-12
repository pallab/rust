use std::env;
use std::process;
use tinygrep::Config;


fn main() {
    let args : Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    dbg!(&config);

    let content = tinygrep::find_in_file(config).unwrap_or_else(|err|{
        eprintln!("Application error : {:?}", err);
        process::exit(2)
    });
    if content.is_empty() {
        println!("Found no matching lines")
    } else {
        println!("Line Matched :\n{}", content.join("\n"));
    }

}

