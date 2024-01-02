use std::env;
use std::fs;
mod options;
mod systems;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--cat" => options::animals::cat::cat(),
            "--h" => options::help::help(),
            _ => println!("Invalid option. Try --h for help."),
        }
    } else if args.len() == 1 {
        let content = fs::read_to_string("src/public/clipet.txt")
        .expect("Could not read file src/public/clipet.txt");
        println!("{}", content);
    }
}
