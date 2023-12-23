use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

struct Game {
    id: u32,
    sets: Vec<Set>
}

struct Set {
    red: u32,
    green: u32,
    blue: u32
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
            let game = Game{id: game_id.parse::<u32>().unwrap(), sets: parse_sets(game_md.1)};
            if is_game_possible(&game) {
                game.id.to_owned()
            } else {
                0
            }
        })
        .sum();
    total
}

fn parse_sets(game_txt: &str) -> Vec<Set> {
    let mut sets: Vec<Set> = Vec::new();
    let hands = game_txt.split(";");
    for hand in hands{
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
            sets.push(Set{red, green, blue});
        }
    }
    sets
}

fn is_game_possible(game: &Game) -> bool {
    for set in &game.sets {
        if set.green > MAX_GREEN || set.red > MAX_RED || set.blue > MAX_BLUE {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{Game, is_game_possible, parse_sets, part_one};

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
    fn test_lines(#[case] game_txt: &str, #[case] expected: bool) {
        let game = Game{id: 0, sets:parse_sets(game_txt)};
        assert_eq!(expected, is_game_possible(&game))
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
