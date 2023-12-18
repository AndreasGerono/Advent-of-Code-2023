use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::str::FromStr;
use std::usize;

type Row = Vec<char>;

struct Grid {
    rows: Vec<Row>,
    width: usize,
    height: usize,
    numbers: Vec<Number>,
    symbols: Vec<(char, usize, usize)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    indexes: Vec<(usize, usize)>,
}

impl Number {
    fn includes(&self, index: (usize, usize)) -> bool {
        self.indexes.contains(&index)
    }
}

trait Symbol {
    fn is_symbol(&self) -> bool;
}

impl Symbol for char {
    fn is_symbol(&self) -> bool {
        !self.is_digit(10) && *self != '.'
    }
}

impl Grid {
    fn neighbors(&self, c: usize, r: usize) -> Vec<(char, usize, usize)> {
        let mut neighbors = Vec::new();
        let c = c as i32;
        let r = r as i32;

        let n_indexes = [
            (c + 1, r + 1),
            (c + 1, r),
            (c + 1, r - 1),
            (c, r + 1),
            (c, r - 1),
            (c - 1, r + 1),
            (c - 1, r),
            (c - 1, r - 1),
        ];

        for (c, r) in n_indexes {
            if c < 0 || r < 0 {
                continue;
            }
            if let Some(&elem) = self
                .rows
                .get(c as usize)
                .and_then(|row| row.get(r as usize))
            {
                neighbors.push((elem, c as usize, r as usize));
            }
        }
        neighbors
    }

    fn process(&mut self) {
        for c in 0..self.height {
            let mut str_number = String::new();
            let mut indexes = Vec::new();
            for r in 0..self.width {
                let current = self.rows[c][r];
                match current {
                    ch if ch.is_digit(10) => {
                        str_number = format!("{}{}", str_number, ch);
                        indexes.push((c, r));
                        if r == (self.width - 1) {
                            let value: u32 = str_number.parse().unwrap();
                            self.numbers.push(Number {
                                value,
                                indexes: indexes.clone(),
                            });
                        }
                    }
                    ch => {
                        if ch.is_symbol() {
                            self.symbols.push((ch, c, r));
                        }
                        if !str_number.is_empty() {
                            let value: u32 = str_number.parse().unwrap();
                            self.numbers.push(Number {
                                value,
                                indexes: indexes.clone(),
                            });
                        }
                        indexes = Vec::new();
                        str_number = String::new();
                    }
                }
            }
        }
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Row> = s.lines().map(|line| line.chars().collect()).collect();

        let height = rows.len();
        let width = rows[0].len();
        let numbers = Vec::new();
        let symbols = Vec::new();

        Ok(Grid {
            rows,
            width,
            height,
            numbers,
            symbols,
        })
    }
}

#[derive(Debug)]
struct Config {
    file_name: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough input arguments");
        }

        let file_name = args[1].to_string();

        Ok(Config { file_name })
    }
}

fn part_one(grid: &Grid) -> u32 {
    grid.numbers
        .iter()
        .filter(|number| {
            number.indexes.iter().any(|(c, r)| {
                grid.neighbors(*c, *r)
                    .iter()
                    .any(|(ch, _, _)| ch.is_symbol())
            })
        })
        .map(|num| num.value)
        .sum()
}

fn part_two(grid: &Grid) -> u32 {
    grid.symbols
        .iter()
        .filter(|(ch, _, _)| *ch == '*')
        .filter_map(|(_, c, r)| {
            Some(
                grid.neighbors(*c, *r)
                    .iter()
                    .filter_map(|(_, c, r)| grid.numbers.iter().find(|num| num.includes((*c, *r))))
                    .collect::<Vec<&Number>>(),
            )
        })
        .map(|mut v| {
            v.dedup();
            v
        })
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().map(|n| n.value).product::<u32>())
        .sum()
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_name = config.file_name.to_string();
    let content = fs::read_to_string(file_name)?;
    let mut grid = content.parse::<Grid>()?;
    grid.process();

    let parts_sum = part_one(&grid);
    let gear_sum = part_two(&grid);

    println!("Sum gear: {}", gear_sum);
    println!("{:?}", parts_sum);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Build config error: {}", e);
        process::exit(1);
    });

    if let Err(e) = run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
