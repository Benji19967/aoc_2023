use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut sum = 0;

    for line in input.lines() {
        let mut possible = true;
        let split = line.split(":").collect::<Vec<&str>>();
        let game = split[0];
        let records = split[1];
        let sets_of_games = records.split(";");
        for set in sets_of_games {
            let cubes_in_set = set.split(",");
            for cube_with_amount in cubes_in_set {
                let amount: u32 = cube_with_amount
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let color = cube_with_amount.split_whitespace().next_back().unwrap();
                let valid = match color {
                    "red" => amount <= 12,
                    "green" => amount <= 13,
                    "blue" => amount <= 14,
                    _ => panic!("Unexpected color"),
                };
                if valid == false {
                    possible = false;
                }
                // writeln!(io::stdout(), "amount: {}", amount)?;
                // writeln!(io::stdout(), "color: {}", color)?;
            }
        }

        let game_id: u32 = game
            .split_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        // writeln!(io::stdout(), "game_id: {}", game_id)?;
        if possible {
            sum += game_id;
        }
    }
    writeln!(io::stdout(), "sum: {}", sum)?;
    Ok(())
}
fn part2(input: &String) -> Result<()> {
    Ok(())
}
