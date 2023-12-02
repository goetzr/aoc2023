use std::io::{self, Read, Write};

type Error = dyn std::error::Error + 'static;
type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut code = 0;
    for line in input.lines() {
        let first = line.chars().filter_map(|c| c.to_digit(10)).next().ok_or("no numbers on line")?;
        let last = line.chars().rev().filter_map(|c| c.to_digit(10)).next().ok_or("no numbers on line")?;
        let val = first * 10 + last;
        code += val;
    }
    writeln!(io::stdout(), "{}", code)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    writeln!(io::stdout(), "{}", input)?;
    Ok(())
}