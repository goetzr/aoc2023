use anyhow::{bail, ensure, Context};
use std::collections::HashSet;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let part_numbers = get_all_part_numbers(input)?;
    let answer: u32 = part_numbers.into_iter().fold(0, |acc, pn| acc + pn.value);
    writeln!(io::stdout(), "{}", answer)?;
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let part_numbers = get_all_part_numbers(input)?;
    let lines = input.lines().collect::<Vec<_>>();
    // 123*456...
    //
    // 123.......
    // ...*456...
    //
    // 123.......
    // ...*......
    // ....456...
    // 
    // 123*456...   // NOT a gear
    // ..789.....
    
    let mut gears = Vec::new();
    if lines.len() == 1 {
        let all_gears = get_gears_special(lines[0])
    }
    for line_idx in 0..lines.len() {
        let line = lines[line_idx];

        // TODO: Ensure the star is not next to 3 or more part numbers! 
    }
    Ok(())
}

mod part2 {
    use super::*;
    
    struct Gear<'a> {
        part_numbers: [&'a PartNumber; 2],
    }
    
    impl<'a> Gear<'a> {
        fn new(pn1: &'a PartNumber, pn2: &'a PartNumber) -> Self {
            Gear { part_numbers: [pn1, pn2] }
        }
    }
    
    fn get_gears_one_line_file(line: &str, part_numbers: &'p Vec<PartNumber>) -> Vec<Gear<'p>> {
        
    }
    fn get_gears_special<'p>(line: &str, part_numbers: [&'p Vec<PartNumber>; 2]) -> Vec<Gear<'p>> {
        let mut gears = Vec::new();
        for star_idx in line.chars().filter(|&c| c == '*') {
            
        }
        gears
    }
    
    fn get_adjacent_part_numbers(star_idx: usize, part_numb)
}
struct PartNumber {
    value: u32,
    range: std::ops::Range<usize>,
}

impl PartNumber {
    fn new(value: u32, beg: usize, end: usize) -> Self {
        let range = std::ops::Range { start: beg, end };
        PartNumber { value, range }
    }
}

fn get_part_numbers_special(
    line: &str,
    line_num: usize,
    ctx_line: &str,
) -> anyhow::Result<Vec<PartNumber>> {
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
                let part_number = PartNumber::new(num, beg, end);
                part_numbers.push(part_number);
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
) -> anyhow::Result<Vec<PartNumber>> {
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
                let part_number = PartNumber::new(num, beg, end);
                part_numbers.push(part_number);
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

fn get_all_part_numbers(input: &str) -> anyhow::Result<Vec<Vec<PartNumber>>> {
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
        let first_line_part_nums = get_part_numbers_special(lines[0], 1, lines[1])?;
        part_numbers.push(first_line_part_nums);
        if lines.len() > 2 {
            for line_idx in 1..lines.len() - 1 {
                let line = lines[line_idx];
                let ctx_lines = [lines[line_idx - 1], lines[line_idx + 1]];
                let mut line_part_numbers = get_part_numbers(line, line_idx + 1, ctx_lines)?;
                part_numbers.push(line_part_numbers);
            }
        }
        let last_line_part_nums = get_part_numbers_special(
            lines[lines.len() - 1],
            lines.len(),
            lines[lines.len() - 2],
        )?;
        part_numbers.push(last_line_part_nums);
    }
    
    Ok(part_numbers)
}

