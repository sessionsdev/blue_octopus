use std::env;
mod arg_parser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let parsed = arg_parser::parse_args(args);

    for arg in &parsed {
        if let Some(ref name) = arg.name {
            println!("Option Name: {}", name);
            if name == "--add" {
                println!("ADD OPTION")
            } else if name == "--remove" {
                println!("REMOVE OPTION")
            } else {
                println!("INVALID OPTION")
            }
        }
        println!("---"); // For separation between different arguments
    }
}

