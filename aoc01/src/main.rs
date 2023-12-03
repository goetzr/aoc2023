use std::io::{self, Write};

type Error = dyn std::error::Error + 'static;
type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    //part1(&input)?;
    part2::part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut code = 0;
    for line in input.lines() {
        let numbers = line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
        assert!(numbers.len() > 0);
        let first = *numbers.first().unwrap();
        let last = *numbers.last().unwrap();
        let val = first * 10 + last;
        code += val;
    }
    writeln!(io::stdout(), "{}", code)?;
    Ok(())
}

mod part2 {
    use super::*;

    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    pub fn part2(input: &str) -> Result<()> {
        let mut code = 0;
        for line in input.lines() {
            let numbers = get_numbers(line);
            assert!(numbers.len() > 0);
            let first = *numbers.first().unwrap();
            let last = *numbers.last().unwrap();
            println!("{}: {}, {}", line, first, last);
            let val = first * 10 + last;
            code += val; 
        }
        writeln!(io::stdout(), "{}", code)?;
        Ok(())
    }

    fn get_numbers(line: &str) -> Vec<u32> {
        let mut numbers = Vec::new();
        if line.is_empty() {
            return numbers;
        }

        let mut substr = line;
        loop {
            let c = substr.chars().next().unwrap();
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                numbers.push(digit);
                substr = &substr[1..];
            } else {
                let mut found = false;
                for (idx, num_str) in crate::part2::NUMBERS.iter().enumerate() {
                    if substr.starts_with(num_str) {
                        found = true;
                        let num = (idx + 1) as u32;
                        numbers.push(num);
                        substr = &substr[num_str.len()..];
                        break;
                    }
                }
                if !found {
                    substr = &substr[1..];
                }
            }
            if substr.len() == 0 {
                break;
            }
        }

        numbers
    }

}