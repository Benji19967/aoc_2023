use phf::phf_map;
use std::io::{self, Read, Write};

static NUMBERS: phf::Map<&'static str, u32> = phf_map! {
        "two" => 2,
        "one" => 1,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

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

fn part2(input: &String) -> Result<u32> {
    // TODO: Move this out of the function. Make it static?

    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut found_numbers: Vec<(usize, u32)> = Vec::new();
        for number in NUMBERS.keys() {
            let mut found: Vec<(usize, u32)> = line
                .match_indices(number)
                .map(|(a, b)| (a, *NUMBERS.get(b).unwrap()))
                .collect();
            found_numbers.append(&mut found);
        }
        for (idx, c) in line.chars().enumerate() {
            if c.is_numeric() {
                found_numbers.push((idx, c.to_digit(10).unwrap()))
            }
        }
        found_numbers.sort();
        sum += found_numbers[0].1 * 10 + found_numbers.last().unwrap().1;
    }
    writeln!(io::stdout(), "Sum: {}", sum)?;
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_basic() {
        assert_eq!(part2(&String::from("1two1")).unwrap(), 11);
    }
    #[test]
    fn part2_characters() {
        assert_eq!(part2(&String::from("two1three")).unwrap(), 23);
    }
    #[test]
    fn part2_mixed() {
        assert_eq!(part2(&String::from("two1")).unwrap(), 21);
    }
    #[test]
    fn part2_mixed_reversed() {
        assert_eq!(part2(&String::from("1two")).unwrap(), 12);
    }
    #[test]
    fn part2_repeat_characters() {
        assert_eq!(part2(&String::from("two1two")).unwrap(), 22);
    }
}
