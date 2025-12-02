use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None)]
struct FilenameArgs {
    #[arg(short, long)]
    file: String,
}

#[derive(Debug, Clone)]
enum RotationDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Rotation {
    direction: RotationDirection,
    amount: i32,
}

impl Rotation {
    fn new(direction: RotationDirection, amount: i32) -> Self {
        Self { direction, amount }
    }
}

impl TryFrom<char> for RotationDirection {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(RotationDirection::Left),
            'R' => Ok(RotationDirection::Right),
            _ => anyhow::bail!("Invalid rotation value"),
        }
    }
}

fn main() -> Result<()> {
    let filename_args = FilenameArgs::parse();

    let mut file = File::open(filename_args.file)?;
    let mut file_string = "".to_string();
    file.read_to_string(&mut file_string)?;

    let mut dial_pos: i32 = 50;
    let mut times_zero_reached_part1 = 0;
    let mut times_zero_reached_part2 = 0;

    for line in file_string.lines() {
        let rotation_dir: RotationDirection = line.chars().nth(0)
            .ok_or_else(|| anyhow::anyhow!("Line is empty"))?.try_into()?;

        let rest_of_line = &line[1..line.len()];
        let num = rest_of_line.parse::<i32>()?;
        let rotation = Rotation::new(rotation_dir, num);
        match rotation.direction {
            RotationDirection::Left => {
                let total = dial_pos - rotation.amount;
                if total <= 0 {
                    times_zero_reached_part2 += (total / 100).abs() + 1;
                    if dial_pos == 0 {
                        times_zero_reached_part2 -= 1;
                    }
                }
                dial_pos = (dial_pos - rotation.amount).rem_euclid(100);
            },
            RotationDirection::Right => {
                let total = dial_pos + rotation.amount;
                if total >= 100 {
                    times_zero_reached_part2 += total / 100;
                }
                dial_pos = (dial_pos + rotation.amount).rem_euclid(100);
            }
        }
        if dial_pos == 0 {
            times_zero_reached_part1 += 1;
        }
    }

    println!("Part 1 answer: {}", times_zero_reached_part1);
    println!("Part 2 answer: {}", times_zero_reached_part2);

    Ok(())
}
