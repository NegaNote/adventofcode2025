use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None)]
struct FilenameArgs {
    #[arg(short, long)]
    file: String,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => anyhow::bail!("unknown operation: {}", s),
        }
    }
}

fn main() -> Result<()> {
    let filename_args = FilenameArgs::parse();

    let mut file = File::open(filename_args.file)?;
    let mut file_string = "".to_string();
    file.read_to_string(&mut file_string)?;

    file_string = file_string.trim().to_string();

    let lines = file_string.lines().collect::<Vec<&str>>();

    let mut grid: Vec<Vec<usize>> = Vec::new();

    for i in 0..lines.len() - 1 {
        let line = lines[i];
        grid.push(line.split_whitespace().map(|x| x.parse::<usize>()).collect::<Result<Vec<usize>, _>>()?);
    }

    let operations: Vec<Operation> = lines[lines.len() - 1].split_whitespace()
        .map(|x| x.parse::<Operation>()).collect::<Result<Vec<Operation>, _>>()?;

    let mut part1 = 0;

    for i in 0..grid[0].len() {
        let mut running_total = 0;
        match operations[i] {
            Operation::Add => {
                for row in &grid {
                    running_total += row[i];
                }
            },
            Operation::Multiply => {
                running_total = 1;
                for row in &grid {
                    running_total *= row[i];
                }
            }
        }
        part1 += running_total;
    }

    println!("Part 1: {}", part1);

    Ok(())
}
