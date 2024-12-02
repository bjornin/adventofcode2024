use anyhow::Result;
use reqwest::header::COOKIE;
use std::{env, fs, path::Path};

pub fn get_input() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    let day_of_month = args[1].parse::<u32>().unwrap();
    println!("Day: {}", day_of_month);
    let url = format!("https://adventofcode.com/2024/day/{}/input", day_of_month);
    let file_path = format!("input/day{}.txt", day_of_month);

    let input = if Path::new(&file_path).exists() {
        fs::read_to_string(file_path)?
    } else {
        println!("Read input from {}", url);
        let cookie = env::var("COOKIE").expect("COOKIE env var not set");
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).header(COOKIE, cookie).send()?.text()?;
        fs::write(file_path, &response)?;
        response
    };
    Ok(input)
}
