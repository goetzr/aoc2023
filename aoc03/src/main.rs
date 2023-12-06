use std::io::{self, Write};
use anyhow::{Context};


fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1::part1(&input)?;
    part2::part2(&input)?;
    Ok(())
}

struct Window<'a, I: Iterator<Item = &'a str>> {
    iter: I,
    lines: [&'a u8; 3],
}

impl<'a, I: Iterator<Item = &'a str>> Window<'a, I> {
    fn new(iter: I) -> Self {
        // TODO: Grab lines. What if there are less than 2 lines?
        let line1 = iter.next();
        Window { iter, }
    }
    fn slide_down(&mut self) {

    }
}
mod part1 {
    use super::*;

    pub fn part1(input: &str) -> anyhow::Result<()> {
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }

}

mod part2 {
    use super::*;

    pub fn part2(input: &str) -> anyhow::Result<()> {
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }

}