mod solutions {
    pub mod d1;
    pub mod d2;
    pub mod d3;
    pub mod d4;
    pub mod d5;
}
use solutions::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    let input = adventofcode2024::get_input().unwrap();
    match args[1].as_str() {
        "1" => println!("1: {} 2: {}", d1::s1(&input), d1::s2(&input)),
        "2" => println!("1: {} 2: {}", d2::s1(&input), d2::s2(&input)),
        "3" => println!("1: {} 2: {}", d3::s1(&input), d3::s2(&input)),
        "4" => println!("1: {} 2: {}", d4::s1(&input), d4::s2(&input)),
        "5" => println!("1: {} 2: {}", d5::s1(&input), d5::s2(&input)),
        _ => eprintln!("Invalid day: {}", args[1]),
    }
}
