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
    let total = part_two(contents.as_str());
    println!("Total: {total}")
}

fn part_two(contents: &str) -> u32 {
    let total: u32 = contents
        .lines()
        .map(|game| {
            let game_md = game.split_once(":").unwrap();
            let game_id = game_md.0.replace("Game ", "");
            let game_sets = parse_sets(game_md.1);
            let game = Game{id:game_id.parse::<u32>().unwrap(), sets:game_sets};
            get_game_power(game)
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

fn get_game_power(game: Game) -> u32{
    /*
    * For each Game, get maximum of each colour across all hands, multiple together and return
    */
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{part_two, get_game_power, parse_sets, Game};

    #[rstest]
    #[case("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_lines(#[case] game_txt: &str, #[case] expected: u32) {
        let game = Game{id:0, sets:parse_sets(game_txt)};
        assert_eq!(expected, get_game_power(game))
    }

    #[test]
    fn test_sample_input() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, part_two(contents))
    }
}
