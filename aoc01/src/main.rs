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

fn to_number(data: &str) -> Option<(u32, &str)> {
    if let Some(digit) = data.chars().next().expect("empty data").to_digit(10) {
        // What if this takes us to the end?
        Some(digit, &data[1..]
    }
}

fn to_number<I: Iterator<Item=char> + Clone>(chars: Box<I>) -> Option<(u32, Box<I>)> {
    Some((7, chars.clone()))
    //unimplemented!();
}

fn get_spelled_out_num(chars: &impl Iterator<Item=char>) -> Option<u32> {
    if chars.clone().take()
}

fn spelled_out(num: i32) -> &'static str {
    match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
    }
}