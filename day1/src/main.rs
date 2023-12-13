use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    input_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let input_path = args[1].clone();
        Ok(Config { input_path })
    }
}

fn process_line(line: &str) -> Result<u32, Box<dyn Error>> {
    let mut only_digits = line.chars().filter(|c| c.is_digit(10));

    let first_digit_char = only_digits.next().ok_or("Did not found any digit")?;
    let mut last_digit_char = first_digit_char;

    if let Some(digit) = only_digits.last() {
        last_digit_char = digit;
    }

    let result = format!("{}{}", first_digit_char, last_digit_char)
        .parse()
        .unwrap();

    println!("{} {}", line, result);

    Ok(result)
}

fn replace_word_nums(line: &str) -> String {
    let digits = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut indexes = Vec::new();

    for i in 1..digits.len() {
        for (index, _) in line.match_indices(digits[i]) {
            indexes.push((index, i));
        }
    }

    indexes.sort();
    let mut new_line = line.to_string();
    for (cnt, (index, i)) in indexes.iter().enumerate() {
        new_line.insert(index + cnt, char::from_digit(*i as u32, 10).unwrap());
    }

    new_line
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input_path)?;
    let mut result = 0;
    let mut result2 = 0;
    for line in contents.lines() {
        result2 += process_line(&replace_word_nums(line))?;
        result += process_line(line)?;
    }
    println!("Result: {}", result);
    println!("Result 2: {}", result2);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem with parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
