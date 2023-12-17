use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.filepath).expect("File was not Read Successfully");
    println!("Contents:\n{contents}");
    let total = part_one(contents.as_str());
    println!("Total: {total}")
}

fn part_one(contents: &str) -> u32 {
    let total: u32 = contents
        .lines()
        .map(|game| {
            let game_md = game.split_once(":").unwrap();
            let game_id = game_md.0.replace("Game ", "");
            println!("Game ID: {game_id}");
            let game_txt = game_md.1;
            if is_game_possible(game_txt) {
                game_id.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum();
    total
}

fn is_game_possible(game: &str) -> bool {
    println!("Game: {game}");
    let hands = game.split(";");
    for hand in hands {
        println!("hand: {hand}");
        let draws = hand.trim().split(",");
        for draw in draws {
            let tuple = draw.trim().split_once(" ").unwrap();
            let count = tuple.0.parse::<u32>().unwrap();
            let colour = tuple.1;
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            if colour == "red" {
                red += count;
            } else if colour == "green" {
                green += count;
            } else if colour == "blue" {
                blue += count;
            } else {
                unimplemented!();
            }

            if green > MAX_GREEN || red > MAX_RED || blue > MAX_BLUE {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{is_game_possible, part_one};

    #[rstest]
    #[case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", true)]
    #[case(
        "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_lines(#[case] line: &str, #[case] expected: bool) {
        assert_eq!(expected, is_game_possible(line))
    }

    #[test]
    fn test_sample_input() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, part_one(contents))
    }
}
