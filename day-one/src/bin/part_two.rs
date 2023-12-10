use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.filepath).expect("File was not Read Successfully");
    let result = part_two(contents.as_str());
    println!("Result: {result}");
}

fn part_two(input: &str)-> u32{
    println!("Input:\n{input}");
    let output = input
        .lines()
        .map(|line|{
            process_line(line)
        })
        .sum::<u32>();
    output
}

fn process_line(line: &str)->u32{
    let mut it = (0..line.len()).filter_map(|index|{
        convert_text_to_number(line, index)
    });

    let first = it.next().expect("Should be a Number");
    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a Number")
}

fn convert_text_to_number(line: &str, index: usize) -> Option<u32> {
    let reduced_line = &line[index..];
    let result = if reduced_line.starts_with("one") {
        '1'
    }else if reduced_line.starts_with("two") {
        '2'
    }else if reduced_line.starts_with("three") {
        '3'
    }else if reduced_line.starts_with("four") {
        '4'
    }else if reduced_line.starts_with("five") {
        '5'
    }else if reduced_line.starts_with("six") {
        '6'
    }else if reduced_line.starts_with("seven") {
        '7'
    }else if reduced_line.starts_with("eight") {
        '8'
    }else if reduced_line.starts_with("nine") {
        '9'
    }else{
        reduced_line.chars().next().unwrap()
    };
    result.to_digit(10)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{process_line, part_two};

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("onetwone", 11)]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32
    ){
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn full_sample(){
        assert_eq!(281, part_two(include_str!("./inputs/part_two_example.txt")));
    }
}
