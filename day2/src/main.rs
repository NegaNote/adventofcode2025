use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Parser)]
#[command(about, long_about = None)]
struct FileArgs {
    #[arg(short, long)]
    file: String,
}

#[derive(Debug, Copy, Clone)]
struct IdRange {
    start: u64,
    end: u64,
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let split = s.split('-').take(2)
            .map(|s2| s2.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>().context(format!("Could not parse input: {}", s))?;
        if split.len() != 2 {
            anyhow::bail!("Wrong format");
        }
        Ok(IdRange { start: split[0], end: split[1] })
    }
}

impl IntoIterator for IdRange {
    type Item = u64;
    type IntoIter = RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

fn main() -> Result<()> {
    let args = FileArgs::parse();
    let mut f = File::open(&args.file)?;
    let mut contents = String::with_capacity(f.metadata()?.len() as usize);
    f.read_to_string(&mut contents)?;
    contents = contents.replace('\n', "");

    let id_strings = contents.split(',');

    let id_ranges = id_strings.into_iter().map(IdRange::from_str).collect::<Result<Vec<_>, _>>()?;

    let mut part1_running_sum = 0;
    let mut part2_running_sum = 0;

    for range in id_ranges {
        for id in range.into_iter() {
            let string = id.to_string();
            if string.len() % 2 == 0 {
                let split_strings = string.split_at(string.len() / 2);
                if split_strings.0 == split_strings.1 {
                    part1_running_sum += id;
                }
            }
            let max_len_to_check = string.len() / 2;
            for len in 1..=max_len_to_check {
                if let Some(reduced) = string.chars().collect::<Vec<_>>().chunks(len)
                    .map(|chunk| chunk.into_iter()
                        .fold("".to_string(), |acc, x| acc + &x.to_string()))
                    .reduce(|acc, x| {
                    if acc == x {
                        acc
                    } else {
                        "".to_string()
                    }
                }) {
                    if reduced != "".to_string() {
                        part2_running_sum += id;
                        break;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", part1_running_sum);
    println!("Part 2: {}", part2_running_sum);

    Ok(())
}
