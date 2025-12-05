use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;
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
struct InclusiveRange {
    start: usize,
    end: usize,
}

impl InclusiveRange {
    fn overlaps(&self, other: &InclusiveRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

fn condense_overlap(range_a: &InclusiveRange, range_b: &InclusiveRange) -> InclusiveRange {
    InclusiveRange {
        start: min(range_a.start, range_b.start),
        end: max(range_a.end, range_b.end),
    }
}

impl Display for InclusiveRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl TryFrom<&str> for InclusiveRange {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dash_position = value.find('-').ok_or_else(|| anyhow::anyhow!("No dash found in range"))?;

        anyhow::ensure!(dash_position < value.len() - 1 && dash_position > 0, "Dash in invalid position");

        let (start_str, end_str) = value.split_at(dash_position);

        let end_str = &end_str[1..];

        let start = usize::from_str(start_str)?;
        let end = usize::from_str(end_str)?;

        Ok(InclusiveRange {
            start,
            end,
        })
    }
}

fn main() -> Result<()> {
    let filename_args = FilenameArgs::parse();

    let mut file = File::open(filename_args.file)?;
    let mut file_string = "".to_string();
    file.read_to_string(&mut file_string)?;

    file_string = file_string.trim().to_string();

    let mut lines = file_string.lines();

    let mut ranges: Vec<InclusiveRange> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let range = InclusiveRange::try_from(line)?;
        ranges.push(range);
    }

    let mut ids_to_check: Vec<usize> = Vec::new();

    for line in lines {
        ids_to_check.push(line.parse::<usize>()?);
    }

    let mut checked_ids: HashSet<usize> = HashSet::new();

    let mut part1 = 0;

    for id in &ids_to_check {
        for range in &ranges {
            if &range.start <= id && id <= &range.end && !checked_ids.contains(id) {
                part1 += 1;
                checked_ids.insert(*id);
            }
        }
    }

    println!("Part 1: {}", part1);

    drop(ids_to_check);
    drop(checked_ids);

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut condensed_ranges: Vec<InclusiveRange> = Vec::new();

    let mut condensed_range = ranges[0];

    for range in &ranges[1..] {
        if range.overlaps(&condensed_range) {
            condensed_range = condense_overlap(&range, &condensed_range);
        } else {
            condensed_ranges.push(condensed_range);
            condensed_range = *range;
        }
    }

    condensed_ranges.push(condensed_range);

    let part2 = condensed_ranges.iter().fold(0, |acc, x| acc + (x.end - x.start) + 1);

    println!("Part 2: {}", part2);

    Ok(())
}
