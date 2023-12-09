use anyhow::{bail, ensure, Context};
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
            bail!("no lines to process");
        }
        let line_lengths = HashSet::<usize>::from_iter(lines.iter().map(|l| l.len()));
        ensure!(line_lengths.len() == 1, "all lines must be the same length");
        let line_length = line_lengths.into_iter().next().unwrap();
        ensure!(line_length > 0, "lines must be non-empty");

        let mut part_numbers = Vec::new();
        if lines.len() > 1 {
            let mut first_line_part_nums = get_part_numbers_special(lines[0], 1, lines[1])?;
            part_numbers.append(&mut first_line_part_nums);
            if lines.len() > 2 {
                for line_idx in 1..lines.len() - 1 {
                    let line = lines[line_idx];
                    let ctx_lines = [lines[line_idx - 1], lines[line_idx + 1]];
                    let mut line_part_numbers = get_part_numbers(line, line_idx + 1, ctx_lines)?;
                    part_numbers.append(&mut line_part_numbers);
                }
            }
            let mut last_line_part_nums = get_part_numbers_special(
                lines[lines.len() - 1],
                lines.len(),
                lines[lines.len() - 2],
            )?;
            part_numbers.append(&mut last_line_part_nums);
        }
        let answer: u32 = part_numbers.into_iter().sum();
        writeln!(io::stdout(), "{}", answer)?;
        Ok(())
    }

    fn get_part_numbers_special(
        line: &str,
        line_num: usize,
        ctx_line: &str,
    ) -> anyhow::Result<Vec<u32>> {
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
                let num = parse_num(&line[beg..end], line_num, beg)?;
                if is_part_num_same_line(line, beg, end) || is_part_num_ctx_line(ctx_line, beg, end)
                {
                    part_numbers.push(num);
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

    fn get_part_numbers(
        line: &str,
        line_num: usize,
        ctx_lines: [&str; 2],
    ) -> anyhow::Result<Vec<u32>> {
        let mut part_numbers = Vec::new();
        let line = line.as_bytes();
        let ctx_lines = [ctx_lines[0].as_bytes(), ctx_lines[1].as_bytes()];

        let mut beg = 0;
        let mut end = 1;
        loop {
            if line[beg].is_ascii_digit() {
                while end < line.len() && line[end].is_ascii_digit() {
                    end += 1;
                }

                // line[beg..end] is a number.
                let num = parse_num(&line[beg..end], line_num, beg)?;
                if is_part_num_same_line(line, beg, end)
                    || is_part_num_ctx_line(ctx_lines[0], beg, end)
                    || is_part_num_ctx_line(ctx_lines[1], beg, end)
                {
                    part_numbers.push(num);
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

    fn parse_num(num: &[u8], line_num: usize, index: usize) -> anyhow::Result<u32> {
        let num = std::str::from_utf8(num).unwrap();
        num.parse::<u32>().with_context(|| {
            format!(
                "line number = {}, index = {}: failed to parse number",
                line_num, index
            )
        })
    }
}

mod part2 {
    use super::*;

    pub fn part2(input: &str) -> anyhow::Result<()> {
        //writeln!(io::stdout(), "{}", input)?;
        Ok(())
    }
}
