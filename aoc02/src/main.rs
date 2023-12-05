use anyhow::{ensure, Context};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
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
    fn parse(game_id: u32, trial_idx: usize, trial: &str) -> anyhow::Result<Self> {
        
        // TODO: Make closure!
        fn error_msg(game_id: u32, trial_idx: usize, msg: &str) -> String {
            format!("game ID: {}, trial index: {}: {}", game_id, trial_idx, msg)
        }

        let cubes = trial.split(',').collect::<Vec<_>>();
        ensure!(cubes.len() > 0, error_msg("at least 1 cube choice ")
            format!(
                "game ID: {}, trial index: {}: at least 1 cube choice must be present in a trial",
                game_id, trial_idx
            )
        );
        for cube in cubes {
            let (number, color) = cube.split_once(' ').ok_or(
                format"malformed cube choice")?;
            let number = number
                .parse::<u32>()
                .context("failed to parse number of cubes in cube choice")?;
            let color = color.trim();
            trials.push(())
        }
    }
}

fn parse_line(line: &str) -> anyhow::Result<Vec<(&str, u32)>> {
    //    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    let (game_id, trials) = line.split_once(':').ok_or("missing colon")?;

    let (_, game_id) = game_id
        .split_once(' ')
        .ok_or("missing space in game label")?;
    let game_id = game_id.parse::<u32>().context("failed to parse game ID")?;

    let trial_strs = trials.split(';').collect::<Vec<_>>();
    let mut trials = Vec::new();
    for trial in trial_strs {
        let cubes = trial.split(',').collect::<Vec<_>>();
        for cube in cubes {
            let (number, color) = cube.split_once(' ').ok_or("missing space in cube choice")?;
            let number = number
                .parse::<u32>()
                .context("failed to parse number of cubes")?;
            trials.push(())
        }
    }

    unimplemented!()
}
mod part1 {
    use super::*;

    fn part1(input: &str) -> Result<()> {
        for line in input.lines() {}
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }
}

fn part2(input: &str) -> Result<()> {
    writeln!(io::stdout(), "{}", input)?;
    Ok(())
}
