mod solutions {
    pub mod d1;
    pub mod d10;
    pub mod d11;
    pub mod d12;
    pub mod d13;
    pub mod d14;
    pub mod d15;
    pub mod d16;
    pub mod d17;
    pub mod d18;
    pub mod d2;
    pub mod d3;
    pub mod d4;
    pub mod d5;
    pub mod d6;
    pub mod d7;
    pub mod d8;
    pub mod d9;
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
        "6" => println!("1: {} 2: {}", d6::s1(&input), d6::s2(&input)),
        "7" => println!("1: {} 2: {}", d7::s1(&input), d7::s2(&input)),
        "8" => println!("1: {} 2: {}", d8::s1(&input), d8::s2(&input)),
        "9" => println!("1: {} 2: {}", d9::s1(&input), d9::s2(&input)),
        "10" => println!("1: {} 2: {}", d10::s1(&input), d10::s2(&input)),
        "11" => println!("1: {} 2: {}", d11::s1(&input), d11::s2(&input)),
        "12" => println!("1: {} 2: {}", d12::s1(&input), d12::s2(&input)),
        "13" => println!("1: {} 2: {}", d13::s1(&input), d13::s2(&input)),
        "14" => println!(
            "1: {} 2: {}",
            d14::s1(&input, (101, 103)),
            d14::s2(&input, (101, 103))
        ),
        "15" => println!("1: {} 2: {}", d15::s1(&input), d15::s2(&input)),
        "16" => println!("1: {} 2: {}", d16::s1(&input), d16::s2(&input)),
        "17" => println!("1: {} 2: {}", d17::s1(&input), d17::s2(&input)),
        "18" => println!(
            "1: {} 2: {:?}",
            d18::s1(&input, 70, 1024),
            d18::s2(&input, 70, 1024)
        ),
        _ => eprintln!("Invalid day: {}", args[1]),
    }
}
