use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
struct Config {
    file_name: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enought arguments");
        }

        let file_name = args[1].to_string();

        Ok(Config { file_name })
    }
}

#[derive(Debug)]
enum Cube {
    Blue(u32),
    Green(u32),
    Red(u32),
}

impl Cube {
    fn new(text: &str) -> Cube {
        let mut raw_split = text.split(" ");

        let num = raw_split
            .next()
            .expect("no space in cube!")
            .parse()
            .expect("cube number not valid");

        let color = raw_split.next().expect("Wrong format in game data");

        let cube = match color {
            "blue" => Cube::Blue(num),
            "red" => Cube::Red(num),
            "green" => Cube::Green(num),
            x => panic!("'{}' is not a valid cube color", x),
        };

        cube
    }
}

type Set = Vec<Cube>;
struct Game {
    sets: Vec<Set>,
    id: u32,
}


impl Game {
    fn build(line: &str) -> Result<Game, Box<dyn Error>> {
        let mut line_split = line.split(": ");

        let id = line_split
            .next()
            .ok_or("game input error, no `:` found")?
            .split(" ")
            .nth(1)
            .ok_or("game id error, no space found")?
            .parse()
            .or_else(|_| Err("game id is not valid"))?;

        let sets = line_split
            .next()
            .ok_or("Input error, no 'data' found")?
            .split("; ")
            .map(|e| e.split(", "))
            .map(|e| e.map(|c| Cube::new(c)).collect::<Vec<Cube>>())
            .collect::<Vec<Set>>();

        Ok(Game { sets, id })
    }
    fn is_possible(&self, r: u32, g: u32, b: u32) -> bool {
        for set in &self.sets {
            for cube in set {
                match cube {
                    Cube::Blue(n) => if n > &b {return false},
                    Cube::Green(n) => if n > &g {return false},
                    Cube::Red(n) => if n > &r {return false},
                }
            }
        }
        true
    }
    fn game_power(&self) -> u32 {
        let mut max_r: Option<u32> = None;
        let mut max_b: Option<u32> = None;
        let mut max_g: Option<u32> = None;

        for set in &self.sets {
            for cube in set {
                match cube {
                    Cube::Blue(n) => if n > &max_b.unwrap_or(0) {max_b = Some(*n)},
                    Cube::Green(n) => if n > &max_g.unwrap_or(0) {max_g = Some(*n)},
                    Cube::Red(n) => if n > &max_r.unwrap_or(0) {max_r = Some(*n)},
                }
            }
        }

        let power = max_b.unwrap_or(1) * max_g.unwrap_or(1) * max_r.unwrap_or(1);
        // println!("r {:?} g {:?} b {:?} p {power}", max_r, max_g, max_b);

        power
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let mut id_sum = 0;
    let mut power_sum = 0;
    for line in contents.lines() {
        let game = Game::build(line)?;
        // 12 red cubes, 13 green cubes, and 14 b
        if game.is_possible(12, 13, 14) {
            id_sum += game.id;
        }
        power_sum += game.game_power();
    }

    println!("The id sum is: {}", id_sum);
    println!("The power sum is: {}", power_sum);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem with parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
