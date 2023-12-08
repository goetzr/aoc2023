use anyhow::{ensure, Context};
use std::collections::HashSet;
use std::io::{self, Write};

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
        ensure!(
            line_lengths.iter().next().unwrap() > 0,
            "all lines must be non-empty"
        );

        let mut answer = 0;

        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }

    fn get_part_numbers_special(
        line: &str,
        line_num: usize,
        ctx_line: &str,
    ) -> anyhow::Result<Vec<u32>> {
        let msg = |msg: &str| format!("line {}: {}", line_num, msg);

        let mut part_numbers = Vec::new();
        let line = line.as_bytes();
        let ctx_line = ctx_line.as_bytes();

        let mut beg = 0;
        let mut end = 1;
        loop {
            if line[beg].is_ascii_digit() {
                while end < line.len() && line[end].is_ascii_digit() {
                    end += 1;
                }

                // line[beg..end] is a number.
                if is_part_num_same_line(line, beg, end) {
                    let part_num = parse_part_num(&line[beg..end], line_num, beg)?;
                    part_numbers.push(part_num);
                } else {
                    if is_part_num_ctx_line(ctx_line, beg, end) {
                        let part_num = parse_part_num(&line[beg..end], line_num, beg)?;
                        part_numbers.push(part_num);
                    }
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

    fn is_part_num_same_line(line: &[u8], beg: usize, end: usize) -> bool {
        beg > 0 && line[beg - 1] != b'.' || end < line.len() && line[end] != b'.'
    }

    fn is_part_num_ctx_line(ctx_line: &[u8], beg: usize, end: usize) -> bool {
        (beg > 0 && ctx_line[beg - 1] != b'.' && !ctx_line[beg - 1].is_ascii_digit())
            || (end < ctx_line.len() && ctx_line[end] != b'.' && !ctx_line[end].is_ascii_digit())
            || ctx_line[beg..end]
                .iter()
                .any(|&b| b != b'.' && !b.is_ascii_digit())
    }

    fn parse_part_num(part_num: &[u8], line_num: usize, index: usize) -> anyhow::Result<u32> {
        let part_num = std::str::from_utf8(part_num).unwrap();
        part_num.parse::<u32>().context(format!(
            "line number = {}, index = {}: failed to parse part number",
            line_num, index
        ))
    }
}

mod part2 {
    use super::*;

    pub fn part2(input: &str) -> anyhow::Result<()> {
        writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }
}
