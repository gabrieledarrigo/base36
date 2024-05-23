use std::{env, error::Error, process::exit};

use base36::*;

fn parse_args() -> Result<i32, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Please provide an integer number. Usage: {} <number>",
            args[0]
        );
        exit(1);
    }

    let number = args[1]
        .parse::<i32>()
        .or(Err("Please provide a valid integer number."))?;

    Ok(number)
}

fn main() -> Result<(), Box<dyn Error>> {
    let number = parse_args()?;
    let in_base36 = Base36::encode(number);

    println!("{}", in_base36);
    Ok(())
}
