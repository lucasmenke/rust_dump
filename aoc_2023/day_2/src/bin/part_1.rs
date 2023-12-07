use regex::Regex;
use std::io::Error;

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

fn main() {
    let input = include_str!("./input_1.txt");
    match process(input) {
        Ok(result) => println!("{}", result),
        Err(error) => println!("{}", error),
    }
}

fn process(input: &str) -> Result<u32, Error> {
    let result = input
        .lines()
        .map(|line| {
            // check if game passes

            // get game id
            let mut game_id = extract_game_number(&line);
        })
        .sum::<u32>();

    Ok(result)
}

fn game_pass(input: &str) -> bool {
    true
}

fn extract_game_number(input: &str) -> Option<u32> {
    let re = Regex::new(r"Game (\d+):").unwrap();
    if let Some(captures) = re.captures(input) {
        if let Some(number) = captures.get(1) {
            return Some(number.as_str().parse().unwrap());
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_code() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(input);
        assert_eq!("8", result.to_string());
    }
}
