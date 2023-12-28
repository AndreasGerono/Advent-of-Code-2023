use std::collections::HashSet;
use std::{env, error::Error, fs, process};

struct Config {
    input_file: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough input parameters");
        }

        let input_file = args[1].clone();

        Ok(Config { input_file })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let input_file = &config.input_file;

    let content = fs::read_to_string(input_file)
        .or_else(|e| Err(format!("Failed to open '{}': {}", input_file, e)))?;

    let mut total_points = 0;
    for line in content.lines() {
        let mut line_split = line.split(": ");
        let _card_w_id = line_split.next().ok_or("missing `:`")?;
        let card_data = line_split.next().ok_or("missing card data")?;
        let mut card_data_split = card_data.split("| ");
        let winning_nums_str = card_data_split.next().ok_or("missing data separator `|`")?;
        let your_nums_str = card_data_split.next().ok_or("missing your numbers `|`")?;

        let winning_nums = winning_nums_str
            .split(" ")
            .filter_map(|str| str.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let your_nums = your_nums_str
            .split(" ")
            .filter_map(|str| str.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let game_points =
            your_nums.intersection(&winning_nums).fold(
                0,
                |acc, _| {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                },
            );

        total_points += game_points;
    }

    println!("Total {}", total_points);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Failed to build config: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application failed with: {}", e);
        process::exit(1);
    }
}
