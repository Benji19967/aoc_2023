use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

// TODO: How to log at different levels?

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let symbols = get_symbols(&input).unwrap();
    let matrix = create_matrix(&input).unwrap();

    part1(&matrix, &symbols)?;
    part2(&matrix, &symbols)?;

    Ok(())
}

fn get_symbols(input: &String) -> Result<HashSet<char>> {
    let mut symbols = HashSet::new();
    for line in input.lines() {
        for c in line.chars() {
            if c.is_numeric() || c == '.' {
                continue;
            } else {
                symbols.insert(c);
            }
        }
    }
    Ok(symbols)
}

fn create_matrix(input: &String) -> Result<Vec<Vec<char>>> {
    let num_rows: usize = input.lines().count();
    let num_columns: usize = input.lines().next().unwrap().len();
    let mut matrix = vec![vec![char::default(); num_columns]; num_rows];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            matrix[row][col] = c;
        }
    }
    Ok(matrix)
}

fn has_neighboring_symbol(
    matrix: &Vec<Vec<char>>,
    symbols: &HashSet<char>,
    i: usize,
    j: usize,
) -> bool {
    for (di, dj) in vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] as Vec<(i32, i32)>
    {
        let row = i as i32 + di;
        let col = j as i32 + dj;
        if (0..matrix.len()).contains(&(row as usize))
            && (0..matrix[0].len()).contains(&(col as usize))
            && symbols.contains(&matrix[row as usize][col as usize])
        {
            return true;
        }
    }
    false
}

fn part1(matrix: &Vec<Vec<char>>, symbols: &HashSet<char>) -> Result<()> {
    let mut sum = 0;

    for i in 0..matrix.len() {
        let mut number: Vec<char> = Vec::new();

        let mut j: usize = 0;
        while j < matrix[0].len() {
            let mut c: char = matrix[i][j];
            let mut found_neighboring_symbol: bool = false;

            while c.is_numeric() && j < matrix[0].len() {
                found_neighboring_symbol =
                    found_neighboring_symbol | has_neighboring_symbol(&matrix, symbols, i, j);
                number.push(c);
                j += 1;
                if j == matrix[0].len() {
                    break;
                }
                c = matrix[i][j];
            }
            if number.len() > 0 && found_neighboring_symbol {
                sum += number
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
            }
            number = Vec::new();
            j += 1;
        }
    }
    writeln!(io::stdout(), "sum: {:?}", sum)?;
    Ok(())
}

fn part2(matrix: &Vec<Vec<char>>, symbols: &HashSet<char>) -> Result<()> {
    // for line in input.lines() {
    //     ();
    // }
    Ok(())
}
