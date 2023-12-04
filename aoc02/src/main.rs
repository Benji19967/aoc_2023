use nom::multi::separated_list1;
use nom::{bytes::complete::tag, IResult};
use std::str::FromStr;
use std::{
    cmp,
    collections::HashMap,
    io::{self, Read, Write},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

impl FromStr for Color {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Color> {
        match input {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            _ => panic!("Unexpected color"),
        }
    }
}

#[derive(Debug)]
struct Cubes {
    color: Color,
    amount: u32,
}

type Games = HashMap<u32, Vec<Vec<Cubes>>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // let games = parse_lines(&input).unwrap();
    let (_, games) = parse_lines_nom(&input).unwrap();

    part1(&games)?;
    part2(&games)?;

    Ok(())
}

fn cube_num_and_color(input: &str) -> IResult<&str, Cubes> {
    let (input, _) = tag(" ")(input)?;
    let (input, amount) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = nom::character::complete::alpha1(input)?;
    let color = Color::from_str(color).unwrap();

    Ok((input, Cubes { color, amount }))
}

fn set_of_cubes(input: &str) -> IResult<&str, Vec<Cubes>> {
    let (input, set) = separated_list1(tag(","), cube_num_and_color)(input)?;
    Ok((input, set))
}

fn sets_of_cubes(input: &str) -> IResult<&str, Vec<Vec<Cubes>>> {
    let (input, sets) = separated_list1(tag(";"), set_of_cubes)(input)?;
    Ok((input, sets))
}

fn parse_line_nom(input: &str) -> IResult<&str, (u32, Vec<Vec<Cubes>>)> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, sets) = sets_of_cubes(input)?;
    Ok((input, (game_id, sets)))
}

fn parse_lines_nom(input: &str) -> IResult<&str, Games> {
    let mut games: Games = HashMap::new();
    for line in input.lines() {
        let (_, (game_id, sets)) = parse_line_nom(line)?;
        games.insert(game_id, sets);
    }
    Ok((input, games))
}

fn parse_lines(input: &String) -> Result<Games> {
    let mut games: Games = HashMap::new();

    for line in input.lines() {
        let mut sets_of_cubes: Vec<Vec<Cubes>> = Vec::new();

        let split = line.split(":").collect::<Vec<&str>>();
        let game = split[0];
        let records = split[1];
        let sets_of_games = records.split(";");
        for set in sets_of_games {
            let mut set_of_cubes: Vec<Cubes> = Vec::new();
            let cubes_in_set = set.split(",");
            for cube_with_amount in cubes_in_set {
                let amount: u32 = cube_with_amount
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let color = cube_with_amount.split_whitespace().next_back().unwrap();
                let color_enum = match color {
                    "red" => Color::RED,
                    "green" => Color::GREEN,
                    "blue" => Color::BLUE,
                    _ => panic!("Unextpected color"),
                };
                let cubes = Cubes {
                    color: color_enum,
                    amount,
                };
                set_of_cubes.push(cubes);
            }
            sets_of_cubes.push(set_of_cubes);
        }
        let game_id: u32 = game
            .split_whitespace()
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        games.insert(game_id, sets_of_cubes);
    }
    Ok(games)
}

fn part1(games: &Games) -> Result<()> {
    let mut sum = 0;

    for (game_id, sets_of_cubes) in games {
        let mut possible = true;
        for set_of_cubes in sets_of_cubes {
            for cube in set_of_cubes {
                let valid = match cube.color {
                    Color::RED => cube.amount <= 12,
                    Color::GREEN => cube.amount <= 13,
                    Color::BLUE => cube.amount <= 14,
                };
                if valid == false {
                    possible = false;
                }
            }
        }
        if possible {
            sum += game_id;
        }
    }
    writeln!(io::stdout(), "sum: {}", sum)?;
    Ok(())
}

fn part2(games: &Games) -> Result<()> {
    let mut sum = 0;

    for (_, sets_of_cubes) in games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set_of_cubes in sets_of_cubes {
            for cube in set_of_cubes {
                match cube.color {
                    Color::RED => min_red = cmp::max(min_red, cube.amount),
                    Color::GREEN => min_green = cmp::max(min_green, cube.amount),
                    Color::BLUE => min_blue = cmp::max(min_blue, cube.amount),
                };
            }
        }
        sum += min_red * min_green * min_blue;
    }
    writeln!(io::stdout(), "sum: {}", sum)?;
    Ok(())
}
