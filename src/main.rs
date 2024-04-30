use std::{env, error::Error, process::exit};

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

fn to_base36(number: i32) -> String {
    let mut number = number;
    let charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let charset_len = charset.len() as i32;

    let mut text = String::new();

    let num_digits = number.ilog10() / charset_len.ilog10();

    for _ in 0..num_digits {
        let char = charset
            .chars()
            .nth((number % charset_len) as usize)
            .unwrap();

        text.push(char);
        number /= charset_len;
    }

    text
}

fn main() -> Result<(), Box<dyn Error>> {
    let number = parse_args()?;
    let in_base36 = to_base36(number);

    println!("{}", in_base36);
    Ok(())
}
