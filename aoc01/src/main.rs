use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for c in line.chars() {
            if c.is_numeric() {
                first = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                last = c.to_digit(10).unwrap();
                break;
            }
        }
        sum += first * 10 + last;
    }
    writeln!(io::stdout(), "Sum: {}", sum)?;
    Ok(())
}

// fn part2(input: &String) -> Result<()> {
//     let mut sum: u32 = 0;
//     for line in input.lines() {}
//     writeln!(io::stdout(), "Sum: {}", sum)?;
//     Ok(())
// }
