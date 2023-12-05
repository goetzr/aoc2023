use anyhow::{anyhow, ensure, Context};
use std::collections::HashSet;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

struct Game {
    id: u32,
    trials: Vec<Trial>,
}

struct Trial {
    red: u32,
    green: u32,
    blue: u32,
}

impl Trial {
    fn parse(game_id: u32, trial_idx: usize, trial: &str) -> anyhow::Result<Self> {
        let msg = |msg: &str| format!("game ID: {}, trial index: {}: {}", game_id, trial_idx, msg);

        let mut trial = Trial {
            red: 0,
            green: 0,
            blue: 0,
        };
        let mut colors_seen = HashSet::new();
        let cubes = trial.split(',').collect::<Vec<_>>();
        ensure!(
            cubes.len() > 0,
            msg("at least 1 cube choice must be present in a trial")
        );
        for cube in cubes {
            let (number, color) = cube
                .split_once(' ')
                .ok_or(anyhow!(msg("cube choice not space-separated")))?;
            let number = number
                .parse::<u32>()
                .context("failed to parse number of cubes in cube choice")?;
            let color = color.trim();
            ensure!(
                colors_seen.insert(color),
                msg("duplicate color in cube choice")
            );

            match color {
                "red" => trial.red = number,
                "green" => trial.green = number,
                "blue" => trial.blue = number,
            };
        }
        Ok(trial)
    }
}

fn parse_line(line: &str) -> anyhow::Result<Vec<Game>> {
    //    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    let (game_label, trials) = line.split_once(':').ok_or(anyhow!("missing colon"))?;

    let (_, game_id) = game_label
        .split_once(' ')
        .ok_or("missing space in game label")?;
    let game_id = game_id
        .parse::<u32>()
        .context("failed to parse game ID from game label")?;

    let trial_strs = trials.split(';').collect::<Vec<_>>();
    let mut trials = Vec::new();
    for (trial_idx, trial) in trial_strs.iter().enumerate() {
        trials.push(Trial::parse(game_id, trial_idx, trial)?);
    }

    let game = Game { id: game_id, trials };
    Ok(game)
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
