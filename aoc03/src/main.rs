use std::io::{self, Write};
use std::collections::HashSet;
use anyhow::{Context, ensure};


fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1::part1(&input)?;
    part2::part2(&input)?;
    Ok(())
}

struct Window<'a> {
    lines: [Option<&'a str>; 3],
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
        let lines = input.lines().collect::<Vec<&str>>();
        if lines.len() == 0 {
            writeln!(io::stdout(), "{}", 0);
            return Ok(());
        }
        ensure!(lines.iter().map(|l| l.len()).collect::<HashSet<_>>().len() == 1, "all lines must be the same length");

        let mut answer = 0;
        if lines.count()
        let mut window: [Option<&str>; 3] = [None; 3];
        let line1 = lines.next();
        if !line1.is_none() {
            let line2 = lines.next();
            if !line2.is_none() {

            }
        }
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }
    
    // TODO: Change this to take two lines. When checking single line, pass same line to both params.
    fn get_part_numbers(line: &str, line_num: usize) -> anyhow::Result<Vec<u32>> {
        let mut part_numbers = Vec::new();
        if line.is_empty() {
            return Ok(part_numbers);
        }

        let msg = |msg: &str| {
            format!("line {}: {}", line_num, msg)
        };

        let line = line.as_bytes();
        let mut beg = 0;
        let mut end = 1;
        loop {
            if line[beg].is_ascii_digit() {
                while end < line.len() && line[end].is_ascii_digit() {
                    end += 1;
                }

                // line[beg..end] is a number.
                let is_part_num = beg > 0 && line[beg] != '.' || end < line.len() && line[end] != '.';
                if is_part_num {
                    let part_num = &line[beg..end];
                    let part_num = std::str::from_utf8(part_num).unwrap();
                    let part_num = part_num.parse::<u32>().context(msg("failed to parse part number"))?;
                    part_numbers.push(part_num);
                }
            }

            if end == line.len() {
                break;
            }
            beg = end;
            end += 1;
        }
        
        Ok(part_numbers)
    }
}

mod part2 {
    use super::*;

    pub fn part2(input: &str) -> anyhow::Result<()> {
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }

}