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
    let total: u32 = contents.lines()
        .map(|game|{
            let game_md = game.split_once(":").unwrap();
            let game_id = game_md.0.replace("Game ", "");
            println!("Game ID: {game_id}");
            let game_txt = game_md.1;
            if process_game(game_txt){
                game_id.parse::<u32>().unwrap()
            }else{
                0 
            }
        })
        .sum();
    println!("Total: {total}")
}

fn process_game(game: &str) -> bool{
    println!("Game: {game}");
    return true
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
