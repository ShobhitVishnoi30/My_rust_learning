use cli_app::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error occured : {:}", err);
        process::exit(1);
    });

    println!("Searching for {:?}", config.query);
    println!("Searching in {:?}", config.file_name);

    if let Err(e) = cli_app::run(config) {
        eprintln!("Error occured : {:}", e);
        process::exit(1);
    };
}
