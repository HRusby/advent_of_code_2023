use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.filepath).expect("File was not Read Successfully");
    let result = part_one(contents.as_str());
    println!("Result: {result}");
}

fn part_one(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            process_line(line)
        })
        .sum::<u32>();
    output
}

fn process_line(line: &str) -> u32 {
    let mut it = line.chars().filter_map(|character| character.to_digit(10));
    let first = it.next().expect("Should be a Number");
    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a Number")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{part_one, process_line};

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_lines(
        #[case] line: &str,
        #[case] expected: u32
    ) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(
            142,
            part_one(include_str!("./inputs/part_one_example.txt"))
        );
    }
}
