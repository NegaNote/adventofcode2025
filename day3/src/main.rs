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

#[derive(Debug, Copy, Clone)]
struct IndexAndDigit {
    index: usize,
    digit: usize,
}

impl IndexAndDigit {
    fn new(index: usize, digit: usize) -> Self {
        Self { index, digit }
    }
}

fn max_digit(bank: &str, search_start: usize, search_end: usize) -> IndexAndDigit {
    let bytes = bank.as_bytes();

    let mut max_index_and_digit = IndexAndDigit::new(search_start, (bytes[search_start] - ('0' as u8)) as usize);

    for i in search_start..search_end {
        let digit = (bytes[i] - ('0' as u8)) as usize;
        if digit > max_index_and_digit.digit {
            max_index_and_digit.index = i;
            max_index_and_digit.digit = digit;
        }
    }

    max_index_and_digit
}

fn max_bank_joltage(bank: &str, num_batteries: usize) -> usize {
    let mut running_sum = 0;
    let mut start_index = 0;

    for count in 0..num_batteries {
        running_sum *= 10;
        let end_index = bank.len() - (num_batteries - count) + 1;

        let index_and_digit = max_digit(bank, start_index, end_index);
        running_sum += index_and_digit.digit;
        start_index = index_and_digit.index + 1;
    }

    running_sum
}

fn main() -> Result<()> {
    let args = FileArgs::parse();
    let mut f = File::open(&args.file)?;
    let mut contents = String::with_capacity(f.metadata()?.len() as usize);
    f.read_to_string(&mut contents)?;
    
    let banks = contents.trim().lines();

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for bank in banks {
        let bank_joltage_part1 = max_bank_joltage(bank, 2);
        part1_answer += bank_joltage_part1;

        let bank_joltage_part2 = max_bank_joltage(bank, 12);
        part2_answer += bank_joltage_part2;
    }

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
    
    Ok(())
}
