use std::io::{self, Write};
use std::collections::HashSet;
use anyhow::{Context, ensure};


fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1::part1(&input)?;
    part2::part2(&input)?;
    Ok(())
}

mod part1 {
    use super::*;

    pub fn part1(input: &str) -> anyhow::Result<()> {
        let lines = input.lines().collect::<Vec<&str>>();
        if lines.len() == 0 {
            writeln!(io::stdout(), "{}", 0);
            return Ok(());
        }
        let line_lengths = HashSet::from_iter(lines.iter().map(|l| l.len()));
        ensure!(line_lengths.len() == 1, "all lines must be the same length");
        ensure!(line_lengths.iter().next().unwrap() > 0, "all lines must be non-empty");

        let mut answer = 0;

        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }
    
    fn get_part_numbers(line: &str, line_num: usize, ctx_lines: &[Option<&str>; 2]) -> anyhow::Result<Vec<u32>> {
        let msg = |msg: &str| {
            format!("line {}: {}", line_num, msg)
        };

        let mut part_numbers = Vec::new();
        let line = line.as_bytes();
        let ctx_lines = [ctx_lines[0].map(|l| l.as_bytes()), ctx_lines[1].map(|l| l.as_bytes())];

        let mut beg = 0;
        let mut end = 1;
        loop {
            if line[beg].is_ascii_digit() {
                while end < line.len() && line[end].is_ascii_digit() {
                    end += 1;
                }

                // line[beg..end] is a number.
                let is_part_num = beg > 0 || && line[beg] != b'.' || end < line.len() && line[end] != b'.';
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