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
    println!("Contents:\n{contents}");
}


#[cfg(test)]
mod tests {
    // use rstest::rstest;

    // use crate::{part_one, process_line};

    // #[rstest]
    // #[case("1abc2", 12)]
    // fn test_lines(
    //     #[case] line: &str,
    //     #[case] expected: u32
    // ) {
    // }
    //
    // #[test]
    // fn test_sample_input() {
    // }
}
