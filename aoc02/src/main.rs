use nom::multi::separated_list1;
use nom::sequence::separated_pair;
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

impl Cubes {
    fn is_valid(&self) -> Result<bool> {
        match self.color {
            Color::RED => Ok(self.amount <= 12),
            Color::BLUE => Ok(self.amount <= 14),
            Color::GREEN => Ok(self.amount <= 13),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Vec<Cubes>>,
}

impl Game {
    fn is_all_cubes_valid(&self) -> bool {
        self.rounds
            .iter()
            .all(|round| round.iter().all(|cube| cube.is_valid().unwrap()))
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // let games = parse_lines(&input).unwrap();
    let (_, games) = parse_lines_nom(&input).unwrap();

    part1(&games)?;
    part2(&games)?;

    Ok(())
}

fn parse_lines_nom(input: &str) -> IResult<&str, Vec<Game>> {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let (_, (id, rounds)) = parse_line_nom(line)?;
        games.push(Game { id, rounds });
    }
    Ok((input, games))
}

fn parse_line_nom(input: &str) -> IResult<&str, (u32, Vec<Vec<Cubes>>)> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, sets) = sets_of_cubes(input)?;
    Ok((input, (game_id, sets)))
}

fn sets_of_cubes(input: &str) -> IResult<&str, Vec<Vec<Cubes>>> {
    let (input, sets) = separated_list1(tag(";"), set_of_cubes)(input)?;
    Ok((input, sets))
}

fn set_of_cubes(input: &str) -> IResult<&str, Vec<Cubes>> {
    let (input, set) = separated_list1(tag(","), cube_num_and_color)(input)?;
    Ok((input, set))
}

fn cube_num_and_color(input: &str) -> IResult<&str, Cubes> {
    let (input, _) = tag(" ")(input)?;
    let (input, (amount, color)) = separated_pair(
        nom::character::complete::u32,
        nom::character::complete::char(' '),
        nom::character::complete::alpha1,
    )(input)?;
    let color = Color::from_str(color).unwrap();
    Ok((input, Cubes { color, amount }))
}

fn parse_lines(input: &String) -> Result<Vec<Game>> {
    let mut games: Vec<Game> = Vec::new();

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
        games.push(Game {
            id: game_id,
            rounds: sets_of_cubes,
        });
    }
    Ok(games)
}

fn part1(games: &Vec<Game>) -> Result<()> {
    let sum = games
        .iter()
        .filter(|game| game.is_all_cubes_valid())
        .map(|game| game.id)
        .sum::<u32>();

    writeln!(io::stdout(), "sum: {}", sum)?;
    Ok(())
}

fn part2(games: &Vec<Game>) -> Result<()> {
    let mut sum = 0;

    for game in games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in &game.rounds {
            for cube in round {
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
