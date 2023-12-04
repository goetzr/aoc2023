use std::io::{self, Write};

type Error = dyn std::error::Error + 'static;
type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

struct Trial {
    red: u32,
    green: u32,
    blue: u32,
}

impl Trial {
    fn new() -> Self {
        Trial { red: 0, green: 0, blue: 0 }
    }
}

fn parse_line(line: &str) -> Result<Vec<Trial>> {
    let (game_id, trials_str) = line.split_once(':').ok_or("missing colon")?;

    let (_, game_id) = game_id.split_once(' ').ok_or("missing space in game label")?;
    // TODO: How to label this error?
    let game_id = game_id.parse::<u32>()?;

    let mut trials = Vec::new();
    let trials_strs = trials_str.split(';').collect::<Vec<_>>();
    for trial in trials_strs {
        let cube_choices = trial.split(',').collect::<Vec<_>>();
        let mut trial = Trial::new();
        for cube_choice in cube_choices {
            let (number, color) = cube_choice.split_once(' ').ok_or("missing space in cube choice")?;
            match color {
                "red" => 
            }
        }
    }
    
    unimplemented!()
}
mod part1 {
    use super::*;

    fn part1(input: &str) -> Result<()> {
        for line in input.lines() {

        }
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }

}

fn part2(input: &str) -> Result<()> {
    writeln!(io::stdout(), "{}", input)?;
    Ok(())
}