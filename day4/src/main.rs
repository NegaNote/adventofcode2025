use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(about, long_about = None)]
struct FileArgs {
    #[arg(short, long)]
    file: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Roll {
    Present,
    NotPresent,
}

impl Roll {
    fn as_usize(&self) -> usize {
        match self {
            Roll::Present => 1,
            Roll::NotPresent => 0,
        }
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Roll::Present => '@',
            Roll::NotPresent => '.',
        })
    }
}

impl TryFrom<char> for Roll {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '@' => Ok(Roll::Present),
            '.' => Ok(Roll::NotPresent),
            _ => Err(anyhow::anyhow!("Invalid Roll: {}", value)),
        }
    }
}



fn num_neighbors_present(grid: &Vec<Vec<Roll>>, i: usize, j: usize) -> usize {
    let mut neighbors = 0;
    if i > 0 {
        if j > 0 {
            neighbors += grid[i - 1][j - 1].as_usize();
        }
        neighbors += grid[i - 1][j].as_usize();
        if j < grid[i].len() - 1 {
            neighbors += grid[i - 1][j + 1].as_usize();
        }
    }

    if j > 0 {
        neighbors += grid[i][j - 1].as_usize();
    }
    if j < grid[0].len() - 1 {
        neighbors += grid[i][j + 1].as_usize();
    }

    if i < grid.len() - 1 {
        if j > 0 {
            neighbors += grid[i + 1][j - 1].as_usize();
        }
        neighbors += grid[i + 1][j].as_usize();
        if j < grid[i].len() - 1 {
            neighbors += grid[i + 1][j + 1].as_usize();
        }
    }
    neighbors
}

/*
fn show_grid(grid: &Vec<Vec<Roll>>) {
    for row in grid.iter() {
        for value in row.iter() {
            print!("{}", value);
        }
        println!();
    }
}
*/

fn main() -> Result<()> {
    let args = FileArgs::parse();
    let mut f = File::open(&args.file)?;
    let mut contents = String::with_capacity(f.metadata()?.len() as usize);
    f.read_to_string(&mut contents)?;

    let mut grid: Vec<Vec<Roll>> = contents.trim().lines()
        .map(|line| {
            line.chars().map(|c| Roll::try_from(c)).collect::<Result<Vec<Roll>>>()
        }).collect::<Result<Vec<Vec<Roll>>>>()?;

    let mut available_rolls = 0;

    for (i, row) in grid.iter().enumerate() {
        for j in 0..row.len() {
            if num_neighbors_present(&grid, i, j) < 4 && grid[i][j] == Roll::Present {
                available_rolls += 1;
            }
        }
    }

    println!("Part 1: {}", available_rolls);

    let mut num_rolls_removed = 0;

    let mut num_rolls_removed_per_pass = 1;

    while num_rolls_removed_per_pass != 0 {
        num_rolls_removed_per_pass = 0;
        let mut new_grid: Vec<Vec<Roll>> = grid.clone();
        for (i, row) in grid.iter().enumerate() {
            for j in 0..row.len() {
                if num_neighbors_present(&grid, i, j) < 4 && grid[i][j] == Roll::Present {
                    new_grid[i][j] = Roll::NotPresent;
                    num_rolls_removed_per_pass += 1;
                    num_rolls_removed += 1;
                }
            }
        }

        grid = new_grid;
    }

    println!("Part 2: {}", num_rolls_removed);

    Ok(())
}
