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
    writeln!(io::stdout(), "{}", input)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    writeln!(io::stdout(), "{}", input)?;
    Ok(())
}